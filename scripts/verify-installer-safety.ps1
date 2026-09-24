<#
.SYNOPSIS
    Verifies installer safety, downgrade prevention, ISPP version guards, and process shutdown rules.

.DESCRIPTION
    Verification harness for SPEC-21 / DEF-23.
    Directly exercises packaging\wiradesk.iss and compiled installer binaries without code duplication:
      1. Compiles packaging\wiradesk.iss with candidate versions to verify ISPP compile-time version guard:
         - Accepts strictly decimal major.minor.patch (e.g. 0.2.0, 0.10.0, 1.0.42).
         - Rejects 4-component versions, empty components, leading 'v', and prerelease/build metadata.
      2. If running as Administrator (default on CI), verifies real compiled installer behavior against
         isolated 64-bit HKLM registry fixtures:
         - Missing uninstall key: succeeds (first install).
         - Older installed version (0.1.0): succeeds (upgrade).
         - Equal installed version (0.2.0): succeeds (reinstall).
         - Newer installed version (0.99.0): rejected before extraction without modal dialog.
         - Numeric SemVer ordering: candidate 0.10.0 vs installed 0.9.9 succeeds; candidate 0.2.9 vs installed 0.2.10 rejected.
         - Blank, whitespace-padded (" 0.2.0 "), and malformed ("0.2.0-invalid") DisplayVersion: rejected fail-closed.
      All registry states are backed up via 64-bit reg export and atomically restored in finally.
      All filesystem test fixtures are cleaned up in finally.
#>
[CmdletBinding()]
param(
    [string]$StageDir,
    [string]$OutDir
)

$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
Set-Location $repoRoot

function Write-Step($msg) { Write-Host "[verify-installer-safety] $msg" -ForegroundColor Cyan }
function Write-Pass($msg) { Write-Host "[verify-installer-safety] PASS: $msg" -ForegroundColor Green }
function Write-Fail($msg) { Write-Host "[verify-installer-safety] FAIL: $msg" -ForegroundColor Red }

# ----------------------------------------------------------------------------
# 1. Locate Inno Setup Compiler (ISCC.exe)
# ----------------------------------------------------------------------------
$isccCmd = Get-Command ISCC.exe -ErrorAction SilentlyContinue
$iscc = if ($isccCmd) { $isccCmd.Source } else { $null }
if (-not $iscc) {
    $fallbackPaths = @()
    if ($env:ProgramFiles) {
        $fallbackPaths += (Join-Path $env:ProgramFiles "Inno Setup 6\ISCC.exe")
    }
    if (${env:ProgramFiles(x86)}) {
        $fallbackPaths += (Join-Path ${env:ProgramFiles(x86)} "Inno Setup 6\ISCC.exe")
    }
    if ($env:LOCALAPPDATA) {
        $fallbackPaths += (Join-Path $env:LOCALAPPDATA "Programs\Inno Setup 6\ISCC.exe")
    }
    if ($env:ISCC_DIR) {
        $fallbackPaths += (Join-Path $env:ISCC_DIR "ISCC.exe")
    }
    $workspaceRoot = Split-Path -Parent (Split-Path -Parent $repoRoot)
    if ($workspaceRoot -and (Test-Path $workspaceRoot)) {
        $siblingInno = Join-Path $workspaceRoot "snapdown\inno\ISCC.exe"
        if (Test-Path $siblingInno) {
            $fallbackPaths += $siblingInno
        }
    }
    foreach ($fb in $fallbackPaths) {
        if (Test-Path $fb) {
            $iscc = $fb
            break
        }
    }
}

if (-not $iscc) {
    throw "ISCC.exe compiler not found in PATH or standard fallback locations."
}
Write-Step "Using Inno Setup compiler: $iscc"

$tempRoot = [System.IO.Path]::GetFullPath((Join-Path $repoRoot "target\installer_safety_test"))
if (-not (Test-Path $tempRoot)) { New-Item -ItemType Directory -Path $tempRoot | Out-Null }
$testArtifactsDir = Join-Path $tempRoot "test_artifacts"
if (-not (Test-Path $testArtifactsDir)) { New-Item -ItemType Directory -Path $testArtifactsDir | Out-Null }

$realIssContent = Get-Content "packaging\wiradesk.iss" -Raw

# ----------------------------------------------------------------------------
# 2. Stage minimal release binaries for compilation
# ----------------------------------------------------------------------------
$resolvedStage = if ($StageDir) { [System.IO.Path]::GetFullPath((Join-Path $repoRoot $StageDir)) } else { Join-Path $tempRoot "stage" }
$resolvedOut = if ($OutDir) { [System.IO.Path]::GetFullPath((Join-Path $repoRoot $OutDir)) } else { Join-Path $tempRoot "dist" }

if (-not (Test-Path $resolvedStage)) { New-Item -ItemType Directory -Path $resolvedStage | Out-Null }
if (-not (Test-Path $resolvedOut)) { New-Item -ItemType Directory -Path $resolvedOut | Out-Null }

Copy-Item "target\release\wiradesk.exe", "target\release\wiradesk-settings.exe" $resolvedStage -Force
Copy-Item "LICENSE", "NOTICE" $resolvedStage -Force
Copy-Item "crates\daemon\wiradesk.ico" $resolvedStage -Force
Copy-Item "assets\installer-logo*.png" $resolvedStage -Force

# ----------------------------------------------------------------------------
# 3. Direct ISPP Compile-Time Version Guard Tests using real packaging\wiradesk.iss
# ----------------------------------------------------------------------------
Write-Step "Testing ISPP compile-time version guard using real packaging\wiradesk.iss..."

$isppTestCases = @(
    @{ Ver = "0.2.0"; ShouldPass = $true; Reason = "standard 3-component" },
    @{ Ver = "0.10.0"; ShouldPass = $true; Reason = "multi-digit minor" },
    @{ Ver = "1.0.42"; ShouldPass = $true; Reason = "multi-digit patch" },
    @{ Ver = "0.2.0.0"; ShouldPass = $false; Reason = "four components" },
    @{ Ver = "v0.2.0"; ShouldPass = $false; Reason = "leading v" },
    @{ Ver = "V0.2.0"; ShouldPass = $false; Reason = "leading V" },
    @{ Ver = "0.2"; ShouldPass = $false; Reason = "missing patch" },
    @{ Ver = "0.2.0-rc.1"; ShouldPass = $false; Reason = "prerelease metadata" },
    @{ Ver = "0.2.0+build.1"; ShouldPass = $false; Reason = "build metadata" },
    @{ Ver = "0..0"; ShouldPass = $false; Reason = "empty component" },
    @{ Ver = ".1.0"; ShouldPass = $false; Reason = "empty major" },
    @{ Ver = "1.0."; ShouldPass = $false; Reason = "empty patch" }
)

$targetVerLine = '#define AppVersion GetStringFileInfo(STAGE_DIR + "\" + DaemonExe, "FileVersion")'

foreach ($tc in $isppTestCases) {
    $customIss = Join-Path $tempRoot "wiradesk_ver_$($tc.Ver.Replace('.', '_')).iss"
    try {
        $replacement = "#define AppVersion `"$($tc.Ver)`""
        $modifiedContent = $realIssContent.Replace($targetVerLine, $replacement)
        [System.IO.File]::WriteAllText($customIss, $modifiedContent)

        $proc = Start-Process -FilePath $iscc -ArgumentList @("/Q", "/DSTAGE_DIR=$resolvedStage", "/DOUT_DIR=$testArtifactsDir", "`"$customIss`"") -NoNewWindow -Wait -PassThru
        $compiled = ($proc.ExitCode -eq 0)

        if ($compiled -ne $tc.ShouldPass) {
            Write-Fail "ISPP guard on production script failed for '$($tc.Ver)': expected pass=$($tc.ShouldPass), got pass=$compiled ($($tc.Reason))"
            exit 1
        }
    } finally {
        if (Test-Path $customIss) { [System.IO.File]::Delete($customIss) }
    }
}
Write-Pass "All 12 candidate version test cases verified directly against production packaging\wiradesk.iss."

# ----------------------------------------------------------------------------
# 4. Check Packaging Safety Directives and Invariants
# ----------------------------------------------------------------------------
Write-Step "Checking packaging\wiradesk.iss for required safety directives and invariants..."
$requiredPatterns = @(
    @{ Pattern = 'UpdateReadyMemo'; Description = 'UpdateReadyMemo function implemented' },
    @{ Pattern = '\{userappdata\}\\WiraDesk'; Description = 'User configuration and log path displayed' },
    @{ Pattern = 'Preserved across updates; clean installs start fresh\.'; Description = 'Concise Ready memo user state summary' },
    @{ Pattern = '\{#TaskName\} \(optional elevated logon task\)'; Description = 'Explicit task identification without service claim' },
    @{ Pattern = 'Setup does not create or enable auto-start'; Description = 'Auto-start non-creation disclaimer' },
    @{ Pattern = 'RegKeyExists\(HKEY_LOCAL_MACHINE_64'; Description = 'Explicit 64-bit HKLM registry check' },
    @{ Pattern = 'RegQueryStringValue\(HKEY_LOCAL_MACHINE_64'; Description = 'Explicit 64-bit HKLM DisplayVersion query' },
    @{ Pattern = 'If you wish to install an older version, please uninstall the current version first\.'; Description = 'Joined downgrade refusal sentence without mid-sentence linebreak' },
    @{ Pattern = 'Setup cannot verify version compatibility\. Please uninstall the current version before continuing\.'; Description = 'Joined invalid version refusal sentence without mid-sentence linebreak' },
    @{ Pattern = 'PROCESS_STATE_ERROR'; Description = 'Fail-closed process probe verification' },
    @{ Pattern = 'StopDaemonAndSettings'; Description = 'Fail-closed process termination implemented' }
)

foreach ($rp in $requiredPatterns) {
    if ($realIssContent -notmatch $rp.Pattern) {
        Write-Fail "packaging\wiradesk.iss is missing required element: $($rp.Description)"
        exit 1
    }
}

# Assert absence of orphaned mid-sentence linebreaks and retired verbose prose
$prohibitedPatterns = @(
    @{ Pattern = 'please uninstall the current'' \+ #13#10 \+\s*''version first\.'; Description = 'Orphaned linebreak before "version first."' },
    @{ Pattern = 'Please uninstall the current'' \+ #13#10 \+\s*''version before continuing\.'; Description = 'Orphaned linebreak before "version before continuing."' },
    @{ Pattern = 'Preserved if present; Setup never bundles or overwrites user state\.'; Description = 'Retired verbose Ready memo user state disclosure' },
    @{ Pattern = 'A missing config\.toml opens first-run onboarding'; Description = 'Retired verbose Ready memo onboarding prose' },
    @{ Pattern = 'The log file is created on first demand'; Description = 'Retired verbose Ready memo log lifecycle prose' }
)

foreach ($pp in $prohibitedPatterns) {
    if ($realIssContent -match $pp.Pattern) {
        Write-Fail "packaging\wiradesk.iss contains prohibited pattern: $($pp.Description)"
        exit 1
    }
}

# Assert [Files] section zero-bundling invariant (never bundles config.toml or wiradesk.log)
$filesMatch = [regex]::Match($realIssContent, '(?ms)\[Files\]\s*(.*?)(?=\r?\n\[[A-Za-z]+\])')
if (-not $filesMatch.Success) {
    Write-Fail "packaging\wiradesk.iss is missing [Files] section."
    exit 1
}
$filesSection = $filesMatch.Groups[1].Value
if ($filesSection -match 'config\.toml' -or $filesSection -match 'wiradesk\.log') {
    Write-Fail "packaging\wiradesk.iss [Files] section bundles user state (config.toml or wiradesk.log found)."
    exit 1
}
Write-Pass "packaging\wiradesk.iss [Files] section zero-bundling invariant confirmed (no config.toml or wiradesk.log)."
Write-Pass "packaging\wiradesk.iss contains all required safety directives and joined dialog formatting."

# ----------------------------------------------------------------------------
# 5. Real Installer Compilation and 64-bit HKLM Registry Decision Table Tests
# ----------------------------------------------------------------------------
function Test-IsAdmin {
    $id = [Security.Principal.WindowsIdentity]::GetCurrent()
    $p = [Security.Principal.WindowsPrincipal]$id
    return $p.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

$isAdmin = Test-IsAdmin
Write-Step "Checking elevation status for HKLM decision table: Administrator=$isAdmin"

if (-not $isAdmin) {
    if ($env:CI) {
        Write-Fail "CI runner is expected to be Administrator, but Test-IsAdmin returned false."
        exit 1
    }
    Write-Host "[verify-installer-safety] NOTICE: Administrator privileges not present in local session. Skipping live HKLM fixture tests." -ForegroundColor Yellow
    exit 0
}

# Locate or compile standard production installer (derived from Cargo.toml)
$workspaceManifest = Join-Path $repoRoot 'Cargo.toml'
$prodVersion = '0.2.0'
if (Test-Path -LiteralPath $workspaceManifest) {
    $inWorkspacePackage = $false
    foreach ($line in Get-Content -LiteralPath $workspaceManifest) {
        if ($line -match '^\s*\[workspace\.package\]') { $inWorkspacePackage = $true; continue }
        if ($inWorkspacePackage -and $line -match '^\s*version\s*=\s*"([^"]+)"') {
            $prodVersion = $Matches[1]
            break
        }
    }
}

$stdSetupPath = Join-Path $resolvedOut "WiraDesk-$prodVersion-x64-setup.exe"
if (-not (Test-Path $stdSetupPath)) {
    Write-Step "Compiling production installer with ISCC..."
    & $iscc "/DSTAGE_DIR=$resolvedStage" "/DOUT_DIR=$resolvedOut" "packaging\wiradesk.iss" *>$null
    if ($LASTEXITCODE -ne 0) {
        Write-Fail "Production installer failed to compile with ISCC (exit code $LASTEXITCODE)."
        exit 1
    }
}

if (-not (Test-Path $stdSetupPath)) {
    Write-Fail "Compiled setup binary not found at $stdSetupPath."
    exit 1
}
$stdSetup = $stdSetupPath
Write-Pass "Production installer verified: $stdSetup"

# Helper function to compile custom-version installer into test-only directory
function Build-CustomVersionInstaller([string]$Ver, [string]$OutFileName) {
    $tempIss = Join-Path $tempRoot "temp_$OutFileName.iss"
    $rep = "#define AppVersion `"$Ver`""
    $mod = $realIssContent.Replace($targetVerLine, $rep)
    [System.IO.File]::WriteAllText($tempIss, $mod)
    try {
        & $iscc /Q "/DSTAGE_DIR=$resolvedStage" "/DOUT_DIR=$testArtifactsDir" "/F$OutFileName" $tempIss *>$null
        if ($LASTEXITCODE -ne 0) { throw "Failed to compile custom installer for $Ver" }
        $expectedPath = Join-Path $testArtifactsDir "$OutFileName.exe"
        if (-not (Test-Path $expectedPath)) {
            $altPath = Join-Path $testArtifactsDir "WiraDesk-$Ver-x64-setup.exe"
            if (Test-Path $altPath) { return $altPath }
            throw "Custom installer binary not found at $expectedPath or $altPath"
        }
        return $expectedPath
    } finally {
        if (Test-Path $tempIss) { [System.IO.File]::Delete($tempIss) }
    }
}

$appIdKey = "Software\Microsoft\Windows\CurrentVersion\Uninstall\{7E4F9C21-6B3D-4A88-9F14-2C5E8D0A1B73}_is1"
$regKeyPath = "HKLM:\$appIdKey"
$backupRegFile = Join-Path $tempRoot "hklm_backup.reg"
$hadPriorKey = $false

# Backup existing key in 64-bit view if present
& reg.exe export "HKLM\$appIdKey" $backupRegFile /y /reg:64 *>$null
if ($LASTEXITCODE -eq 0 -and (Test-Path $backupRegFile)) {
    $hadPriorKey = $true
    Write-Step "Existing 64-bit HKLM uninstall key backed up to $backupRegFile."
}

# Helper to run silent installer with bounded timeout without /SUPPRESSMSGBOXES to verify non-interactive behavior
function Invoke-SilentInstaller([string]$ExePath, [string]$TargetDir, [int]$TimeoutSec = 15) {
    if (Test-Path $TargetDir) {
        Remove-Item -Path $TargetDir -Recurse -Force -ErrorAction SilentlyContinue | Out-Null
    }
    $p = Start-Process -FilePath $ExePath -ArgumentList "/VERYSILENT /NORESTART /DIR=`"$TargetDir`"" -PassThru
    $exited = $p.WaitForExit($TimeoutSec * 1000)
    if (-not $exited) {
        $p.Kill()
        throw "Installer hung (likely displayed a modal dialog in silent mode) and was killed after $TimeoutSec seconds."
    }
    # Clean up test daemon if launched
    Get-Process -Name "wiradesk" -ErrorAction SilentlyContinue | Where-Object { $_.Path -like "$TargetDir*" } | Stop-Process -Force -ErrorAction SilentlyContinue
    return [PSCustomObject]@{
        ExitCode  = $p.ExitCode
        Extracted = (Test-Path (Join-Path $TargetDir "wiradesk.exe"))
    }
}

try {
    # ------------------------------------------------------------------------
    # Case 1: Newer Version Installed -> Downgrade Rejected (exit non-zero, no files extracted)
    # ------------------------------------------------------------------------
    Write-Step "Decision Table: Installed Newer Version (0.99.0)..."
    & reg.exe delete "HKLM\$appIdKey" /f /reg:64 *>$null
    & reg.exe add "HKLM\$appIdKey" /v "DisplayVersion" /t REG_SZ /d "0.99.0" /f /reg:64 *>$null

    $res = Invoke-SilentInstaller $stdSetup (Join-Path $tempRoot "inst_newer")
    if ($res.ExitCode -eq 0 -or $res.Extracted) {
        Write-Fail "Downgrade rejection failed! ExitCode=$($res.ExitCode), Extracted=$($res.Extracted)"
        exit 1
    }
    Write-Pass "Newer version correctly rejected without modal dialog: ExitCode=$($res.ExitCode), files not extracted."

    # ------------------------------------------------------------------------
    # Case 2: Numeric SemVer Ordering (0.2.9 vs 0.2.10)
    # ------------------------------------------------------------------------
    Write-Step "Decision Table: Numeric SemVer Comparison (candidate 0.2.9 vs installed 0.2.10)..."
    $installer029 = Build-CustomVersionInstaller "0.2.9" "WiraDesk-0.2.9-test"
    & reg.exe add "HKLM\$appIdKey" /v "DisplayVersion" /t REG_SZ /d "0.2.10" /f /reg:64 *>$null

    $res = Invoke-SilentInstaller $installer029 (Join-Path $tempRoot "inst_semver_rej")
    if ($res.ExitCode -eq 0 -or $res.Extracted) {
        Write-Fail "Numeric SemVer rejection failed! ExitCode=$($res.ExitCode), Extracted=$($res.Extracted)"
        exit 1
    }
    Write-Pass "Candidate 0.2.9 rejected against installed 0.2.10: ExitCode=$($res.ExitCode)."

    Write-Step "Decision Table: Numeric SemVer Comparison (candidate 0.10.0 vs installed 0.9.9)..."
    $installer0100 = Build-CustomVersionInstaller "0.10.0" "WiraDesk-0.10.0-test"
    & reg.exe add "HKLM\$appIdKey" /v "DisplayVersion" /t REG_SZ /d "0.9.9" /f /reg:64 *>$null

    $res = Invoke-SilentInstaller $installer0100 (Join-Path $tempRoot "inst_semver_ok")
    if ($res.ExitCode -ne 0 -or (-not $res.Extracted)) {
        Write-Fail "Numeric SemVer upgrade failed! ExitCode=$($res.ExitCode), Extracted=$($res.Extracted)"
        exit 1
    }
    Write-Pass "Candidate 0.10.0 successfully upgraded installed 0.9.9: ExitCode=$($res.ExitCode)."

    # ------------------------------------------------------------------------
    # Case 3: Malformed & Blank Version Handling
    # ------------------------------------------------------------------------
    Write-Step "Decision Table: Blank DisplayVersion ('')..."
    & reg.exe add "HKLM\$appIdKey" /v "DisplayVersion" /t REG_SZ /d "" /f /reg:64 *>$null
    $res = Invoke-SilentInstaller $stdSetup (Join-Path $tempRoot "inst_blank")
    if ($res.ExitCode -eq 0 -or $res.Extracted) {
        Write-Fail "Blank version rejection failed! ExitCode=$($res.ExitCode), Extracted=$($res.Extracted)"
        exit 1
    }
    Write-Pass "Blank version rejected fail-closed: ExitCode=$($res.ExitCode)."

    Write-Step "Decision Table: Whitespace DisplayVersion (' 0.2.0 ')..."
    & reg.exe add "HKLM\$appIdKey" /v "DisplayVersion" /t REG_SZ /d " 0.2.0 " /f /reg:64 *>$null
    $res = Invoke-SilentInstaller $stdSetup (Join-Path $tempRoot "inst_space")
    if ($res.ExitCode -eq 0 -or $res.Extracted) {
        Write-Fail "Whitespace version rejection failed! ExitCode=$($res.ExitCode), Extracted=$($res.Extracted)"
        exit 1
    }
    Write-Pass "Whitespace version rejected fail-closed: ExitCode=$($res.ExitCode)."

    Write-Step "Decision Table: Malformed DisplayVersion ('0.2.0-invalid')..."
    & reg.exe add "HKLM\$appIdKey" /v "DisplayVersion" /t REG_SZ /d "0.2.0-invalid" /f /reg:64 *>$null
    $res = Invoke-SilentInstaller $stdSetup (Join-Path $tempRoot "inst_malformed")
    if ($res.ExitCode -eq 0 -or $res.Extracted) {
        Write-Fail "Malformed version rejection failed! ExitCode=$($res.ExitCode), Extracted=$($res.Extracted)"
        exit 1
    }
    Write-Pass "Malformed version rejected fail-closed: ExitCode=$($res.ExitCode)."

    # ------------------------------------------------------------------------
    # Case 4: Missing Key (First Install)
    # ------------------------------------------------------------------------
    Write-Step "Decision Table: No Existing Key (first install)..."
    & reg.exe delete "HKLM\$appIdKey" /f /reg:64 *>$null
    $res = Invoke-SilentInstaller $stdSetup (Join-Path $tempRoot "inst_first")
    if ($res.ExitCode -ne 0 -or (-not $res.Extracted)) {
        Write-Fail "First install failed! ExitCode=$($res.ExitCode), Extracted=$($res.Extracted)"
        exit 1
    }
    Write-Pass "First install succeeded: ExitCode=$($res.ExitCode), files extracted."

    # ------------------------------------------------------------------------
    # Case 5: Equal Version Installed (Reinstall)
    # ------------------------------------------------------------------------
    Write-Step "Decision Table: Equal Installed Version ($prodVersion)..."
    & reg.exe add "HKLM\$appIdKey" /v "DisplayVersion" /t REG_SZ /d "$prodVersion" /f /reg:64 *>$null
    $res = Invoke-SilentInstaller $stdSetup (Join-Path $tempRoot "inst_equal")
    if ($res.ExitCode -ne 0 -or (-not $res.Extracted)) {
        Write-Fail "Equal reinstall failed! ExitCode=$($res.ExitCode), Extracted=$($res.Extracted)"
        exit 1
    }
    Write-Pass "Equal version reinstall succeeded: ExitCode=$($res.ExitCode), files extracted."

    # ------------------------------------------------------------------------
    # Case 6: Older Version Installed (Upgrade)
    # ------------------------------------------------------------------------
    Write-Step "Decision Table: Older Installed Version (0.1.0)..."
    & reg.exe add "HKLM\$appIdKey" /v "DisplayVersion" /t REG_SZ /d "0.1.0" /f /reg:64 *>$null
    $res = Invoke-SilentInstaller $stdSetup (Join-Path $tempRoot "inst_older")
    if ($res.ExitCode -ne 0 -or (-not $res.Extracted)) {
        Write-Fail "Older version upgrade failed! ExitCode=$($res.ExitCode), Extracted=$($res.Extracted)"
        exit 1
    }
    Write-Pass "Older version upgrade succeeded: ExitCode=$($res.ExitCode), files extracted."

    Write-Pass "All 64-bit HKLM registry decision table cases verified on real installer binaries."
} finally {
    # ------------------------------------------------------------------------
    # Complete Registry & Filesystem Cleanup
    # ------------------------------------------------------------------------
    Write-Step "Cleaning up test registry entries and temporary fixtures..."
    & reg.exe delete "HKLM\$appIdKey" /f /reg:64 *>$null
    if ($hadPriorKey -and (Test-Path $backupRegFile)) {
        & reg.exe import $backupRegFile /reg:64 *>$null
        Write-Step "Prior 64-bit HKLM registry state restored from backup."
    }

    # Clean temporary directories and compiled test binaries
    if (Test-Path $tempRoot) {
        Remove-Item -Path $tempRoot -Recurse -Force -ErrorAction SilentlyContinue | Out-Null
    }
}

Write-Pass "verify-installer-safety completed successfully."
