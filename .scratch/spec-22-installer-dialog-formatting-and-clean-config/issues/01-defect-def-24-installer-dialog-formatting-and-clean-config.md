---
id: SPEC-22-01
component: settings
satisfies: []
blocked_by: []
status: ready-for-agent
tests:
  - scripts/verify-installer-safety.ps1
  - ci::build::installer-safety
---

# 01: Defect DEF-24 — Installer dialog text formatting and clean-installation configuration clarity

**What to build:** Refine `packaging/wiradesk.iss` dialog formatting and configuration lifecycle clarity.

1. **Joined Sentence Formatting in Rejection Dialogs (`InitializeSetup`):**
   - Eliminate premature `#13#10` linebreaks that orphan trailing words onto separate lines in `InitializeSetup`.
   - Change downgrade message to:
     `'If you wish to install an older version, please uninstall the current version first.'`
   - Change invalid record message to:
     `'Setup cannot verify version compatibility. Please uninstall the current version before continuing.'`
2. **Configuration and Log Lifecycle Disclosure (`UpdateReadyMemo`):**
   - Update `UpdateReadyMemo` in `packaging/wiradesk.iss` to state that `%APPDATA%\WiraDesk` is preserved when already present and that Setup never bundles or overwrites user state.
   - The implementation and its wording must distinguish runtime behavior from installer behavior: a missing `config.toml` opens first-run onboarding, which writes a fresh default configuration only when onboarding completes; `wiradesk.log` is created only when the daemon emits a log entry. Neither file is copied from the repository or pre-created by Setup.
3. **Safety Harness Assertions (`scripts/verify-installer-safety.ps1`):**
   - Update `scripts/verify-installer-safety.ps1` to assert the joined dialog sentence patterns, absence of the two orphaned mid-sentence linebreaks, the Ready-page preservation disclosure, and the `[Files]` zero-bundling invariant for `config.toml` and `wiradesk.log`.

**Blocked by:** none

**Status:** ready-for-agent

## Acceptance Criteria

- [ ] **Joined Downgrade Refusal Text:** `packaging/wiradesk.iss` contains `'If you wish to install an older version, please uninstall the current version first.'` on a single continuous string without a mid-sentence `#13#10` before `version first.`.
- [ ] **Joined Invalid Version Refusal Text:** `packaging/wiradesk.iss` contains `'Setup cannot verify version compatibility. Please uninstall the current version before continuing.'` on a single continuous string without a mid-sentence `#13#10` before `version before continuing.`.
- [ ] **Ready Memo Lifecycle Disclosure:** `UpdateReadyMemo` states that `%APPDATA%\WiraDesk` is preserved if present and that Setup never bundles or overwrites user state.
- [ ] **Runtime Initialization Boundary:** The implementation-facing copy distinguishes the runtime lifecycle: a missing configuration opens first-run onboarding, completion writes a fresh default `config.toml`, and `wiradesk.log` is created only when the daemon writes a log entry. It does not claim that Setup or application startup eagerly creates a log file.
- [ ] **Zero Bundled Config Invariant:** `packaging/wiradesk.iss` `[Files]` section continues to install only `wiradesk.exe`, `wiradesk-settings.exe`, `LICENSE.txt`, and `NOTICE.txt` (never bundling `config.toml` or `wiradesk.log`).
- [ ] **Safety Harness Verification:** `scripts/verify-installer-safety.ps1` checks the joined dialog strings, the two absent orphaned-linebreak patterns, the Ready-page preservation disclosure, and the `[Files]` zero-bundling invariant, then passes cleanly without regression.
- [ ] **Workspace Integrity:** `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `$env:WIRADESK_SKIP_MANIFEST = '1'; cargo test --workspace` pass cleanly.
