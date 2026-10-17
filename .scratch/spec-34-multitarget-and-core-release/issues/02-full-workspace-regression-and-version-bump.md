---
id: SPEC-34-02
title: Execute full workspace regression pass and declare minor bump to 0.4.0
spec: SPEC-34
status: closed
depends_on: [SPEC-34-01]
---

# SPEC-34-02 — Execute full workspace regression pass and declare minor bump to 0.4.0

## Scope
1. Execute full local Windows test harness:
   ```powershell
   cargo fmt --all --check
   cargo clippy --workspace --all-targets -- -D warnings
   $env:WIRADESK_SKIP_MANIFEST = '1'; cargo test --workspace --no-fail-fast
   ```
2. Run publication hygiene gate:
   ```powershell
   powershell -ExecutionPolicy Bypass -File .\scripts\verify-public-export.ps1 -Path . -SkipHistory
   ```
3. Execute manual/scripted smoke test on physical Windows desktop:
   - Launch `wiradesk.exe` and confirm system tray icon appears without Tier-2/3 warnings.
   - Trigger keyboard snap actions (halves, thirds, maximize) and confirm window repositioning.
   - Trigger same-app cycling (`Win+` `) across 3+ windows and verify focus rotation.
   - Trigger visual switcher hold overlay and verify card selection navigation.
   - Launch Settings GUI from tray menu, modify preference, save, and confirm daemon reloads without error.
4. Declare minor version bump to `0.4.0` under `CHANGELOG.md ## [Unreleased]` for owner authorization.
5. Verify local dependency readiness for downstream library consumers via local Cargo path patch.

## Verification
- All 775+ unit tests across all workspace crates pass 100% green without regression.
- Publication hygiene gate passes all checks cleanly.
- Runtime smoke test checklist passes on Windows physical desktop.
- `CHANGELOG.md` accurately documents the Four-Crate modular architecture under `[Unreleased]`.
