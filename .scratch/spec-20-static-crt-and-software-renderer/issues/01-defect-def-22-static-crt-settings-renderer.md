---
id: SPEC-20-01
component: settings
satisfies: []
blocked_by: []
status: ready-for-agent
tests:
  - shared::tests::msvc_target_compiles_with_crt_static
  - binary::tests::pe_import_scanner_detects_all_dynamic_msvc_crt_families
  - binary::tests::pe_import_scanner_rejects_malformed_pe_input
  - binary::tests::pe_import_scanner_passes_clean_static_binary
---

# 01: Defect DEF-22 — Static CRT linking via Slint software renderer migration and installer cleanup

**What to build:** Fix `DEF-22` without retaining an installer-based Visual C++ Redistributable fallback.

1. In `crates/settings/Cargo.toml`, replace Slint's `renderer-skia` feature with `renderer-software`; retain `backend-winit`, `accessibility`, and `compat-1-2`; and set `default-features = false` on direct `i-slint-backend-winit`. Update `Cargo.lock`. The normal settings dependency graph must not contain `skia-bindings` or `skia-safe`.
2. Create `.cargo/config.toml` with `rustflags = ["-C", "target-feature=+crt-static"]` only under `[target.x86_64-pc-windows-msvc]`.
3. Delete the `vc_redist.x64.exe` `[Files]` and `[Run]` branches and their fallback commentary from `packaging/wiradesk.iss`.
4. Make `scripts/verify-release-binary.ps1` strict. Remove `-AllowDynamicCrtIfBundled` from its parameter list and implementation; it must not read installer text. Given a release-artifact directory, it must require and parse exactly `wiradesk.exe` and `wiradesk-settings.exe`, failing for a missing file, malformed PE, or any case-insensitive import matching `VCRUNTIME*.dll`, `MSVCP*.dll`, `UCRTBASE.dll`, or `api-ms-win-crt-*.dll`. Keep its dynamic-import definition aligned with `crates/shared/src/binary.rs`.
5. Remove the bypass flag from `.github/workflows/release.yml` and add the strict scanner to `.github/workflows/ci.yml`, immediately after each locked release build and before installer packaging.
6. Replace the fallback-shaped test in `crates/shared/src/lib.rs` with an unconditional `assert!(cfg!(target_feature = "crt-static"))` under its Windows MSVC compile guard. Extend shared PE-scanner tests for all prohibited import families, case-insensitive matching, clean input, and malformed input.
7. Update `AD-11a` in `.how/settings/SDD-settings.md`, the accepted `.constitution/project/codebase-stack-guide.md`, and `deny.toml` commentary for the software-renderer/static-CRT design. Regenerate `NOTICE` from dependency metadata if it changes.

**Blocked by:** none

**Status:** ready-for-agent

## Acceptance Criteria

- [ ] **Software renderer graph:** `crates/settings/Cargo.toml` declares `slint` with `default-features = false` and exactly `backend-winit`, `renderer-software`, `accessibility`, and `compat-1-2`; direct `i-slint-backend-winit` also disables defaults. `cargo tree -p settings` reports neither `skia-bindings` nor `skia-safe`. `Cargo.lock` is updated and `cargo build --workspace --release --locked` succeeds.
- [ ] **Target-only static CRT:** `.cargo/config.toml` contains `[target.x86_64-pc-windows-msvc]` with `rustflags = ["-C", "target-feature=+crt-static"]`; no broad `[build] rustflags` is introduced. On Windows MSVC, `shared::tests::msvc_target_compiles_with_crt_static` contains one direct assertion of `cfg!(target_feature = "crt-static")`, with no fallback branch.
- [ ] **Strict binary verification:** `scripts/verify-release-binary.ps1 -Path target\\release` requires the two named release executables, rejects malformed/non-PE input, and reports zero prohibited CRT imports for each. It detects `VCRUNTIME*.dll`, `MSVCP*.dll`, `UCRTBASE.dll`, and `api-ms-win-crt-*.dll` case-insensitively. The script defines no `AllowDynamicCrtIfBundled` parameter or code path and does not accept a text declaration as evidence of runtime availability.
- [ ] **Scanner test coverage:** shared tests prove the scanner finds each prohibited import family, including mixed-case names; preserves the clean-static fixture; and returns an error for malformed PE input instead of treating it as a binary with zero imports.
- [ ] **Installer cleanup:** `packaging/wiradesk.iss` has zero executable `[Files]` or `[Run]` entries, conditions, or fallback comments for `vc_redist.x64.exe`/`VCRedist`; it packages only the existing application payload and assets.
- [ ] **CI and release parity:** `ci.yml` and `release.yml` invoke the strict scanner after `cargo build --workspace --release --locked` and before any installer compilation. Neither workflow, nor the script signature, contains `AllowDynamicCrtIfBundled`.
- [ ] **Documentation and policy:** `AD-11a` and the accepted stack guide identify `renderer-software` and the target-scoped static CRT policy; `deny.toml` describes the actual renderer path; `cargo-deny check` is clean; and `NOTICE` is regenerated if `cargo metadata` changes it.
- [ ] **Clean-environment smoke:** On a clean x64 Windows VM with no Visual Studio or VC++ Redistributable installed, launch the published loose binary pair by elevating the daemon and opening Settings from the tray. Both must start without `STATUS_DLL_NOT_FOUND (0xC0000135)`. Exercise all five Settings panes, shortcut recording, the hold-delay stepper, and Mouse preset dropdown in light and dark themes; repeat the visual check at 100%, 125%, 150%, and 200% DPI and once in an RDP/VM software-rendered session. Record the result in the spec folder; any fidelity failure is a new defect.
- [ ] **Workspace Verification Suite:** `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, and `$env:WIRADESK_SKIP_MANIFEST = '1'; cargo test --workspace --locked` pass with zero failures.
