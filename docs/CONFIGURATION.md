# Configuration Reference — Wira Desk

Wira Desk configuration is stored as a human-readable TOML file located at:

```text
%APPDATA%\WiraDesk\config.toml
```

Settings configured through the companion Settings app are written directly to this file. You can also edit it manually while the daemon is running; Wira Desk automatically reloads on file save.

---

## 1. Complete Default `config.toml`

```toml
[general]
auto_start = false
check_updates = true

[switcher]
shortcut = "win+backtick"
shortcut_enabled = true
fallback_shortcut = "alt+backtick"
fallback_shortcut_enabled = true
visual_enabled = true
visual_hold_delay_ms = 300

[snapping]
snap_half_left = "ctrl+alt+left"
snap_half_left_enabled = true
snap_half_right = "ctrl+alt+right"
snap_half_right_enabled = true
snap_half_top = "ctrl+alt+up"
snap_half_top_enabled = true
snap_half_bottom = "ctrl+alt+down"
snap_half_bottom_enabled = true
snap_maximize = "ctrl+alt+enter"
snap_maximize_enabled = true
snap_third_left = "ctrl+alt+1"
snap_third_left_enabled = true
snap_third_middle = "ctrl+alt+2"
snap_third_middle_enabled = true
snap_third_right = "ctrl+alt+3"
snap_third_right_enabled = true
snap_percent_left = "ctrl+alt+shift+left"
snap_percent_left_enabled = true
percent_left = 67
snap_percent_right = "ctrl+alt+shift+right"
snap_percent_right_enabled = true
percent_right = 67
snap_percent_top = "ctrl+alt+shift+up"
snap_percent_top_enabled = true
percent_top = 33
snap_percent_bottom = "ctrl+alt+shift+down"
snap_percent_bottom_enabled = true
percent_bottom = 67

[layout]
move_next_monitor_shortcut = "ctrl+alt+shift+enter"
move_next_monitor_shortcut_enabled = true
stack_shortcut = "ctrl+alt+shift+s"
stack_shortcut_enabled = true
stack_width_percent = 70

[vm_bypass]
bypass_processes = [
    "mstsc.exe",
    "vmconnect.exe",
    "vmware.exe",
    "VirtualBoxVM.exe",
    "MobaXterm.exe"
]
bypass_classes = [
    "VMwareUnityWindow"
]

[mouse]
enabled = true
thumb_back = "prev_virtual_desktop"
thumb_forward = "next_virtual_desktop"
tilt_left = "show_desktop"
tilt_right = "task_view"
```

---

## 2. Configuration Sections

### `[general]`
- `auto_start` (*boolean*, default: `false`): Registers or unregisters the scheduled task to start the daemon elevated at Windows user logon.
- `check_updates` (*boolean*, default: `true`): Periodically queries GitHub releases over HTTPS for newer versions. No telemetry or tracking identifier is transmitted.

### `[switcher]`
- `shortcut` (*string*, default: `"win+backtick"`): Primary chord for cycling windows of the active app on the current monitor and virtual desktop.
- `shortcut_enabled` (*boolean*, default: `true`): Toggles the primary switcher shortcut.
- `fallback_shortcut` (*string*, default: `"alt+backtick"`): Alternative chord for cycling when the primary shortcut collides with another app.
- `fallback_shortcut_enabled` (*boolean*, default: `true`): Toggles the fallback cycling shortcut.
- `visual_enabled` (*boolean*, default: `true`): Enables the visual switcher overlay with live thumbnails when holding the chord.
- `visual_hold_delay_ms` (*integer*, default: `300`, range: `100..=500`): Duration in milliseconds to hold the chord before opening the visual switcher overlay.

### `[snapping]`
- Configures one-keystroke window snapping to screen halves (50%), thirds (33%), and custom directional edge percentages (`1..=99%`, left/right/bottom default `67%`, top default `33%`).

### `[layout]`
- `move_next_monitor_shortcut`: Moves the active window to the next physical display monitor.
- `stack_shortcut`: Arranges up to 3 application windows into an overlapping stacked layout.
- `stack_width_percent`: Width percentage allocated to stacked windows.

### `[vm_bypass]`
Lists process executable names (`bypass_processes`) and window class names (`bypass_classes`) where keyboard and mouse hooks bypass chord interception to allow remote desktop and virtual machine sessions full control.

### `[mouse]`
- `enabled` (*boolean*, default: `true`): Toggles low-level mouse button hook (`WH_MOUSE_LL`).
- `thumb_back` / `thumb_forward`: Action mapped to extra side thumb buttons (`XBUTTON1` / `XBUTTON2`).
- `tilt_left` / `tilt_right`: Action mapped to horizontal wheel tilting.

#### Available Mouse Preset Slugs (20 Total)
1. **Virtual Desktops:** `prev_virtual_desktop`, `next_virtual_desktop`
2. **Windows Shell:** `show_desktop`, `task_view`
3. **Switching:** `cycle_forward`, `cycle_backward`
4. **Snapping & Arrangement:** `snap_left`, `snap_right`, `snap_top`, `snap_bottom`, `snap_third_left`, `snap_third_middle`, `snap_third_right`, `snap_percent_left`, `snap_percent_right`, `snap_percent_top`, `snap_percent_bottom`, `maximize`, `move_next_monitor`, `stack`
5. **Passthrough:** `passthrough` (hands the button click to the focused application untouched)

---

## 3. Factory Reset

To restore default settings and trigger first-run onboarding on next launch, delete `config.toml`:

```powershell
Remove-Item "$env:APPDATA\WiraDesk\config.toml"
```
