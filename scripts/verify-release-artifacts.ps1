# ==============================================================================
# scripts/verify-release-artifacts.ps1
#
# Asserts release artifact invariants:
# 1. No loose executables in DistDir (only setup installer and portable zip)
# 2. Portable zip contains exactly wiradesk.exe, wiradesk-settings.exe, LICENSE.txt, NOTICE.txt
# 3. SHA256SUMS contains valid hashes for setup installer and portable zip
# 4. If latest.json is present (or -RequireLatestJson is specified), it validates
# ==============================================================================

[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$DistDir,

    [Parameter(Mandatory = $false)]
    [string]$Version,

    [Parameter(Mandatory = $false)]
    [switch]$RequireLatestJson
)

$ErrorActionPreference = 'Stop'

if (-not (Test-Path $DistDir)) {
    throw "DistDir '$DistDir' does not exist."
}

# 1. Resolve version
if (-not $Version) {
    $setupFile = Get-ChildItem -Path $DistDir -Filter "WiraDesk-*-x64-setup.exe" | Select-Object -First 1
    if (-not $setupFile) {
        throw "Could not locate setup installer in $DistDir matching 'WiraDesk-*-x64-setup.exe'."
    }
    if ($setupFile.Name -match '^WiraDesk-(.+)-x64-setup\.exe$') {
        $Version = $Matches[1]
    } else {
        throw "Could not extract version from installer name '$($setupFile.Name)'."
    }
}

$expectedSetupName = "WiraDesk-$Version-x64-setup.exe"
$expectedZipName = "WiraDesk-$Version-x64-portable.zip"
$setupPath = Join-Path $DistDir $expectedSetupName
$zipPath = Join-Path $DistDir $expectedZipName
$sumsPath = Join-Path $DistDir "SHA256SUMS"
$latestPath = Join-Path $DistDir "latest.json"

# 2. Strict artifact allowlist in DistDir (reject loose binaries and any extraneous artifacts)
$allowedFiles = @($expectedSetupName, $expectedZipName, "SHA256SUMS", "latest.json")
$allFiles = Get-ChildItem -Path $DistDir -File
$extraneous = $allFiles | Where-Object { $_.Name -notin $allowedFiles }
if ($extraneous) {
    $names = ($extraneous | ForEach-Object { $_.Name }) -join ', '
    throw "Prohibited extraneous artifact(s) found in ${DistDir}: $names. Staging directory must contain only the setup installer, portable zip, SHA256SUMS, and optional latest.json."
}

if (-not (Test-Path $setupPath)) {
    throw "Expected setup installer '$expectedSetupName' not found in $DistDir."
}
if (-not (Test-Path $zipPath)) {
    throw "Expected portable archive '$expectedZipName' not found in $DistDir."
}
if (-not (Test-Path $sumsPath)) {
    throw "Expected SHA256SUMS file not found in $DistDir."
}

# 3. Validate portable zip entries
Add-Type -AssemblyName System.IO.Compression.FileSystem
$zip = [System.IO.Compression.ZipFile]::OpenRead($zipPath)
try {
    $entries = @($zip.Entries | ForEach-Object { $_.FullName })
    $requiredEntries = @("wiradesk.exe", "wiradesk-settings.exe", "LICENSE.txt", "NOTICE.txt")

    foreach ($req in $requiredEntries) {
        if ($entries -notcontains $req) {
            throw "Portable zip '$expectedZipName' is missing required file '$req'. Contents: $($entries -join ', ')"
        }
    }

    if ($entries.Count -ne 4) {
        throw "Portable zip '$expectedZipName' must contain exactly 4 files, but contains $($entries.Count): $($entries -join ', ')"
    }
}
finally {
    $zip.Dispose()
}

# 4. Validate SHA256SUMS
$actualSetupHash = (Get-FileHash -Path $setupPath -Algorithm SHA256).Hash.ToLower()
$actualZipHash = (Get-FileHash -Path $zipPath -Algorithm SHA256).Hash.ToLower()

$sumsLines = Get-Content -Path $sumsPath
$sumsMap = @{}
foreach ($line in $sumsLines) {
    if ($line -match '^\s*([0-9a-fA-F]{64})\s+[\*]?(.+)\s*$') {
        $sumsMap[$Matches[2].Trim()] = $Matches[1].ToLower()
    }
}

if (-not $sumsMap.ContainsKey($expectedSetupName)) {
    throw "SHA256SUMS does not contain an entry for '$expectedSetupName'."
}
if ($sumsMap[$expectedSetupName] -ne $actualSetupHash) {
    throw "SHA256SUMS checksum mismatch for '$expectedSetupName': expected $($sumsMap[$expectedSetupName]), got $actualSetupHash."
}

if (-not $sumsMap.ContainsKey($expectedZipName)) {
    throw "SHA256SUMS does not contain an entry for '$expectedZipName'."
}
if ($sumsMap[$expectedZipName] -ne $actualZipHash) {
    throw "SHA256SUMS checksum mismatch for '$expectedZipName': expected $($sumsMap[$expectedZipName]), got $actualZipHash."
}

foreach ($key in $sumsMap.Keys) {
    if ($key -ne $expectedSetupName -and $key -ne $expectedZipName) {
        throw "SHA256SUMS contains extraneous entry '$key'. SHA256SUMS must only cover setup installer and portable zip."
    }
}

# 5. Validate latest.json if required or present
if ($RequireLatestJson -or (Test-Path $latestPath)) {
    if (-not (Test-Path $latestPath)) {
        throw "RequireLatestJson was specified, but '$latestPath' does not exist."
    }
    $latest = Get-Content -Path $latestPath -Raw | ConvertFrom-Json
    if ($latest.version -ne $Version) {
        throw "latest.json version '$($latest.version)' does not match expected version '$Version'."
    }
    if ($latest.setup_sha256.ToLower() -ne $actualSetupHash) {
        throw "latest.json setup_sha256 '$($latest.setup_sha256)' does not match actual installer hash '$actualSetupHash'."
    }
    if (-not $latest.setup_url.EndsWith($expectedSetupName)) {
        throw "latest.json setup_url '$($latest.setup_url)' does not point to '$expectedSetupName'."
    }
}

Write-Host "[verify-release-artifacts] PASS: $DistDir verified ($Version - installer, portable zip, SHA256SUMS)."
