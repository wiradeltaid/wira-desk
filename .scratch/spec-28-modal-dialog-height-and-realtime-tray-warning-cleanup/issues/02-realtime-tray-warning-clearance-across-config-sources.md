---
id: SPEC-28-02
component: window-management
satisfies: []
blocked_by: []
status: open
tests:
  - tray::tests::shortcut_collision_warning_clears_on_clean_reload
  - tray::tests::stale_collision_warning_cannot_relatch_after_successful_reload
  - tray::tests::autostart_acl_warning_clears_when_task_unregistered_or_admin_only
  - tray::tests::autostart_acl_warning_persists_if_task_remains_registered_in_non_admin_path
  - tray::tests::critical_hook_state_persists_across_collision_cleanup
---

# 02: Real-time tray warning clearance on shortcut collision repair and configuration re-evaluation

**What to build:**
1. In `crates/daemon/src/log.rs`:
   - Define constant `pub const WARN_CAUSE_CONFIG_COLLISION: usize = 4;`.
2. In `crates/daemon/src/hook.rs`:
   - In `load_shortcuts_from_config`, update collision warnings emitted from `unbind_duplicates` to use `crate::log::warn_with_cause(worker_hwnd, &format!(...), crate::log::WARN_CAUSE_CONFIG_COLLISION);`.
3. In `crates/daemon/src/tray.rs`:
   - Add field `pub config_collision: bool` to struct `WarningCauses`.
   - In `handle_log_warning`:
     - Map `crate::log::WARN_CAUSE_CONFIG_COLLISION` to `data.warning_causes.config_collision = true`.
   - In `handle_config_reload_outcome`:
     - Single ordered owner for configuration reload outcomes:
       - On `ReloadOutcome::Applied { .. }`:
         - Clear `data.warning_causes.config_rejected = false;`.
         - Clear `data.warning_causes.config_collision = false;`.
         - Verify confirmed scheduler state:
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
       - On `ReloadOutcome::Rejected(_)`:
         - Set `data.warning_causes.config_rejected = true;`.
     - Update `data.warning_latched = data.warning_causes.any_active();`.
     - If `!data.warning_latched && data.state == TrayState::Warning`:
       - Transition to `TrayState::Normal` via `set_state(data, TrayState::Normal)`.
     - Preserve `TrayState::Critical` precedence so a dead hook thread is never downgraded to Normal.
4. Add comprehensive unit tests in `tray.rs` covering:
   - Setting a shortcut collision warning sets `config_collision` and latches Warning.
   - Subsequent valid reload without collision clears `config_collision` and restores `Normal` in real time.
   - Interleaving test: collision warning -> clean reload -> delayed stale collision warning message cannot re-latch Warning state.
   - Disabling auto-start on reload clears an active `acl_insecure` warning if confirmed unregistered, but preserves the warning if the task remains registered in a non-admin directory.
   - Critical hook state (`TrayState::Critical`) remains unaffected by any reload outcome.

**Blocked by:** None.

**Status:** open

## Acceptance Criteria

- [ ] Shortcut collision warnings logged during shortcut binding are tagged with `WARN_CAUSE_CONFIG_COLLISION`.
- [ ] A successful configuration reload clears both `config_rejected` and `config_collision`.
- [ ] Delayed stale collision warning dispatch cannot re-latch Warning state after a clean applied reload.
- [ ] Auto-start ACL warning is re-evaluated against verified Task Scheduler status and binary path safety, clearing if unregistered or admin-only and persisting if the task remains registered in a non-admin path.
- [ ] When all active warning causes are resolved and the tray icon is in `TrayState::Warning`, it immediately resets to `TrayState::Normal` and re-renders via `Shell_NotifyIconW(NIM_MODIFY)`.
- [ ] Critical hook state (`TrayState::Critical`) is never downgraded by a reload outcome.
- [ ] Tests named in frontmatter pass cleanly.
