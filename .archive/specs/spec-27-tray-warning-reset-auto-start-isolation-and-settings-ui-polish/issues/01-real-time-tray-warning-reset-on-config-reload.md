---
id: SPEC-27-01
component: window-management
satisfies: []
blocked_by: []
status: closed
tests:
  - tray::tests::successful_reload_clears_config_warning_and_restores_normal_tray_state
  - tray::tests::non_config_warning_persists_across_successful_reload
  - tray::tests::successful_reload_does_not_downgrade_critical_state
  - config::tests::reload_with_valid_config_returns_applied_outcome
---

# 01: Real-time tray warning reset upon successful configuration reload

**What to build:**
1. In `crates/daemon/src/tray.rs`, introduce structured warning cause tracking in `TrayData` (e.g. `WarningCauses` tracking `config_rejected`, `acl_insecure`, and `simulated`).
2. On `WM_APP_RELOAD_CONFIG` in `wndproc_impl`:
   - Receive the `ReloadOutcome` from `crate::config::handle_reload_message(hwnd, data.hook_thread_id)`.
   - If `matches!(outcome, ReloadOutcome::Applied { .. })`, clear `data.warning_causes.config_rejected = false`.
   - If `matches!(outcome, ReloadOutcome::Rejected(_))`, set `data.warning_causes.config_rejected = true`.
   - Update `data.warning_latched = data.warning_causes.any_active()`.
   - If `!data.warning_latched && data.state == TrayState::Warning`, immediately transition the tray icon back to `TrayState::Normal` via `set_state(data, TrayState::Normal)`.
   - If `data.state == TrayState::Critical`, do not change `data.state` (preserves severity precedence where a dead hook outranks a clean config reload).
3. Add comprehensive unit tests in `tray.rs` and `config.rs` exercising:
   - Config reload failure sets `config_rejected` and latches `TrayState::Warning`.
   - Subsequent valid config reload clears `config_rejected` and restores `TrayState::Normal`.
   - If a non-config warning (e.g. `acl_insecure`) is active, a successful config reload does NOT clear `warning_latched` and leaves the tray in `TrayState::Warning`.
   - Critical hook state (`TrayState::Critical`) remains `Critical` even if a reload succeeds.

**Blocked by:** None.

**Status:** closed

## Acceptance Criteria

- [x] When a configuration reload succeeds with `ReloadOutcome::Applied`, the config-rejection warning cause is cleared (`data.warning_causes.config_rejected = false`).
- [x] If no other warning causes remain active and the tray is currently in `TrayState::Warning`, a successful reload immediately updates the icon to `TrayState::Normal`.
- [x] If another warning cause remains active (such as an insecure binary ACL), a successful config reload preserves the warning latch and does not downgrade the tray icon to `Normal`.
- [x] If the tray icon is in `TrayState::Critical`, a successful reload does not downgrade the icon to `Normal`.
- [x] If a reload fails with `ReloadOutcome::Rejected`, the config warning cause and `TrayState::Warning` remain active.
- [x] Tests named in frontmatter pass cleanly.
