<#
.SYNOPSIS
    Verifies canonical public facts and numbers across product source and public surfaces.

.DESCRIPTION
    Verification harness for canonical public facts and numbers across product source and public surfaces.
    Reads docs\public-facts.yaml and enforces three rules:
      1. Re-derive: For every derived fact, inspects source code via pattern and fails on drift.
      2. Require provenance: For every measured fact with a set value, enforces that metric,
         measured_on, and measured_build are populated.
      3. Check surfaces: For every fact with declared appears_in paths, verifies the rendered value
         is present and fails if any known stale patterns are detected.

.EXAMPLE
    .\scripts\verify-public-facts.ps1
#>
[CmdletBinding()]
param(
    [string]$Path = 'docs\public-facts.yaml'
)

$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
Set-Location $repoRoot

function Write-Pass($m) { Write-Host "[facts] PASS  $m" -ForegroundColor Green }
function Write-Fail($m) { Write-Host "[facts] FAIL  $m" -ForegroundColor Red }
function Write-Step($m) { Write-Host "[facts] $m" -ForegroundColor Cyan }

$factsFile = if ([System.IO.Path]::IsPathRooted($Path)) { $Path } else { Join-Path $repoRoot $Path }
if (-not (Test-Path $factsFile)) {
    Write-Fail "Public facts file not found at: $factsFile"
    exit 1
}

Write-Step "Loading canonical public facts from $Path ..."

# Lightweight YAML parser for public-facts.yaml schema
$rawLines = Get-Content $factsFile
$facts = [System.Collections.Generic.List[PSCustomObject]]::new()
$current = $null
$activeList = $null

for ($i = 0; $i -lt $rawLines.Count; $i++) {
    $line = $rawLines[$i]
    if ($line -match '^\s*#') { continue }
    if ($line.Trim() -eq '') { continue }

    if ($line -match '^\s*-\s+id:\s*([A-Za-z0-9_]+)\s*$') {
        if ($current) { $facts.Add([PSCustomObject]$current) }
        $current = [ordered]@{
            id = $matches[1]
            value = $null
            metric = $null
            measured_on = $null
            measured_build = $null
            source = $null
            derived_file = $null
            derived_pattern = $null
            appears_in = [System.Collections.Generic.List[string]]::new()
            stale_patterns = [System.Collections.Generic.List[string]]::new()
        }
        $activeList = $null
    } elseif ($current) {
        if ($line -match '^\s*appears_in:\s*\[\]\s*$') {
            $activeList = $null
        } elseif ($line -match '^\s*appears_in:\s*$') {
            $activeList = 'appears_in'
        } elseif ($line -match '^\s*stale_patterns:\s*\[\]\s*$') {
            $activeList = $null
        } elseif ($line -match '^\s*stale_patterns:\s*$') {
            $activeList = 'stale_patterns'
        } elseif ($line -match '^\s*value:\s*(.+)$') {
            $v = $matches[1].Trim()
            $v = $v.Trim('"').Trim("'")
            $current['value'] = if ($v -eq 'null' -or $v -eq '~') { $null } else { $v }
            $activeList = $null
        } elseif ($line -match '^\s*metric:\s*(.+)$') {
            $current['metric'] = $matches[1].Trim().Trim('"').Trim("'")
            $activeList = $null
        } elseif ($line -match '^\s*measured_on:\s*(.+)$') {
            $v = $matches[1].Trim().Trim('"').Trim("'")
            $current['measured_on'] = if ($v -eq 'null' -or $v -eq '~') { $null } else { $v }
            $activeList = $null
        } elseif ($line -match '^\s*measured_build:\s*(.+)$') {
            $v = $matches[1].Trim().Trim('"').Trim("'")
            $current['measured_build'] = if ($v -eq 'null' -or $v -eq '~') { $null } else { $v }
            $activeList = $null
        } elseif ($line -match '^\s*source:\s*(.+)$') {
            $current['source'] = $matches[1].Trim().Trim('"').Trim("'")
            $activeList = $null
        } elseif ($line -match '^\s*file:\s*(.+)$') {
            $current['derived_file'] = $matches[1].Trim().Trim('"').Trim("'")
            $activeList = $null
        } elseif ($line -match '^\s*pattern:\s*(.+)$') {
            $current['derived_pattern'] = $matches[1].Trim().Trim('"').Trim("'")
            $activeList = $null
        } elseif ($line -match '^\s*-\s+''?([^''"]+)''?\s*$') {
            $item = $matches[1].Trim()
            if ($activeList -eq 'appears_in') {
                $current['appears_in'].Add($item)
            } elseif ($activeList -eq 'stale_patterns') {
                $current['stale_patterns'].Add($item)
            }
        }
    }
}
if ($current) { $facts.Add([PSCustomObject]$current) }

Write-Step "Loaded $($facts.Count) canonical facts. Executing verifications..."

$failures = [System.Collections.Generic.List[string]]::new()

# -----------------------------------------------------------------------------
# Rule 1: Re-derive code-backed facts
# -----------------------------------------------------------------------------
foreach ($f in $facts) {
    if ($f.derived_file -and $f.derived_pattern) {
        $srcPath = Join-Path $repoRoot $f.derived_file
        if (-not (Test-Path $srcPath)) {
            $failures.Add("Fact '$($f.id)': source file not found at '$($f.derived_file)'")
            Write-Fail "[$($f.id)] source file '$($f.derived_file)' not found"
            continue
        }

        $srcContent = Get-Content $srcPath -Raw
        if ($srcContent -match $f.derived_pattern) {
            $extracted = $matches[1].Trim()
            if ($extracted -eq $f.value) {
                Write-Pass "[$($f.id)] re-derived from $($f.derived_file): '$extracted' == '$($f.value)'"
            } else {
                $msg = "Fact '$($f.id)': code in '$($f.derived_file)' has '$extracted' but public-facts.yaml declared '$($f.value)'"
                $failures.Add($msg)
                Write-Fail $msg
            }
        } else {
            $msg = "Fact '$($f.id)': pattern '$($f.derived_pattern)' found no match in '$($f.derived_file)'"
            $failures.Add($msg)
            Write-Fail $msg
        }
    }
}

# -----------------------------------------------------------------------------
# Rule 2: Require provenance for measured facts
# -----------------------------------------------------------------------------
foreach ($f in $facts) {
    if ($f.source -and $null -ne $f.value -and $f.value -ne 'TBD') {
        # Check if fact specifies a numeric/measured metric
        if ($f.metric) {
            if ([string]::IsNullOrWhiteSpace($f.measured_on) -or [string]::IsNullOrWhiteSpace($f.measured_build)) {
                $msg = "Fact '$($f.id)': value is set to '$($f.value)' but provenance (measured_on: '$($f.measured_on)', measured_build: '$($f.measured_build)') is incomplete"
                $failures.Add($msg)
                Write-Fail $msg
            } else {
                Write-Pass "[$($f.id)] provenance confirmed: measured_on=$($f.measured_on), build=$($f.measured_build), metric=$($f.metric)"
            }
        }
    }
}

# -----------------------------------------------------------------------------
# Rule 3: Check surfaces (appears_in & stale_patterns)
# -----------------------------------------------------------------------------
foreach ($f in $facts) {
    if ($f.appears_in.Count -gt 0) {
        foreach ($glob in $f.appears_in) {
            $targetFiles = Resolve-Path (Join-Path $repoRoot $glob) -ErrorAction SilentlyContinue |
                ForEach-Object { $_.Path }

            if (-not $targetFiles -or $targetFiles.Count -eq 0) {
                $msg = "Fact '$($f.id)': target path glob '$glob' matched no files"
                $failures.Add($msg)
                Write-Fail $msg
                continue
            }

            foreach ($tf in $targetFiles) {
                $relPath = if ($tf.StartsWith($repoRoot, [System.StringComparison]::OrdinalIgnoreCase)) {
                    $tf.Substring($repoRoot.Length).TrimStart('\', '/')
                } else {
                    $tf
                }
                $tfContent = Get-Content $tf -Raw

                # Confirm current value presence
                if ($f.value -and $tfContent -notmatch [regex]::Escape($f.value)) {
                    $msg = "Fact '$($f.id)': declared value '$($f.value)' not found in surface '$relPath'"
                    $failures.Add($msg)
                    Write-Fail $msg
                } else {
                    Write-Pass "[$($f.id)] confirmed on surface '$relPath'"
                }

                # Check against stale patterns
                if ($f.stale_patterns.Count -gt 0) {
                    foreach ($stalePat in $f.stale_patterns) {
                        if ($tfContent -match $stalePat) {
                            $msg = "Fact '$($f.id)': stale pattern '$stalePat' detected in surface '$relPath'"
                            $failures.Add($msg)
                            Write-Fail $msg
                        }
                    }
                }
            }
        }
    }
}

Write-Host "-----------------------------------------------------------------"
if ($failures.Count -eq 0) {
    Write-Pass "All $($facts.Count) canonical public facts verified successfully."
    exit 0
} else {
    Write-Fail "Public facts verification failed with $($failures.Count) error(s)."
    exit 1
}
