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
   - Add a test-addressable identifier on the dialog card (`accessible-role: group; accessible-label: "Factory reset confirmation card";`) and expose `property <length> modal_card_height: dialog_layout.preferred-height;` so that UI tests can measure the rendered card's actual geometry and vertical bounds rather than checking source code strings alone.
   - Center the card vertically and horizontally:
     ```slint
     x: (parent.width - self.width) / 2;
     y: (parent.height - self.height) / 2;
     width: 440px;
     height: dialog_layout.preferred-height;
     ```
   - Ensure the card is compact (~200px-230px tall), cleanly wrapping the title, description, and action buttons with consistent 24px padding and 14px spacing.
   - Add runtime Slint snapshot regression tests measuring the rendered card bounds directly: asserting that the card height is strictly bounded (`card.height < 260px` and `card.height >= 180px`) and vertically centered (`y > 100px` and `y + card.height < 460px`).

2. **Real-Time Tray Warning Clearance Across All Configuration Sources (`crates/daemon/src/tray.rs`, `crates/daemon/src/hook.rs`, `crates/daemon/src/log.rs`, `crates/daemon/src/config.rs`)**:
   - Introduce dedicated warning cause identifiers in `crates/daemon/src/log.rs`:
     - `WARN_CAUSE_CONFIG_COLLISION: usize = 4;`
   - In `crates/daemon/src/hook.rs`:
     - When unbinding duplicate shortcuts, tag the warning post with `WARN_CAUSE_CONFIG_COLLISION`.
   - In `crates/daemon/src/tray.rs`:
     - Model `config_collision: bool` in `WarningCauses`.
     - In `handle_log_warning`, map `WARN_CAUSE_CONFIG_COLLISION` to `data.warning_causes.config_collision = true`.
     - In `handle_config_reload_outcome`:
       - Single ordered owner for configuration reload outcomes:
         - On `ReloadOutcome::Applied { .. }`:
           - Clear both `data.warning_causes.config_rejected = false` and `data.warning_causes.config_collision = false`.
           - Verify confirmed scheduler state: check `crate::autostart::is_registered()`.
             - If `!crate::autostart::is_registered()`, confirmed that no auto-start task exists -> clear `data.warning_causes.acl_insecure = false`.
             - If task is registered, re-evaluate `crate::acl::replaceable_by_non_admin(&current_exe)`: if `AdminOnly`, clear `acl_insecure = false`; if `NonAdminWritable`, keep `acl_insecure = true`.
         - On `ReloadOutcome::Rejected(_)`:
           - Set `data.warning_causes.config_rejected = true`.
       - After updating causes, set `data.warning_latched = data.warning_causes.any_active();`.
       - If `!data.warning_latched && data.state == TrayState::Warning`, immediately transition `data.state` to `TrayState::Normal` and update the tray icon via `modify_icon(data)`.
       - If `data.state == TrayState::Critical`, preserve `Critical` state so that a dead keyboard hook is never downgraded.
   - Add unit and integration tests covering:
     - Collision warning sets `config_collision` and Warning state.
     - Interleaving test: collision warning -> clean applied reload -> delayed stale collision warning does not re-latch Warning.
     - Auto-start ACL warning is cleared if and only if the task is confirmed unregistered or registered at an admin-only path.
     - Critical hook state (`TrayState::Critical`) remains `Critical` across reloads.

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
- In `crates/daemon/src/log.rs`:
  - Define `pub const WARN_CAUSE_CONFIG_COLLISION: usize = 4;`.
- In `crates/daemon/src/hook.rs`:
  - When reporting duplicate shortcut unbinding, call `crate::log::warn_with_cause(worker_hwnd, &format!(...), crate::log::WARN_CAUSE_CONFIG_COLLISION);`.
- In `crates/daemon/src/tray.rs`:
  - Add `pub config_collision: bool` to `WarningCauses`.
  - In `handle_log_warning`:
    - On `WARN_CAUSE_CONFIG_COLLISION`, set `data.warning_causes.config_collision = true`.
  - In `handle_config_reload_outcome`:
    - When `ReloadOutcome::Applied { .. }` is received:
      - `data.warning_causes.config_rejected = false;`
      - `data.warning_causes.config_collision = false;`
      - Re-evaluate location safety against verified scheduler state:
        ```rust
        if !crate::autostart::is_registered() {
            data.warning_causes.acl_insecure = false;
        } else if let Ok(exe) = std::env::current_exe() {
            if crate::acl::replaceable_by_non_admin(&exe) == crate::acl::Verdict::AdminOnly {
                data.warning_causes.acl_insecure = false;
            } else {
                data.warning_causes.acl_insecure = true;
            }
        }
        ```
    - Recalculate `data.warning_latched = data.warning_causes.any_active();`.
    - If `!data.warning_latched && data.state == TrayState::Warning`, call `set_state(data, TrayState::Normal)`.
    - Retain `TrayState::Critical` guard.

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
