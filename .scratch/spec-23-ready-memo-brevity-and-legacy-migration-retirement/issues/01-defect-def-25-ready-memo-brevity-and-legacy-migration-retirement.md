---
id: SPEC-23-01
component: settings
satisfies: []
blocked_by: []
status: ready-for-agent
tests:
  - scripts/verify-installer-safety.ps1
  - ci::build::installer-safety
---

# 01: Defect DEF-25 — Ready memo brevity and legacy WinTick migration retirement

**What to build:** Simplify installer Ready memo copywriting and retire legacy WinTick migration.

1. **Ready to Install Memo Brevity (`packaging/wiradesk.iss`):**
   - Streamline `UpdateReadyMemo` configuration and logs copy to be clean, professional, and concise.
   - Replace the internal runtime explanation with this accurate summary:
     `Preserved across updates; clean installs start fresh.`
   - This wording deliberately makes no unconditional uninstall-preservation claim: the uninstaller can remove user state when the user explicitly requests it.
   - Maintain the zero-bundling invariant in `[Files]`. Update `scripts/verify-installer-safety.ps1` to require the concise wording and reject the retired onboarding/log-lifecycle prose.

2. **Retire All Legacy WinTick Migration Shims (`M-01`, `M-02`, `M-03`):**
   - Remove the legacy `WinTick` migration shims across the workspace:
     - `M-01`: Remove `crates/shared/src/migrate.rs` and its calls in `crates/daemon/src/main.rs` and `crates/settings/src/main.rs`.
     - `M-02` & `M-03`: Remove `crates/daemon/src/legacy.rs` (`migrate_scheduled_task` and `stop_legacy_daemon`) and its calls in `crates/daemon/src/main.rs`.
   - WinTick was an early prototype name from July 2026. Silent automated migration from `%APPDATA%\WinTick` inadvertently resurrects obsolete pre-DEC-008 shortcuts (`ctrl+win+left`, `ctrl+win+right`, `ctrl+win+down`) and historical July 2026 log files whenever a user attempts a clean install by clearing `%APPDATA%\WiraDesk`.
   - Retire the migration-only tests. Retain or extend default-configuration coverage to establish the current defaults (`ctrl+alt+left`, `ctrl+alt+right`, `ctrl+alt+shift+s`).
   - A clean start may create current runtime log entries on demand; the requirement is that it imports no historical WinTick config, logs, scheduled tasks, or mutexes.
   - Clean up documentation references to WinTick shims in `.how/settings/SDD-settings.md` and `.how/_platform/` architecture inventories.

**Blocked by:** none

**Status:** ready-for-agent

## Acceptance Criteria

- [ ] **Concise Ready Memo Copy:** `packaging/wiradesk.iss` `UpdateReadyMemo` presents exactly `Preserved across updates; clean installs start fresh.` for Configuration and logs, without the retired multi-line internal runtime explanation or an unconditional uninstall-preservation claim.
- [ ] **Safety Harness Verification:** `scripts/verify-installer-safety.ps1` requires the concise Ready memo wording, rejects the retired verbose wording, preserves the zero-bundling assertion, and passes cleanly without regression.
- [ ] **Complete Legacy WinTick Migration Retirement:** All WinTick shims (`M-01` in `crates/shared/src/migrate.rs`, `M-02` and `M-03` in `crates/daemon/src/legacy.rs`) and their startup calls are removed from production code. No production code references `WinTick` paths, registry keys, scheduled tasks, or mutexes.
- [ ] **Clean Default Configuration with Legacy Residue Present:** With `%APPDATA%\WiraDesk` absent even when `%APPDATA%\WinTick` exists, Wira Desk follows the standard first-run/default-configuration path with modern bindings (`ctrl+alt+left`, `ctrl+alt+right`, `ctrl+alt+shift+s`) and imports no legacy log content.
- [ ] **Workspace Integrity:** `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `$env:WIRADESK_SKIP_MANIFEST = '1'; cargo test --workspace` pass cleanly.
