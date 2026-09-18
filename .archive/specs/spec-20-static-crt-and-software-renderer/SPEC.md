# SPEC-20: Static CRT Linking via Slint Software Renderer Migration and Installer Cleanup

## Problem Statement

On a clean Windows installation without the Microsoft Visual C++ Redistributable, either published executable can fail at process startup with `STATUS_DLL_NOT_FOUND (0xC0000135)` because its PE import table names a dynamic MSVC C/C++ runtime DLL. This blocks the loose-binary distribution and causes package-validation failures. The current installer fallback is unsafe evidence: CI neither stages a redistributable payload nor verifies its presence, and the release scanner can be bypassed by a text match in the installer script.

The daemon can already build with a static CRT. Settings prevents a workspace-wide policy because its Slint Skia renderer pulls native C++ Skia binaries that cannot use the required static MSVC runtime configuration in the supported build environment.

## Solution

Replace Settings' Slint Skia renderer with Slint's software renderer, enable `+crt-static` for the Windows MSVC target across the workspace, and delete the no-longer-valid Visual C++ Redistributable installer branch. Make the PE import gate strict: it scans the two release artifacts just built, rejects malformed or missing PE inputs, and rejects every dynamic MSVC CRT import family. The outcome is two self-contained release executables; it is not a new installer feature or a renderer redesign.

## User Outcomes

1. A user can start the elevated daemon from the published loose-binary pair on a clean Windows installation without a missing-runtime dialog.
2. With that daemon running, the user can open Settings from the tray and use all five panes without a missing-runtime dialog or a renderer startup failure.
3. Package validation receives binaries with no dynamic MSVC CRT imports, so a stale installer declaration cannot mask a dependency regression.
4. The installer contains only the application payload and existing application assets; it no longer conditionally installs a redistributable that portable distribution cannot use.
5. Settings remains usable in light and dark themes, at 100%, 125%, 150%, and 200% DPI, and in a software-rendered VM/RDP session.

## Implementation Decisions

1. **Software renderer and dependency graph.** In `crates/settings/Cargo.toml`, replace Slint's `renderer-skia` feature with `renderer-software`, retain `backend-winit`, `accessibility`, and `compat-1-2`, and set `default-features = false` on the direct `i-slint-backend-winit` dependency. Update `Cargo.lock`. `cargo tree -p settings` must contain neither `skia-bindings` nor `skia-safe`.
2. **Target-scoped static CRT.** Create `.cargo/config.toml` with `rustflags = ["-C", "target-feature=+crt-static"]` only under `[target.x86_64-pc-windows-msvc]`. It applies to every workspace binary and does not alter unrelated targets.
3. **Installer cleanup.** Remove the `vc_redist.x64.exe` `[Files]` entry, its `[Run]` command, and their fallback comments from `packaging/wiradesk.iss`; the installer must not silently skip a dependency it claims to supply.
4. **Strict binary gate.** `scripts/verify-release-binary.ps1` must no longer accept `-AllowDynamicCrtIfBundled` or inspect installer text. It must require and scan exactly `wiradesk.exe` and `wiradesk-settings.exe` from the supplied release-artifact directory, fail if either is absent or malformed, and fail on a case-insensitive import of `VCRUNTIME*.dll`, `MSVCP*.dll`, `UCRTBASE.dll`, or `api-ms-win-crt-*.dll`. The shared in-memory scanner and its tests must use the same definition.
5. **CI and release parity.** Both `.github/workflows/ci.yml` and `.github/workflows/release.yml` must run the strict scanner immediately after their locked release build and before installer packaging. Neither workflow may supply a bypass flag.
6. **Documentation and generated licensing inventory.** Update `AD-11a` in `.how/settings/SDD-settings.md` and the accepted `.constitution/project/codebase-stack-guide.md` to name the software renderer and target-scoped static CRT rationale. Update `deny.toml` commentary for the renderer path. Regenerate `NOTICE` from the changed dependency metadata if its generator produces a difference.

## Testing and Evidence

- **Automated:** Run `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, and `$env:WIRADESK_SKIP_MANIFEST = '1'; cargo test --workspace --locked`. The shared test asserts `cfg!(target_feature = "crt-static")` on every Windows MSVC test build. PE-scanner tests cover each prohibited CRT family, case-insensitive matching, a clean import list, malformed PE input, and absent required binaries.
- **Release artifacts:** Run `cargo build --workspace --release --locked`, then the scanner against `target\\release`. The scanner must report zero prohibited imports for each named executable; a pre-existing/stale artifact, an absent executable, or a parse failure is a failure rather than a clean result.
- **Dependency and policy:** `cargo tree -p settings` contains no `skia-bindings` or `skia-safe`; `cargo-deny check` passes; the checked-in lockfile and, if changed, `NOTICE` match the resulting graph.
- **Manual clean-environment smoke:** On a clean x64 Windows VM with no Visual Studio or Microsoft Visual C++ Redistributable installed, copy the published loose binary pair, elevate and launch `wiradesk.exe`, then open Settings through its tray menu. Record that both processes start with no `0xC0000135` dialog. In that Settings session, exercise all five panes, shortcut recording, the General-pane hold-delay stepper, and Mouse preset dropdown open/select/dismiss under both light and dark themes. Repeat renderer checks at 100%, 125%, 150%, and 200% DPI, and once in an RDP or VM software-rendered session; record any visual defect as a new defect rather than silently accepting it.

## Out of Scope

- Adding FemtoVG, GPU fallback, a new rendering abstraction, or a Settings layout/style redesign.
- Changing the daemon's native Win32/DWM visual-switcher overlay.
- Changing package-submission workflows or claiming that a package validator has accepted a future submission.
- Bundling, downloading, or installing a Visual C++ Redistributable.

## Further Notes

The owner selected the software-renderer option over retaining Skia with a redistributable fallback because the loose-binary distribution and package validation need the binaries themselves to be self-contained. The clean-VM smoke is complementary to the import gate: it validates process startup and Settings interaction; it does not replace the deterministic PE gate in CI.
