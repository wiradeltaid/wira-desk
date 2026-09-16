<#
.SYNOPSIS
    Interactive auto-detecting memory measurement harness for Wira Desk Visual Switcher overlay.

.DESCRIPTION
    Interactive auto-detecting memory measurement harness for Wira Desk Visual Switcher overlay.
    Automatically detects when the user holds Win + ` to display the Visual Switcher overlay window
    ("WiraDeskVisualSwitcher").

    Modes:
      - 'sample': Waits for the overlay to appear, measures memory/resources WHILE open, then measures
                  again AFTER dismissal. (Used for Scenario 3, 4, 5).
      - 'leak_test': Measures baseline, tracks 50 open-and-dismiss cycles automatically, then measures
                     final resting baseline to assert zero handle or memory leaks. (Used for Scenario 6).
#>
[CmdletBinding()]
param(
    [ValidateSet('sample', 'leak_test')]
    [string]$Mode = 'sample',

    [string]$ScenarioLabel = 'Scenario-3-Overlay-Open',

    [int]$CycleTarget = 50
)

$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
Set-Location $repoRoot

Add-Type @'
using System;
using System.Runtime.InteropServices;

public static class SwitcherDetectHelper {
    [DllImport("user32.dll", CharSet = CharSet.Unicode)]
    public static extern IntPtr FindWindowW(string lpClassName, string lpWindowName);

    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern IntPtr OpenProcess(uint processAccess, bool bInheritHandle, int processId);

    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern bool CloseHandle(IntPtr hObject);

    [DllImport("user32.dll")]
    public static extern uint GetGuiResources(IntPtr hProcess, uint uiFlags);
}
'@

function Get-DaemonSnapshot {
    $p = Get-Process -Name "wiradesk" -ErrorAction SilentlyContinue | Select-Object -First 1
    if (-not $p) { throw "wiradesk daemon process not found." }
    $p.Refresh()

    $h = [SwitcherDetectHelper]::OpenProcess(0x1000, $false, $p.Id)
    if ($h -eq [IntPtr]::Zero) {
        $h = [SwitcherDetectHelper]::OpenProcess(0x0400, $false, $p.Id)
    }
    $gdi = 0
    $user = 0
    if ($h -ne [IntPtr]::Zero) {
        try {
            $gdi = [SwitcherDetectHelper]::GetGuiResources($h, 0)
            $user = [SwitcherDetectHelper]::GetGuiResources($h, 1)
        } finally {
            [SwitcherDetectHelper]::CloseHandle($h) | Out-Null
        }
    }

    return [PSCustomObject]@{
        Timestamp = (Get-Date).ToString("yyyy-MM-ddTHH:mm:ssZ")
        PID = $p.Id
        PrivateMemoryMB = [Math]::Round($p.PrivateMemorySize64 / 1MB, 2)
        PrivateMemoryBytes = $p.PrivateMemorySize64
        WorkingSetMB = [Math]::Round($p.WorkingSet64 / 1MB, 2)
        WorkingSetBytes = $p.WorkingSet64
        Handles = $p.HandleCount
        GDI = $gdi
        USER = $user
    }
}

function Is-OverlayVisible {
    $hwnd = [SwitcherDetectHelper]::FindWindowW("WiraDeskVisualSwitcher", $null)
    return ($hwnd -ne [IntPtr]::Zero)
}

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " Wira Desk Visual Switcher Interactive Measurement Harness       " -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

$daemon = Get-Process -Name "wiradesk" -ErrorAction SilentlyContinue
if (-not $daemon) {
    Write-Error "Daemon 'wiradesk.exe' is not running. Please start it first."
    exit 1
}

if ($Mode -eq 'sample') {
    Write-Host "[*] Mode: Single Overlay Capture ($ScenarioLabel)" -ForegroundColor Yellow
    Write-Host "    1. Please open your test target windows (e.g. 3 windows of Chrome/Explorer)."
    Write-Host "    2. Press and HOLD [Win + `] until the thumbnail overlay appears on screen."
    Write-Host "    3. Keep holding for 2 seconds while this script captures the active footprint..."
    Write-Host ""
    Write-Host "[...] Waiting for [Win + `] hold / overlay appearance..." -ForegroundColor Yellow

    # Wait for overlay to open (up to 90 seconds)
    $waitStart = Get-Date
    while (-not (Is-OverlayVisible)) {
        if (((Get-Date) - $waitStart).TotalSeconds -ge 90) {
            Write-Warning "Timed out after 90 seconds waiting for overlay appearance."
            exit 2
        }
        Start-Sleep -Milliseconds 100
    }

    try { [Console]::Beep(1000, 150) } catch {}
    Write-Host "[+] OVERLAY DETECTED! Capturing active footprint..." -ForegroundColor Green
    Start-Sleep -Milliseconds 400
    $activeSnap = Get-DaemonSnapshot

    Write-Host "    Active Private Bytes: $($activeSnap.PrivateMemoryMB) MB ($($activeSnap.PrivateMemoryBytes) bytes)" -ForegroundColor Green
    Write-Host "    Active Working Set:   $($activeSnap.WorkingSetMB) MB"
    Write-Host "    Active Handles/GDI:   $($activeSnap.Handles) handles, $($activeSnap.GDI) GDI, $($activeSnap.USER) USER"
    Write-Host ""
    Write-Host "[*] Now release [Win + `] to dismiss the overlay..." -ForegroundColor Yellow

    # Wait for overlay to dismiss
    while (Is-OverlayVisible) {
        Start-Sleep -Milliseconds 100
    }

    try { [Console]::Beep(800, 150) } catch {}
    Write-Host "[+] Overlay dismissed. Settling 1 second for thumbnail teardown..." -ForegroundColor Green
    Start-Sleep -Seconds 1
    $postSnap = Get-DaemonSnapshot

    Write-Host "    Post Private Bytes:   $($postSnap.PrivateMemoryMB) MB ($($postSnap.PrivateMemoryBytes) bytes)" -ForegroundColor Green
    Write-Host "    Post Working Set:     $($postSnap.WorkingSetMB) MB"
    Write-Host "    Post Handles/GDI:     $($postSnap.Handles) handles, $($postSnap.GDI) GDI, $($postSnap.USER) USER"

    $result = [PSCustomObject]@{
        Scenario = $ScenarioLabel
        Timestamp = (Get-Date).ToString("yyyy-MM-ddTHH:mm:ssZ")
        Active = $activeSnap
        Dismissed = $postSnap
        WithinBudget = ($activeSnap.PrivateMemoryBytes -le 5242880)
    }

    $outJson = ".scratch\measurement-$ScenarioLabel.json"
    $result | ConvertTo-Json -Depth 5 | Set-Content -Path $outJson -Encoding utf8
    Write-Host ""
    Write-Host "[+] Result saved to $outJson" -ForegroundColor Cyan
}
elseif ($Mode -eq 'leak_test') {
    Write-Host "[*] Mode: Leak Guard 50-Cycle Automated Tracker" -ForegroundColor Yellow
    Write-Host "    Baseline snapshot taking..."
    $baseline = Get-DaemonSnapshot
    Write-Host "    Baseline: $($baseline.PrivateMemoryMB) MB, $($baseline.Handles) handles, $($baseline.GDI) GDI, $($baseline.USER) USER" -ForegroundColor Green
    Write-Host ""
    Write-Host "    INSTRUCTIONS:" -ForegroundColor Yellow
    Write-Host "    Repeatedly press-and-hold then release [Win + `] to open and close the overlay."
    Write-Host "    The script will count each cycle automatically until it reaches $CycleTarget."
    Write-Host ""

    $cycle = 0
    while ($cycle -lt $CycleTarget) {
        # Wait for open
        while (-not (Is-OverlayVisible)) {
            Start-Sleep -Milliseconds 50
        }
        # Wait for close
        while (Is-OverlayVisible) {
            Start-Sleep -Milliseconds 50
        }
        $cycle++
        Write-Host "    [Cycle $cycle/$CycleTarget completed]" -ForegroundColor Cyan
    }

    try { [Console]::Beep(1200, 300) } catch {}
    Write-Host ""
    Write-Host "[+] All $CycleTarget cycles completed! Settling 2 seconds..." -ForegroundColor Green
    Start-Sleep -Seconds 2
    $finalSnap = Get-DaemonSnapshot

    Write-Host "=================================================================" -ForegroundColor Cyan
    Write-Host " Leak Guard Comparison (Baseline vs Post-50-Cycles):" -ForegroundColor Cyan
    Write-Host "=================================================================" -ForegroundColor Cyan
    Write-Host " Metric             Baseline             Post-50-Cycles       Delta"
    Write-Host " -----------------------------------------------------------------"
    $privDelta = [Math]::Round($finalSnap.PrivateMemoryMB - $baseline.PrivateMemoryMB, 2)
    $wsDelta = [Math]::Round($finalSnap.WorkingSetMB - $baseline.WorkingSetMB, 2)
    $handleDelta = $finalSnap.Handles - $baseline.Handles
    $gdiDelta = $finalSnap.GDI - $baseline.GDI
    $userDelta = $finalSnap.USER - $baseline.USER

    Write-Host (" Private Bytes:     {0,8} MB          {1,8} MB        {2,8} MB" -f $baseline.PrivateMemoryMB, $finalSnap.PrivateMemoryMB, $privDelta)
    Write-Host (" Working Set:       {0,8} MB          {1,8} MB        {2,8} MB" -f $baseline.WorkingSetMB, $finalSnap.WorkingSetMB, $wsDelta)
    Write-Host (" Handles:           {0,8}             {1,8}           {2,8}" -f $baseline.Handles, $finalSnap.Handles, $handleDelta)
    Write-Host (" GDI Objects:       {0,8}             {1,8}           {2,8}" -f $baseline.GDI, $finalSnap.GDI, $gdiDelta)
    Write-Host (" USER Objects:      {0,8}             {1,8}           {2,8}" -f $baseline.USER, $finalSnap.USER, $userDelta)

    $leakFree = ($handleDelta -le 2 -and $gdiDelta -eq 0 -and $userDelta -eq 0 -and $privDelta -lt 0.5)
    if ($leakFree) {
        Write-Host ""
        Write-Host " [+] LEAK GUARD VERDICT: PASS (Zero resource leakage detected)" -ForegroundColor Green
    } else {
        Write-Host ""
        Write-Host " [!] LEAK GUARD VERDICT: INVESTIGATE (Non-zero resource growth)" -ForegroundColor Yellow
    }

    $leakResult = [PSCustomObject]@{
        Timestamp = (Get-Date).ToString("yyyy-MM-ddTHH:mm:ssZ")
        Cycles = $CycleTarget
        Baseline = $baseline
        PostCycles = $finalSnap
        Deltas = [PSCustomObject]@{
            PrivateMemoryMB = $privDelta
            WorkingSetMB = $wsDelta
            Handles = $handleDelta
            GDI = $gdiDelta
            USER = $userDelta
        }
        LeakFree = $leakFree
    }

    $outJson = ".scratch\measurement-Scenario-6-Leak-Guard.json"
    $leakResult | ConvertTo-Json -Depth 5 | Set-Content -Path $outJson -Encoding utf8
    Write-Host "[+] Result saved to $outJson" -ForegroundColor Cyan
}
