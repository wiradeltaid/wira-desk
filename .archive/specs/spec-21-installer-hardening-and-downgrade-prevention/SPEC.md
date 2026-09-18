# SPEC-21: Installer Hardening — Downgrade Prevention, Path Confirmation, and Fail-Closed Process Termination

## Problem Statement

The Wira Desk Windows installer (`packaging/wiradesk.iss`) has four safety gaps:

1. **Silent version downgrade:** An older installer can overwrite a newer Wira Desk installation. This can reintroduce fixed vulnerabilities or pair older binaries with a configuration written by a newer release.
2. **Incomplete install confirmation:** The *Ready to Install* page shows only `{app}`. It does not explain that configuration and logs remain under `%APPDATA%\WiraDesk`, or that auto-start is an optional elevated logon task rather than a Windows service or an action performed by Setup.
3. **Fail-open process termination:** `PrepareToInstall` always returns an empty string. If the daemon survives the bounded `WM_CLOSE`/force-kill path, or Settings survives its non-forced close request, extraction can continue into a locked binary and leave the user with an OS-level copy failure.
4. **Ambiguous version boundary:** `AppVersion` comes from the daemon binary resource and currently has no installer-local assertion that it is a stable three-component version. A parser that truncates a fourth component or prerelease/build metadata would compare a different version than the installer displays.

## Solution

Adopt the relevant Snapdown patterns while preserving Wira Desk's materially different architecture: Wira Desk is a per-machine elevated install with an `HKLM` uninstall record, two binaries, and an optional task owned by the daemon after installation; it is not Snapdown's per-user `HKCU` install with a selectable data vault.

1. **Downgrade prevention (`InitializeSetup`):** Add one strict `X.Y.Z` parser and numeric comparator in `[Code]`. Query the existing AppId's `DisplayVersion` in the explicit 64-bit `HKLM` registry view. A missing uninstall key means first install and proceeds. A present but blank or malformed `DisplayVersion`, or one numerically newer than the installer version, aborts before file operations. Interactive runs show the reason; `/SILENT` and `/VERYSILENT` log it and exit non-zero without waiting for a dialog.
2. **Installation summary (`UpdateReadyMemo`):** On wizard runs, add the application destination, the current user's `%APPDATA%\WiraDesk` configuration/log location, and the exact `WiraDesk` task name. The text must state that auto-start is optional, enabled later from Settings or the tray, and is not installed or enabled by Setup. The memo is descriptive only: it neither changes the fixed Program Files destination nor creates user data or a task.
3. **Fail-closed process termination:** Refactor shutdown into a result-returning operation. It keeps the daemon's polite `WM_CLOSE` and bounded `/F` fallback, then verifies the daemon window and process are both absent. It requests Settings exit without `/F` to avoid discarding an unsaved draft and verifies that process is absent. `PrepareToInstall` returns a descriptive error if either executable remains, so Inno aborts before `[Files]` extraction.
4. **Compile-time version guard:** Add an ISPP guard that accepts only stable, exactly three-component decimal `major.minor.patch` `AppVersion` values. It rejects missing components, a fourth component, empty or non-decimal components, and `v`, `-`, or `+` metadata. This matches the installer comparison contract; prerelease/install-channel semantics are explicitly out of scope rather than silently miscompared.

## User Outcomes

1. An older installer cannot replace a newer installation. The user receives an explanation interactively; unattended callers receive a non-zero failure and an installer log entry.
2. An equal version is an allowed reinstall, and a newer version is an allowed upgrade. Both preserve the fixed installation location and existing configuration.
3. The Ready page accurately identifies all persistent locations and makes clear that no scheduled task exists until the user enables auto-start.
4. A surviving daemon or Settings process stops the install cleanly before any files are extracted.
5. `/SILENT` and `/VERYSILENT` use the same downgrade and process-safety policy as the wizard. The path memo remains wizard-only because silent callers cannot review it.

## Implementation Decisions

1. **Stable version grammar:** `AppVersion` and every accepted `DisplayVersion` are exactly `major.minor.patch`, with each component a non-empty decimal integer. Comparison is numeric, not lexical (`0.10.0` is newer than `0.9.9`); equal versions are allowed. A malformed existing uninstall value fails closed because proceeding would make downgrade protection unverifiable.
2. **Registry lookup:** Use the existing AppId uninstall key, `Software\Microsoft\Windows\CurrentVersion\Uninstall\{7E4F9C21-6B3D-4A88-9F14-2C5E8D0A1B73}_is1`, under `HKLM` with an explicit 64-bit registry view. Restore the prior registry view after the lookup so this code cannot affect later installer operations.
3. **Silent safety:** The downgrade branch must log the installed and candidate versions and return `False` in every mode. It must not display a modal dialog in a silent run. The process-termination failure is returned from `PrepareToInstall`, which makes the installer surface its ordinary failure result without extracting files.
4. **Process identity and exit verification:** A `taskkill` exit code alone is not evidence of exit. Verify the daemon's named window is absent and verify both executable image names are absent after their respective bounded shutdown paths. Settings remains non-forced; a surviving Settings process is a clean abort, not a reason to discard a draft.
5. **Ready memo wording:** Use `ExpandConstant('{userappdata}\WiraDesk')` for the displayed configuration/log path and name `WiraDesk` literally for the optional task. Do not call the task a service, imply it is created during install, or offer either location as configurable.
6. **Evidence runs the real installer:** Add a PowerShell installer-safety harness, invoked by CI after the existing installer compile. It stages the release binaries, compiles the installer, creates and removes an isolated 64-bit `HKLM` AppId fixture, invokes `/VERYSILENT`, and proves each decision-table result from the installer exit code and filesystem state: no existing key, older installed version, equal installed version, newer installed version, and malformed installed version. It also compiles deliberate invalid-version fixtures to prove the ISPP guard rejects four-component and metadata-bearing versions. The harness must clean every registry and filesystem fixture in `finally`.

## Testing and Evidence

- Compile the installer with Inno Setup 6.7.3 and retain the existing compiler-version floor assertion.
- Run the installer-safety harness described above in CI; its checks exercise Inno Pascal code and registry-view behavior rather than reimplementing comparison in Rust or PowerShell.
- Manually verify the interactive Ready page on a test machine: it accurately identifies `{app}`, `%APPDATA%\WiraDesk`, and the optional user-controlled `WiraDesk` task.
- Manually verify process refusal with a deliberately non-terminating fixture or a controlled locked-binary setup; verify no `[Files]` extraction occurred before the informative failure.
- Run `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, and `$env:WIRADESK_SKIP_MANIFEST = '1'; cargo test --workspace --locked`.

## Out of Scope

- Changing `PrivilegesRequired=admin`, the per-machine Program Files destination, or the daemon's ownership of auto-start.
- Introducing a selectable install directory, a per-user install, a service, or a task created by Setup.
- Permitting or ordering prerelease/build-metadata installers; that needs an explicit release-channel design rather than a truncated comparison.
- Changing the installer compiler toolchain or adding localization.
