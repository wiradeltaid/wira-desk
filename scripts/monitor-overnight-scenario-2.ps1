<#
.SYNOPSIS
    Overnight 8-hour memory & resource sampler for Wira Desk daemon (Scenario 2).

.DESCRIPTION
    Runs in background for up to 8 hours (default 480 minutes).
    1. Keeps system awake via SetThreadExecutionState (ES_CONTINUOUS | ES_SYSTEM_REQUIRED | ES_AWAYMODE_REQUIRED).
    2. Takes process memory & Win32 GUI resource snapshots every 15 minutes.
    3. Records timestamp, elapsed minutes, Private Bytes, Working Set, Handles, GDI, and USER objects.
    4. Appends to .scratch/scenario-2-overnight.csv and updates .scratch/scenario-2-overnight.json.
    5. Cleanly restores system power state upon completion or cancellation.

.PARAMETER DurationHours
    Total run duration in hours. Default: 8.

.PARAMETER IntervalMinutes
    Sampling interval in minutes. Default: 15.

.EXAMPLE
    .\monitor-overnight-scenario-2.ps1 -DurationHours 8 -IntervalMinutes 15
#>
[CmdletBinding()]
param(
    [double]$DurationHours = 8.0,
    [int]$IntervalMinutes = 15
)

$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
Set-Location $repoRoot

$csvPath = Join-Path $repoRoot ".scratch\scenario-2-overnight.csv"
$jsonPath = Join-Path $repoRoot ".scratch\scenario-2-overnight.json"
$lockPath = Join-Path $repoRoot ".scratch\scenario-2.pid"

# Ensure .scratch directory exists
$scratchDir = Join-Path $repoRoot ".scratch"
if (-not (Test-Path $scratchDir)) { New-Item -ItemType Directory -Path $scratchDir | Out-Null }

# Record our own PID
$PID | Set-Content -Path $lockPath -Encoding utf8

Add-Type @'
using System;
using System.Runtime.InteropServices;

public static class Win32PowerKeepAwake {
    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern uint SetThreadExecutionState(uint esFlags);

    public const uint ES_CONTINUOUS = 0x80000000;
    public const uint ES_SYSTEM_REQUIRED = 0x00000001;
    public const uint ES_AWAYMODE_REQUIRED = 0x00000040;
}

public static class Win32ProcHelper {
    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern IntPtr OpenProcess(uint processAccess, bool bInheritHandle, int processId);

    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern bool CloseHandle(IntPtr hObject);

    [DllImport("user32.dll")]
    public static extern uint GetGuiResources(IntPtr hProcess, uint uiFlags);
}
'@

# Prevent Windows from entering sleep during the test
[Win32PowerKeepAwake]::SetThreadExecutionState(
    [Win32PowerKeepAwake]::ES_CONTINUOUS -bor
    [Win32PowerKeepAwake]::ES_SYSTEM_REQUIRED -bor
    [Win32PowerKeepAwake]::ES_AWAYMODE_REQUIRED
) | Out-Null

# Initialize CSV header if not exists
if (-not (Test-Path $csvPath)) {
    "Timestamp,ElapsedMinutes,PID,PrivateMemoryMB,PrivateMemoryBytes,WorkingSetMB,WorkingSetBytes,Handles,GdiObjects,UserObjects" | Set-Content -Path $csvPath -Encoding utf8
}

$startTime = Get-Date
$totalMinutes = [int]($DurationHours * 60)
$elapsed = 0

try {
    while ($elapsed -le $totalMinutes) {
        $daemon = Get-Process -Name "wiradesk" -ErrorAction SilentlyContinue | Select-Object -First 1
        if ($daemon) {
            $daemon.Refresh()
            $h = [Win32ProcHelper]::OpenProcess(0x1000, $false, $daemon.Id)
            if ($h -eq [IntPtr]::Zero) {
                $h = [Win32ProcHelper]::OpenProcess(0x0400, $false, $daemon.Id)
            }
            $gdi = 0
            $user = 0
            if ($h -ne [IntPtr]::Zero) {
                try {
                    $gdi = [Win32ProcHelper]::GetGuiResources($h, 0)
                    $user = [Win32ProcHelper]::GetGuiResources($h, 1)
                } finally {
                    [Win32ProcHelper]::CloseHandle($h) | Out-Null
                }
            }

            $sampleTime = (Get-Date).ToString("yyyy-MM-ddTHH:mm:ssZ")
            $privMB = [Math]::Round($daemon.PrivateMemorySize64 / 1MB, 2)
            $wsMB = [Math]::Round($daemon.WorkingSet64 / 1MB, 2)

            $csvLine = "$sampleTime,$elapsed,$($daemon.Id),$privMB,$($daemon.PrivateMemorySize64),$wsMB,$($daemon.WorkingSet64),$($daemon.HandleCount),$gdi,$user"
            Add-Content -Path $csvPath -Value $csvLine -Encoding utf8

            # Update JSON representation
            $statusObj = [ordered]@{
                Status = if ($elapsed -ge $totalMinutes) { "Completed" } else { "Running" }
                StartedAt = $startTime.ToString("yyyy-MM-ddTHH:mm:ssZ")
                LastSampleAt = $sampleTime
                ElapsedMinutes = $elapsed
                TotalPlannedMinutes = $totalMinutes
                LatestSample = [ordered]@{
                    PID = $daemon.Id
                    PrivateMemoryMB = $privMB
                    PrivateMemoryBytes = $daemon.PrivateMemorySize64
                    WorkingSetMB = $wsMB
                    WorkingSetBytes = $daemon.WorkingSet64
                    Handles = $daemon.HandleCount
                    GdiObjects = $gdi
                    UserObjects = $user
                    WithinBudget5MB = ($daemon.PrivateMemorySize64 -le 5242880)
                }
            }
            $statusObj | ConvertTo-Json -Depth 5 | Set-Content -Path $jsonPath -Encoding utf8
        }

        # Sleep until next interval or finish
        if ($elapsed -lt $totalMinutes) {
            Start-Sleep -Seconds ($IntervalMinutes * 60)
            $elapsed = [int]((Get-Date) - $startTime).TotalMinutes
        } else {
            break
        }
    }
} finally {
    # Restore normal system execution state
    [Win32PowerKeepAwake]::SetThreadExecutionState([Win32PowerKeepAwake]::ES_CONTINUOUS) | Out-Null
    if (Test-Path $lockPath) { Remove-Item $lockPath -Force -ErrorAction SilentlyContinue }
}
