# SPEC-28: Compact Modal Dialog Height and Comprehensive Real-Time Tray Warning Clearance

## Problem Statement

Following user testing of Wira Desk v0.2.4, two usability defects and lifecycle gaps were identified:
1. **Modal Dialog Height Stretched to Window:** When clicking `[Reset all settings…]` in the About pane, the "Restore all preferences to defaults?" confirmation dialog container stretches nearly to the full height of the 560px window. This occurs because the dialog's `Rectangle` container within `modal_focus := FocusScope` specifies `width: 440px;` without an explicit or content-proportional height constraint (such as `height: dialog_layout.preferred-height;`), causing Slint's layout resolver to expand the card to the full height of the parent window.
2. **Tray Warning Dot Does Not Clear in Real-Time:** When a user encounters a configuration or shortcut collision problem, the tray icon displays a red warning dot. After fixing the problem in Settings and saving:
   - Shortcut collision warnings logged during `unbind_duplicates` in `crates/daemon/src/hook.rs` post `WM_APP_LOG_WARNING` with `WARN_CAUSE_GENERIC`, which falls through in `crates/daemon/src/tray.rs` to latch `data.warning_causes.simulated = true`. A subsequent successful reload clears only `data.warning_causes.config_rejected`, leaving `simulated` permanently latched. Furthermore, asynchronous warning messages from the Hook thread could arrive out-of-order after a clean reload.
   - If auto-start is registered while the binary runs from a non-admin location (e.g. `target\release` or user profile during testing), `warn_if_location_replaceable` latches `data.warning_causes.acl_insecure = true`. On config reload, even if the user toggles auto-start off in Settings, `acl_insecure` is never re-evaluated against verified Task Scheduler state or location safety, leaving the warning icon latched.
   - Consequently, the tray icon does not return to normal in real time upon resolving these configuration and preference issues.

## Solution

1. **Compact Modal Dialog Height (`crates/settings/ui/main_window.slint`)**:
   - Bind the Factory Reset dialog card container's height directly to its content layout: `height: dialog_layout.preferred-height;`.
   - Name the inner `VerticalLayout` as `dialog_layout := VerticalLayout { ... }`.
   - Add a test-addressable identifier on the dialog card (`accessible-role: group; accessible-label: "Factory reset confirmation card";`) so that UI test harnesses can locate and measure the rendered card's actual geometry and vertical bounds.
   - Center the card vertically and horizontally:
     ```slint
     x: (parent.width - self.width) / 2;
     y: (parent.height - self.height) / 2;
     width: 440px;
     height: dialog_layout.preferred-height;
     ```
   - Ensure the card is compact (~200px-230px tall), cleanly wrapping the title, description, and action buttons with consistent 24px padding and 14px spacing.
   - Add runtime Slint snapshot regression tests measuring the rendered card bounds directly at normal window geometry (760×560): asserting that the card height is strictly bounded (`card.height < window.height / 2`, `< 280px` and `>= 180px`) and vertically centered with midpoint within 10px tolerance of window vertical center.

2. **Real-Time Tray Warning Clearance Across All Configuration Sources (crates/daemon/src/tray.rs, crates/daemon/src/hook.rs, crates/daemon/src/log.rs, crates/daemon/src/config.rs)**:
   - Causal ownership and synchronous derivation for shortcut collisions:
     - In crates/daemon/src/config.rs, assign a monotonic config_generation: u64 and evaluate duplicate shortcut bindings synchronously during 
eload:
       `
ust
       pub enum ReloadOutcome {
           Applied {
               auto_start: bool,
               has_shortcut_collision: bool,
               generation: u64,
           },
           Rejected(String),
       }
       `
   - Introduce dedicated warning cause identifiers in crates/daemon/src/log.rs:
     - WARN_CAUSE_CONFIG_COLLISION: usize = 4;
   - In crates/daemon/src/hook.rs:
     - When unbinding duplicate shortcuts, tag the warning post with WARN_CAUSE_CONFIG_COLLISION and carry the snapshot's generation.
   - In crates/daemon/src/tray.rs:
     - Model config_collision: bool and last_applied_generation: u64 in WarningCauses.
     - In handle_log_warning, ignore any collision warning message older than last_applied_generation.
     - In handle_config_reload_outcome:
       - Single synchronous authoritative owner for configuration reload outcomes:
         - On ReloadOutcome::Applied { auto_start: _, has_shortcut_collision, generation }:
           - Record data.warning_causes.last_applied_generation = generation;.
           - Clear data.warning_causes.config_rejected = false;.
           - Synchronously update data.warning_causes.config_collision = has_shortcut_collision;.
           - Verify confirmed scheduler state via structured TaskStatus:
             - If TaskStatus::Absent: positively confirmed that no auto-start task exists -> set data.warning_causes.acl_insecure = false;.
             - If TaskStatus::Registered:
               - Re-evaluate crate::acl::replaceable_by_non_admin(&current_exe):
                 - Verdict::AdminOnly: clear data.warning_causes.acl_insecure = false;.
                 - Verdict::NonAdminWritable: set data.warning_causes.acl_insecure = true;.
                 - Verdict::Unknown: fail-safely retain previous cl_insecure state.
             - If TaskStatus::Unknown: fail-safely retain previous cl_insecure state (never infer safety from an observation failure).
         - On ReloadOutcome::Rejected(_):
           - Set data.warning_causes.config_rejected = true;.
       - After updating causes, set data.warning_latched = data.warning_causes.any_active();.
       - If !data.warning_latched && data.state == TrayState::Warning, immediately transition data.state to TrayState::Normal and update the tray icon via modify_icon(data).
       - If data.state == TrayState::Critical, preserve Critical state so that a dead keyboard hook is never downgraded.
   - Add unit and integration tests covering:
     - Synchronous reload outcome clearing collision and restoring Normal state.
     - Interleaving test: collision warning -> clean applied reload -> delayed stale collision warning cannot re-latch Warning.
     - Auto-start ACL warning is cleared if and only if the task is confirmed unregistered or registered at an admin-only path; retained fail-safely on query failure or unknown ACL verdict.
     - Critical hook state (TrayState::Critical) remains Critical across reloads.

## User Stories

1. As a user opening the "Restore all preferences to defaults?" confirmation dialog, I want the modal window to be a compact card sized to fit its text and buttons, so that it does not unnecessarily dominate the screen with empty vertical space.
2. As a keyboard or mouse user viewing the confirmation dialog, I want the dialog to be vertically centered and visually balanced within the Settings window.
3. As a user who fixed a shortcut collision in Settings and clicked Save, I want the system tray icon to immediately remove its red warning indicator in real time, confirming the app is healthy.
4. As a user who disabled auto-start after seeing a location warning, I want the system tray warning to clear immediately on save without having to restart the daemon, provided the task is verified unregistered.
5. As a user with an unrecoverable keyboard hook failure (`TrayState::Critical`), I want the icon to remain critical even if a configuration reload succeeds.

## Implementation Decisions

- In `crates/settings/ui/main_window.slint`:
  - Update `modal_focus` inner card `Rectangle`:
    ```slint
    Rectangle {
        width: 440px;
        height: dialog_layout.preferred-height;
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
        background: Palette.bg_card;
        border-width: 1px;
        border-color: Palette.stroke_card;
        border-radius: 12px;
        drop-shadow-blur: 24px;
        drop-shadow-color: Palette.is_dark ? rgba(0, 0, 0, 0.6) : rgba(0, 0, 0, 0.2);
        drop-shadow-offset-y: 8px;
        accessible-role: group;
        accessible-label: "Factory reset confirmation card";

        dialog_layout := VerticalLayout {
            padding: 24px;
            spacing: 14px;
            ...
        }
    }
    ```
- In crates/daemon/src/config.rs:
  - Extend ReloadOutcome::Applied to carry has_shortcut_collision: bool and generation: u64.
  - Increment monotonic config_generation: u64 during 
eload.
  - Introduce utostart::task_status() returning structured TaskStatus (Absent, Registered, Unknown).
- In crates/daemon/src/log.rs:
  - Define pub const WARN_CAUSE_CONFIG_COLLISION: usize = 4;.
- In crates/daemon/src/hook.rs:
  - When duplicate shortcuts are unbound, tag warning with WARN_CAUSE_CONFIG_COLLISION and carry the snapshot generation.
- In crates/daemon/src/tray.rs:
  - Add pub config_collision: bool and pub last_applied_generation: u64 to WarningCauses.
  - In handle_log_warning:
    - Drop collision warnings with generation older than last_applied_generation.
  - In handle_config_reload_outcome:
    - When ReloadOutcome::Applied { auto_start: _, has_shortcut_collision, generation } is received:
      - data.warning_causes.last_applied_generation = generation;
      - data.warning_causes.config_rejected = false;
      - data.warning_causes.config_collision = has_shortcut_collision;
      - Re-evaluate location safety against observed utostart::task_status():
        `
ust
        match crate::autostart::task_status() {
            TaskStatus::Absent => data.warning_causes.acl_insecure = false,
            TaskStatus::Registered => {
                if let Ok(exe) = std::env::current_exe() {
                    match crate::acl::replaceable_by_non_admin(&exe) {
                        crate::acl::Verdict::AdminOnly => data.warning_causes.acl_insecure = false,
                        crate::acl::Verdict::NonAdminWritable => data.warning_causes.acl_insecure = true,
                        crate::acl::Verdict::Unknown => {}, // fail-safe: retain
                    }
                }
            }
            TaskStatus::Unknown => {}, // fail-safe: retain
        }
        `
    - Recalculate data.warning_latched = data.warning_causes.any_active();.
    - If !data.warning_latched && data.state == TrayState::Warning, call set_state(data, TrayState::Normal).
    - Retain TrayState::Critical guard.
## Testing Decisions

- Slint UI tests in `crates/settings/src/shortcut_row_slint_snapshot.rs`:
  - Assert via UI automation / AccessKit geometry that opening the Factory Reset confirmation dialog renders a card whose measured height is between `180px` and `250px`, strictly less than half the window height (`< 280px`).
  - Assert that the dialog card is vertically centered within `[y > 100px, y + height < 460px]`.
- Daemon tests in `crates/daemon/src/tray.rs`:
  - Verify that a shortcut collision warning sets `config_collision` and transitions to `TrayState::Warning`.
  - Verify that a subsequent reload with a collision-free config clears `config_collision` and immediately restores `TrayState::Normal`.
  - Interleaving test: collision warning -> clean reload -> delayed stale collision warning message does not re-latch Warning state.
  - Verify that an `acl_insecure` warning clears when auto-start is verified absent or moved to an admin-only path, and persists if the task remains registered in a non-admin path.
  - Verify that `TrayState::Critical` is never downgraded by any configuration reload outcome.

## Out of Scope

- Changing the text or button choices of the Factory Reset dialog.
- Modifying how shortcuts are unbound at startup or during runtime.
