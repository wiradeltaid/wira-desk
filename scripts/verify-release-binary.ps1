# scripts/verify-release-binary.ps1
#
# Release binary dependency scanner and regression guard.
# Inspects Windows PE import tables of release executables to verify that no
# dynamic CRT libraries (VCRUNTIME*.dll, MSVCP*.dll, UCRTBASE.dll, api-ms-win-crt-*.dll)
# are imported.

[CmdletBinding()]
param(
    [string]$Path = 'target\release'
)

$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
Set-Location $repoRoot

$targetDir = if ([System.IO.Path]::IsPathRooted($Path)) { $Path } else { Join-Path $repoRoot $Path }

if (-not (Test-Path $targetDir)) {
    Write-Error "Target path not found: $targetDir"
    exit 1
}

$requiredBinaries = @("wiradesk.exe", "wiradesk-settings.exe")

function Get-PeImports {
    param([string]$FilePath)

    $bytes = [System.IO.File]::ReadAllBytes($FilePath)
    if ($bytes.Length -lt 0x40 -or $bytes[0] -ne 0x4D -or $bytes[1] -ne 0x5A) {
        throw "Invalid or missing DOS header (not a valid PE binary)"
    }

    $e_lfanew = [BitConverter]::ToUInt32($bytes, 0x3C)
    if ($e_lfanew + 24 -gt $bytes.Length) {
        throw "Invalid PE header offset (out of file bounds)"
    }
    if ($bytes[$e_lfanew] -ne 0x50 -or $bytes[$e_lfanew + 1] -ne 0x45) {
        throw "Invalid PE signature (missing PE\0\0)"
    }

    $fileHeader = $e_lfanew + 4
    $numSections = [BitConverter]::ToUInt16($bytes, $fileHeader + 2)
    $optHeaderSize = [BitConverter]::ToUInt16($bytes, $fileHeader + 16)
    $optHeader = $fileHeader + 20

    if ($optHeader + $optHeaderSize -gt $bytes.Length) {
        throw "Invalid optional header size"
    }

    $magic = [BitConverter]::ToUInt16($bytes, $optHeader)
    $importDirOffset = if ($magic -eq 0x010B) { $optHeader + 104 } elseif ($magic -eq 0x020B) { $optHeader + 120 } else { throw "Unsupported PE magic 0x$($magic.ToString('X4'))" }

    if ($importDirOffset + 8 -gt $optHeader + $optHeaderSize) {
        return @() # No import directory
    }

    $importRva = [BitConverter]::ToUInt32($bytes, $importDirOffset)
    $importSize = [BitConverter]::ToUInt32($bytes, $importDirOffset + 4)
    if ($importRva -eq 0 -or $importSize -eq 0) { return @() }

    $secTable = $optHeader + $optHeaderSize
    $sections = @()
    for ($i = 0; $i -lt $numSections; $i++) {
        $sec = $secTable + $i * 40
        if ($sec + 40 -gt $bytes.Length) {
            throw "Corrupted or out-of-bounds section table"
        }
        $sections += [PSCustomObject]@{
            VirtualAddress   = [BitConverter]::ToUInt32($bytes, $sec + 12)
            VirtualSize      = [BitConverter]::ToUInt32($bytes, $sec + 8)
            SizeOfRawData    = [BitConverter]::ToUInt32($bytes, $sec + 16)
            PointerToRawData = [BitConverter]::ToUInt32($bytes, $sec + 20)
        }
    }

    function RvaToOffset($rva) {
        foreach ($s in $sections) {
            $sz = [Math]::Max($s.VirtualSize, $s.SizeOfRawData)
            if ($rva -ge $s.VirtualAddress -and $rva -lt ($s.VirtualAddress + $sz)) {
                $offset = $rva - $s.VirtualAddress
                if ($offset -lt $s.SizeOfRawData) {
                    $rawOffset = $s.PointerToRawData + $offset
                    if ($rawOffset -lt $bytes.Length) {
                        return $rawOffset
                    }
                }
            }
        }
        return $null
    }

    $importOffset = RvaToOffset $importRva
    if ($null -eq $importOffset) {
        throw "Corrupted or out-of-bounds import table RVA 0x$($importRva.ToString('X8'))"
    }

    $dllNames = @()
    $currDesc = $importOffset
    $foundNullTerminator = $false

    while ($currDesc + 20 -le $bytes.Length) {
        $allZero = $true
        for ($k = 0; $k -lt 20; $k++) {
            if ($bytes[$currDesc + $k] -ne 0) { $allZero = $false; break }
        }
        if ($allZero) {
            $foundNullTerminator = $true
            break
        }

        $nameRva = [BitConverter]::ToUInt32($bytes, $currDesc + 12)
        if ($nameRva -eq 0) {
            throw "Invalid null Name RVA in non-null import descriptor at offset 0x$($currDesc.ToString('X8'))"
        }
        $nameOffset = RvaToOffset $nameRva
        if ($null -eq $nameOffset) {
            throw "Invalid or out-of-bounds Name RVA 0x$($nameRva.ToString('X8')) in import descriptor"
        }

        $end = $nameOffset
        while ($end -lt $bytes.Length -and $bytes[$end] -ne 0) { $end++ }
        if ($end -ge $bytes.Length) {
            throw "Unterminated DLL name string at offset 0x$($nameOffset.ToString('X8'))"
        }
        if ($end -eq $nameOffset) {
            throw "Empty DLL name string at offset 0x$($nameOffset.ToString('X8'))"
        }

        $dllName = [System.Text.Encoding]::ASCII.GetString($bytes, $nameOffset, $end - $nameOffset)
        $dllNames += $dllName
        $currDesc += 20
    }

    if (-not $foundNullTerminator) {
        throw "Import directory missing terminating null descriptor"
    }

    return $dllNames
}

$hasFailures = $false

foreach ($binName in $requiredBinaries) {
    $binPath = Join-Path $targetDir $binName
    if (-not (Test-Path $binPath)) {
        Write-Error "Required binary not found: $binPath"
        $hasFailures = $true
        continue
    }

    Write-Host "[*] Scanning $binName ..."
    try {
        $imports = Get-PeImports $binPath
    } catch {
        Write-Error "Malformed PE binary in $($binName): $_"
        $hasFailures = $true
        continue
    }

    $prohibitedImports = @($imports | Where-Object {
        $name = $_.ToLower()
        $stem = if ($name.EndsWith(".dll")) { $name.Substring(0, $name.Length - 4) } else { $name }
        $stem -like "vcruntime*" -or
        $stem -like "msvcp*" -or
        $stem -like "ucrtbase*" -or
        $stem -like "api-ms-win-crt-*"
    })

    if ($prohibitedImports.Count -gt 0) {
        Write-Error "Prohibited dynamic CRT imports detected in $($binName): $($prohibitedImports -join ', ')"
        $hasFailures = $true
    } else {
        Write-Host "    [+] Clean: 0 dynamic MSVC CRT imports." -ForegroundColor Green
    }
}

if ($hasFailures) {
    exit 1
} else {
    Write-Host "[+] All release binary import checks passed successfully." -ForegroundColor Green
    exit 0
}
