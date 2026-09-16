<#
.SYNOPSIS
    Measures and records memory, handle, and GDI/USER resource footprints for Wira Desk.

.DESCRIPTION
    Harness for DEF-11 verification.
    Measures:
      - WorkingSet64
      - PrivateMemorySize64 (Private Bytes)
      - PagedMemorySize64
      - HandleCount
      - GDI and USER objects (via user32!GetGuiResources)

    Also captures test conditions:
      - Git commit SHA and status
      - Build configuration (debug vs release)
      - Windows version and build
      - Displays/monitors configuration (count, resolution, DPI, scaling)
      - Target process command line and PID

    Outputs results to console and writes structured JSON to .scratch/memory-measurement.json.

.PARAMETER Target
    The executable to measure ('daemon' or 'settings'). Default: 'daemon'.

.PARAMETER Profile
    Build profile ('release' or 'debug'). Default: 'release'.

.PARAMETER OutFile
    Path to write structured JSON results. Default: '.scratch/memory-measurement.json'.

.EXAMPLE
    .\measure-memory.ps1 -Target daemon -Profile release
#>
[CmdletBinding()]
param(
    [ValidateSet('daemon', 'settings', 'both')]
    [string]$Target = 'daemon',

    [ValidateSet('release', 'debug')]
    [string]$Profile = 'release',

    [string]$OutFile = '.scratch/memory-measurement.json'
)

$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
Set-Location $repoRoot

Add-Type @'
using System;
using System.Runtime.InteropServices;
using System.Collections.Generic;

public class Win32MonitorEntry {
    public int Width;
    public int Height;
    public uint DpiX;
    public uint DpiY;
    public double ScalePercent;
}

public static class Win32SysHelper {
    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern IntPtr OpenProcess(uint processAccess, bool bInheritHandle, int processId);

    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern bool CloseHandle(IntPtr hObject);

    [DllImport("user32.dll")]
    public static extern uint GetGuiResources(IntPtr hProcess, uint uiFlags);

    [DllImport("user32.dll")]
    private static extern bool EnumDisplayMonitors(IntPtr hdc, IntPtr lprcClip, MonitorEnumDelegate lpfnEnum, IntPtr dwData);

    private delegate bool MonitorEnumDelegate(IntPtr hMonitor, IntPtr hdcMonitor, ref RECT lprcMonitor, IntPtr dwData);

    [StructLayout(LayoutKind.Sequential)]
    private struct RECT {
        public int left, top, right, bottom;
    }

    [DllImport("shcore.dll")]
    private static extern int GetDpiForMonitor(IntPtr hmonitor, int dpiType, out uint dpiX, out uint dpiY);

    public static List<Win32MonitorEntry> GetMonitors() {
        var list = new List<Win32MonitorEntry>();
        EnumDisplayMonitors(IntPtr.Zero, IntPtr.Zero, (IntPtr hMon, IntPtr hdc, ref RECT rc, IntPtr data) => {
            var m = new Win32MonitorEntry();
            m.Width = rc.right - rc.left;
            m.Height = rc.bottom - rc.top;
            uint dx = 96, dy = 96;
            try {
                GetDpiForMonitor(hMon, 0, out dx, out dy);
            } catch {}
            m.DpiX = dx;
            m.DpiY = dy;
            m.ScalePercent = Math.Round((double)dx / 96.0 * 100.0, 1);
            list.Add(m);
            return true;
        }, IntPtr.Zero);
        return list;
    }
}
'@

function Get-ProcessResourceSnapshot([System.Diagnostics.Process]$p) {
    $p.Refresh()
    $h = [Win32SysHelper]::OpenProcess(0x1000, $false, $p.Id) # PROCESS_QUERY_LIMITED_INFORMATION
    if ($h -eq [IntPtr]::Zero) {
        $h = [Win32SysHelper]::OpenProcess(0x0400, $false, $p.Id) # PROCESS_QUERY_INFORMATION
    }

    $gdi = 0
    $user = 0
    if ($h -ne [IntPtr]::Zero) {
        try {
            $gdi = [Win32SysHelper]::GetGuiResources($h, 0)
            $user = [Win32SysHelper]::GetGuiResources($h, 1)
        } finally {
            [Win32SysHelper]::CloseHandle($h) | Out-Null
        }
    }

    return [PSCustomObject]@{
        PID = $p.Id
        ProcessName = $p.ProcessName
        WorkingSet64 = $p.WorkingSet64
        WorkingSetMB = [Math]::Round($p.WorkingSet64 / 1MB, 2)
        PrivateMemorySize64 = $p.PrivateMemorySize64
        PrivateMemoryMB = [Math]::Round($p.PrivateMemorySize64 / 1MB, 2)
        PagedMemorySize64 = $p.PagedMemorySize64
        PagedMemoryMB = [Math]::Round($p.PagedMemorySize64 / 1MB, 2)
        HandleCount = $p.HandleCount
        GdiObjects = $gdi
        UserObjects = $user
    }
}

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " Wira Desk Memory & Resource Measurement (DEF-11)" -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

# 1. Environment Conditions
$gitSha = (git rev-parse HEAD).Trim()
$gitStatus = if ((git status --porcelain).Trim()) { "dirty" } else { "clean" }
$os = [System.Environment]::OSVersion
$monitors = [Win32SysHelper]::GetMonitors()

$conditions = [PSCustomObject]@{
    Timestamp = (Get-Date).ToString("yyyy-MM-ddTHH:mm:ssZ")
    GitCommit = $gitSha
    GitWorkingTree = $gitStatus
    Profile = $Profile
    OS = [System.Environment]::OSVersion.VersionString
    WindowsBuild = (Get-ItemProperty "HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion").CurrentBuildNumber
    MonitorCount = $monitors.Count
    Monitors = $monitors
}

Write-Host "Commit:      $gitSha ($gitStatus)"
Write-Host "OS:          $($conditions.OS) (Build $($conditions.WindowsBuild))"
Write-Host "Monitors:    $($monitors.Count) monitor(s) detected"
foreach ($m in $monitors) {
    Write-Host "             - $($m.Width)x$($m.Height) @ $($m.ScalePercent)% DPI ($($m.DpiX)x$($m.DpiY))"
}
Write-Host ""

$results = [ordered]@{}
$results["Conditions"] = $conditions
$measurements = [ordered]@{}

# 2. Daemon Measurement
if ($Target -eq 'daemon' -or $Target -eq 'both') {
    Write-Host "[-] Locating running wiradesk daemon..." -ForegroundColor Yellow
    $daemonProc = Get-Process -Name "wiradesk" -ErrorAction SilentlyContinue | Select-Object -First 1
    if (-not $daemonProc) {
        Write-Warning "wiradesk process not found. Attempting to locate executable..."
        $daemonExe = Join-Path $repoRoot "target\$Profile\wiradesk.exe"
        if (-not (Test-Path $daemonExe)) {
            throw "Executable not found at $daemonExe. Run cargo build --$Profile first."
        }
        Write-Host "    Note: Daemon requires Administrator. Please ensure it is running."
    } else {
        $snap = Get-ProcessResourceSnapshot $daemonProc
        $measurements["Daemon"] = $snap

        Write-Host "[+] Daemon Resource Footprint (PID $($snap.PID)):" -ForegroundColor Green
        Write-Host "    Private Bytes:       $($snap.PrivateMemoryMB) MB  ($($snap.PrivateMemorySize64) bytes)"
        Write-Host "    Working Set:         $($snap.WorkingSetMB) MB  ($($snap.WorkingSet64) bytes)"
        Write-Host "    Paged Memory:        $($snap.PagedMemoryMB) MB  ($($snap.PagedMemorySize64) bytes)"
        Write-Host "    Handle Count:        $($snap.HandleCount)"
        Write-Host "    GDI Objects:         $($snap.GdiObjects)"
        Write-Host "    USER Objects:        $($snap.UserObjects)"

        $budgetBytes = 5 * 1024 * 1024
        if ($snap.PrivateMemorySize64 -le $budgetBytes) {
            Write-Host "    Budget Check:        PASS (Private Bytes <= 5.0 MB budget per DEC-027)" -ForegroundColor Green
        } else {
            Write-Host "    Budget Check:        EXCEEDED (Private Bytes > 5.0 MB budget per DEC-027)" -ForegroundColor Red
        }
    }
}

# 3. Settings Measurement
if ($Target -eq 'settings' -or $Target -eq 'both') {
    Write-Host ""
    Write-Host "[-] Measuring Settings UI..." -ForegroundColor Yellow
    $settingsExe = Join-Path $repoRoot "target\$Profile\wiradesk-settings.exe"
    if (-not (Test-Path $settingsExe)) {
        throw "Settings executable not found at $settingsExe. Run cargo build --$Profile first."
    }

    $existing = Get-Process -Name "wiradesk-settings" -ErrorAction SilentlyContinue
    $p = $null
    $launchedByUs = $false
    if ($existing) {
        $p = $existing | Select-Object -First 1
    } else {
        $env:WIRADESK_SETTINGS_ALLOW_NO_DAEMON = '1'
        $p = Start-Process -FilePath $settingsExe -PassThru
        $launchedByUs = $true
        Start-Sleep -Seconds 2
    }

    try {
        $snap = Get-ProcessResourceSnapshot $p
        $measurements["Settings"] = $snap

        Write-Host "[+] Settings Resource Footprint (PID $($snap.PID)):" -ForegroundColor Green
        Write-Host "    Private Bytes:       $($snap.PrivateMemoryMB) MB  ($($snap.PrivateMemorySize64) bytes)"
        Write-Host "    Working Set:         $($snap.WorkingSetMB) MB  ($($snap.WorkingSet64) bytes)"
        Write-Host "    Paged Memory:        $($snap.PagedMemoryMB) MB  ($($snap.PagedMemorySize64) bytes)"
        Write-Host "    Handle Count:        $($snap.HandleCount)"
        Write-Host "    GDI Objects:         $($snap.GdiObjects)"
        Write-Host "    USER Objects:        $($snap.UserObjects)"
    } finally {
        if ($launchedByUs -and $p -and -not $p.HasExited) {
            Stop-Process -Id $p.Id -Force -ErrorAction SilentlyContinue
        }
    }
}

$results["Measurements"] = $measurements

# 4. Save to OutFile
$outDir = Split-Path -Parent (Join-Path $repoRoot $OutFile)
if (-not (Test-Path $outDir)) { New-Item -ItemType Directory -Path $outDir | Out-Null }
$jsonPath = Join-Path $repoRoot $OutFile
$results | ConvertTo-Json -Depth 6 | Set-Content -Path $jsonPath -Encoding utf8
Write-Host ""
Write-Host "[+] Results written to: $jsonPath" -ForegroundColor Cyan
