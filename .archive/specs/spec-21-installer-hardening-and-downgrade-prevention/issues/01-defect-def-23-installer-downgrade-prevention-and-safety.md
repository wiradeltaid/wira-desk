---
id: SPEC-21-01
component: settings
satisfies: []
blocked_by: []
status: closed
tests:
  - scripts/verify-installer-safety.ps1
  - ci::build::installer-safety
---

# 01: Defect DEF-23 — Installer downgrade prevention, installation path confirmation, and fail-closed process shutdown

**What to build:** Harden `packaging/wiradesk.iss` using Snapdown only as a pattern source. Preserve Wira Desk's per-machine elevated design, dual executable lifecycle, fixed Program Files destination, and daemon-owned optional auto-start task.

1. **Strict downgrade prevention in `InitializeSetup`:**
   - Add a single strict `ParseVersion(const VStr: String; var Major, Minor, Patch: Integer): Boolean` and numeric `CompareVersions(const V1Str, V2Str: String): Integer` for exactly three non-empty decimal components.
   - Do not trim a fourth component or `-`/`+` metadata. Those inputs are invalid.
   - Read `DisplayVersion` for the current AppId from `HKLM\Software\Microsoft\Windows\CurrentVersion\Uninstall\{7E4F9C21-6B3D-4A88-9F14-2C5E8D0A1B73}_is1` in an explicit 64-bit registry view; restore the prior view afterward.
   - Allow a missing key, an older installed version, and an identical installed version. Reject a newer installed version and a present-but-blank-or-malformed installed version before extraction.
   - In interactive mode, show the refusal reason. In `/SILENT` or `/VERYSILENT`, log it and return `False` without displaying a modal dialog.
2. **Transparent path confirmation in `UpdateReadyMemo`:**
   - On the interactive Ready page, show the application destination (`{app}`), the current user's configuration/log location (`{userappdata}\WiraDesk`), and the optional auto-start task name (`WiraDesk`).
   - State that Setup neither creates nor enables the task; the user enables it later from Settings or the tray. Do not call it a service and do not imply any listed location is configurable.
3. **Fail-closed process shutdown in `PrepareToInstall`:**
   - Make the shutdown routine report whether the daemon and Settings have actually exited. A successful `taskkill` invocation is insufficient evidence.
   - Preserve the daemon's polite `WM_CLOSE`, bounded `/F` fallback, and post-fallback check. Verify both the daemon window and `wiradesk.exe` process are absent.
   - Request Settings exit without `/F`, preserving unsaved edits. Verify `wiradesk-settings.exe` is absent after the request.
   - If either executable remains, return an informative error string from `PrepareToInstall`; no `[Files]` action may begin.
4. **Compile-time stable-version guard:**
   - Add ISPP validation for `{#AppVersion}` that allows only `major.minor.patch`, with exactly two separators and decimal components. Reject an empty component, a fourth component, a leading `v`, and prerelease/build metadata.
5. **Executable installer evidence:**
   - Add `scripts/verify-installer-safety.ps1`, invoked in the CI build job after the ordinary installer compile. It must compile and run the real installer against isolated, cleaned-up fixtures rather than testing a reimplemented parser.
   - The harness must verify no-key, installed-older, installed-equal, installed-newer, and malformed-installed-version decisions using the installer's exit code and absence/presence of extracted files. It must compile fixtures demonstrating ISPP rejection of four-component and metadata-bearing candidate versions. All registry and filesystem fixtures must be removed in `finally`.

**Blocked by:** none

**Status:** ready-for-agent

## Acceptance Criteria

- [x] **Compile-Time Version Guard:** `packaging/wiradesk.iss` accepts only a stable, exactly-three-component decimal `AppVersion`. `0.2.0.0`, `v0.2.0`, `0.2`, `0.2.0-rc.1`, and `0.2.0+build.1` fail compilation.
- [x] **64-bit HKLM Lookup:** The installer reads the existing AppId uninstall record in an explicit 64-bit `HKLM` registry view and restores the previous view after the query.
- [x] **Downgrade Rejection:** A newer valid installed version, or a malformed/blank `DisplayVersion` in an existing AppId record, aborts in `InitializeSetup` before extraction. Interactive mode explains the refusal; silent modes log and exit non-zero without a modal dialog.
- [x] **Clean Upgrade and Reinstall:** A missing uninstall record, an older installed version, and an identical installed version proceed without a downgrade prompt.
- [x] **Numeric SemVer Ordering:** The comparison treats `0.10.0` as newer than `0.9.9`, `0.2.10` as newer than `0.2.9`, and equality as neither newer nor older.
- [x] **Ready to Install Summary:** The interactive Ready page explicitly shows `{app}`, `{userappdata}\WiraDesk`, and optional task `WiraDesk`; it states that Setup does not create or enable auto-start and does not call it a service.
- [x] **Fail-Closed Process Shutdown:** A surviving daemon window/process or Settings process makes `PrepareToInstall` abort with an informative error before `[Files]` extraction. The daemon retains its polite-close then bounded-force fallback; Settings is never force-killed.
- [x] **Executable Installer Tests:** `scripts/verify-installer-safety.ps1` runs the real installer and proves every registry decision-table row plus candidate-version ISPP rejections, cleaning its `HKLM` and filesystem fixtures even if an assertion fails. CI runs it after installer compilation.
- [x] **Workspace Integrity:** `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, and `$env:WIRADESK_SKIP_MANIFEST = '1'; cargo test --workspace --locked` pass cleanly.
