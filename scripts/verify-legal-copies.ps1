# ==============================================================================
# scripts/verify-legal-copies.ps1
#
# Verifies repository legal copies and copy stamps:
# 1. Asserts existence of PRIVACY.id.md, PRIVACY.md, SECURITY.id.md, and SECURITY.md
# 2. Confirms standardized copy stamp comment block below top heading
# 3. Confirms zero unpopulated placeholders (<UPDATE-ENDPOINT, <TANGGAL, <GO-LIVE)
# 4. Confirms zero local machine paths
# ==============================================================================

[CmdletBinding()]
param(
    [string]$Path = '.'
)

$ErrorActionPreference = 'Stop'
$repoRoot = (Resolve-Path -LiteralPath $Path).ProviderPath

$filesToCheck = @(
    @{
        File = 'PRIVACY.id.md'
        Stamp = '<!-- Copied from the Wira Delta Indonesia legal source (wira-desk/privacy.id.md) on'
        ExpectedSource = 'wira-desk/privacy.id.md'
    },
    @{
        File = 'PRIVACY.md'
        Stamp = '<!-- Copied from the Wira Delta Indonesia legal source (wira-desk/privacy.en.md) on'
        ExpectedSource = 'wira-desk/privacy.en.md'
    },
    @{
        File = 'SECURITY.id.md'
        Stamp = '<!-- Copied from the Wira Delta Indonesia legal source (wira-desk/security.id.md) on'
        ExpectedSource = 'wira-desk/security.id.md'
    },
    @{
        File = 'SECURITY.md'
        Stamp = '<!-- Copied from the Wira Delta Indonesia legal source (wira-desk/security.en.md) on'
        ExpectedSource = 'wira-desk/security.en.md'
    }
)

$failures = [System.Collections.Generic.List[string]]::new()

foreach ($entry in $filesToCheck) {
    $filePath = Join-Path $repoRoot $entry.File
    if (-not (Test-Path $filePath)) {
        $failures.Add("Required legal file '$($entry.File)' not found in $repoRoot.")
        Write-Host "[legal] FAIL: Missing file $($entry.File)" -ForegroundColor Red
        continue
    }

    $content = Get-Content -LiteralPath $filePath -Raw

    # 1. Assert file begins with H1 heading
    if ($content -notmatch '^\s*#\s+[^\r\n]+') {
        $failures.Add("File '$($entry.File)' does not start with a top-level H1 heading.")
        Write-Host "[legal] FAIL: Missing H1 heading at top of $($entry.File)" -ForegroundColor Red
    }

    # 2. Assert full two-line copy stamp comment block immediately below top heading separated by one blank line
    $expectedSourceEscaped = [regex]::Escape($entry.ExpectedSource)
    $fullStampRegex = "(?s)^\s*#\s+[^\r\n]+[ \t]*\r?\n\r?\n<!-- Copied from the Wira Delta Indonesia legal source \($expectedSourceEscaped\) on \d{4}-\d{2}-\d{2}\.\r?\n\s+Edit the source, then copy it here again\. -->"
    if ($content -notmatch $fullStampRegex) {
        $failures.Add("File '$($entry.File)' does not carry the full two-line copy stamp immediately below H1 for source '$($entry.ExpectedSource)'.")
        Write-Host "[legal] FAIL: Invalid copy stamp structure or placement in $($entry.File)" -ForegroundColor Red
    }

    # Check for unpopulated placeholders
    if ($content -match '<(UPDATE-ENDPOINT|TANGGAL|GO-LIVE)[^>]*>') {
        $failures.Add("File '$($entry.File)' contains unpopulated placeholder: '$($Matches[0])'.")
        Write-Host "[legal] FAIL: Unpopulated placeholder in $($entry.File): $($Matches[0])" -ForegroundColor Red
    }

    # Check for local machine paths
    if ($content -match '[A-Za-z]:\\[Users|Developer|home|tmp]') {
        $failures.Add("File '$($entry.File)' contains local machine path: '$($Matches[0])'.")
        Write-Host "[legal] FAIL: Local machine path in $($entry.File): $($Matches[0])" -ForegroundColor Red
    }
}

if ($failures.Count -eq 0) {
    Write-Host "[legal] PASS: All 4 legal files verified with valid stamps and zero placeholders." -ForegroundColor Green
    exit 0
} else {
    Write-Host "[legal] FAILED with $($failures.Count) error(s)." -ForegroundColor Red
    exit 1
}
