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
1. In `crates/daemon/src/config.rs`:
   - Evaluate shortcut collisions synchronously during `reload`:
     - Inspect the validated `Config` shortcuts against collision rules (`unbind_duplicates`).
     - Extend `ReloadOutcome::Applied`:
       ```rust
       pub enum ReloadOutcome {
           Applied {
               auto_start: bool,
               has_shortcut_collision: bool,
               generation: u64,
           },
           Rejected(String),
       }
       ```
     - Increment a monotonic `config_generation: u64` with each reload pass.
   - Extend `converge_auto_start` or add a scheduler observer returning a structured `TaskStatus`:
     ```rust
     #[derive(Debug, Clone, Copy, PartialEq, Eq)]
     pub enum TaskStatus {
         Absent,
         Registered,
         Unknown,
     }
     ```
     - Positive exit code 0 maps to `Registered`.
     - Explicit exit code 1 (task not found) maps to `Absent`.
     - Non-zero/non-one or execution failure maps to `Unknown` (fail-safe).
2. In `crates/daemon/src/log.rs`:
   - Define constant `pub const WARN_CAUSE_CONFIG_COLLISION: usize = 4;`.
3. In `crates/daemon/src/hook.rs`:
   - When duplicate shortcuts are unbound in `load_shortcuts_from_config`, tag warning with `WARN_CAUSE_CONFIG_COLLISION` and carry the snapshot's generation.
4. In `crates/daemon/src/tray.rs`:
   - Add fields to `WarningCauses`:
     ```rust
     pub config_collision: bool,
     pub last_applied_generation: u64,
     ```
   - In `handle_log_warning`:
     - Discard any collision warning whose generation is older than `data.warning_causes.last_applied_generation` (causal ownership prevents delayed stale messages from re-latching Warning state).
   - In `handle_config_reload_outcome`:
     - Synchronous authoritative owner for configuration reload outcomes:
       - On `ReloadOutcome::Applied { auto_start: _, has_shortcut_collision, generation }`:
         - Set `data.warning_causes.last_applied_generation = generation;`.
         - Clear `data.warning_causes.config_rejected = false;`.
         - Set `data.warning_causes.config_collision = has_shortcut_collision;`.
         - Re-evaluate location safety against observed scheduler state:
           - Query `autostart::task_status()`:
             - On `TaskStatus::Absent`: positively confirmed absent -> set `data.warning_causes.acl_insecure = false;`.
             - On `TaskStatus::Registered`:
               - Check `replaceable_by_non_admin(&current_exe)`:
                 - `Verdict::AdminOnly`: set `data.warning_causes.acl_insecure = false;`.
                 - `Verdict::NonAdminWritable`: set `data.warning_causes.acl_insecure = true;`.
                 - `Verdict::Unknown`: retain existing `acl_insecure` state (fail-safe).
             - On `TaskStatus::Unknown`: retain existing `acl_insecure` state (fail-safe; never infer safety from an observation error).
       - On `ReloadOutcome::Rejected(_)`:
         - Set `data.warning_causes.config_rejected = true;`.
     - Update `data.warning_latched = data.warning_causes.any_active();`.
     - If `!data.warning_latched && data.state == TrayState::Warning`:
       - Transition to `TrayState::Normal` via `set_state(data, TrayState::Normal)`.
     - Preserve `TrayState::Critical` precedence so a dead hook thread is never downgraded to Normal.
5. Add comprehensive unit tests in `tray.rs` covering:
   - Setting a shortcut collision warning sets `config_collision` and latches Warning.
   - Subsequent valid reload without collision clears `config_collision` and restores `Normal` in real time.
   - Interleaving test: collision warning -> clean reload -> delayed stale collision warning message cannot re-latch Warning state.
   - Scheduler observation test: `TaskStatus::Absent` clears warning; `TaskStatus::Registered` with non-admin path latches warning; `TaskStatus::Unknown` or `Verdict::Unknown` fail-safely retains previous warning state.
   - Critical hook state (`TrayState::Critical`) remains unaffected by any reload outcome.

**Blocked by:** None.

**Status:** open

## Acceptance Criteria

- [ ] Shortcut collision warnings are tagged with `WARN_CAUSE_CONFIG_COLLISION` and bound to config generation.
- [ ] `ReloadOutcome::Applied` synchronously owns `has_shortcut_collision` and updates `config_collision` in real time.
- [ ] Delayed stale collision warning dispatch cannot re-latch Warning state after a clean applied reload.
- [ ] Auto-start ACL warning is re-evaluated against verified Task Scheduler observation (`TaskStatus`), clearing only on positively confirmed absence or admin-only path, and fail-safely retaining warning on `Unknown`.
- [ ] When all active warning causes are resolved and the tray icon is in `TrayState::Warning`, it immediately resets to `TrayState::Normal` and re-renders via `Shell_NotifyIconW(NIM_MODIFY)`.
- [ ] Critical hook state (`TrayState::Critical`) is never downgraded by a reload outcome.
- [ ] Tests named in frontmatter pass cleanly.
