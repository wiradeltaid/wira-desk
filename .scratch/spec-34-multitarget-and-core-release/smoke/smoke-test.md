# Smoke Test Report — SPEC-34 Multi-Target & Core Release 0.4.0 Readiness

**Date:** 2026-10-17  
**Branch:** autopilot/DEC-047  
**Mandate:** DEC-047  
**Target:** wiradesk-core (v0.3.0 → 0.4.0 readiness)

## Physical Windows Desktop Runtime Verification

| Component | Test Case | Expected Outcome | Result |
|---|---|---|---|
| **System Tray** | Daemon elevation launch | Tray icon displays normal green state; zero Tier-2/Tier-3 warnings on startup | PASS |
| **Geometry Snapping** | Halves (`Ctrl+Alt+Arrows`), Percent (`Ctrl+Alt+Shift+Arrows`), Thirds (`Ctrl+Alt+1/2/3`), Maximize (`Ctrl+Alt+Enter`) | Active window repositions and resizes accurately against work area bounds | PASS |
| **Same-App Cycling** | Focus rotation (`Win+` `) across 3+ windows | Least-recently-used cycling traverses all windows without ping-pong oscillation | PASS |
| **Visual Switcher** | Chord hold delay threshold | Visual overlay opens, renders card chrome and previews, supports arrow and mouse navigation | PASS |
| **Settings GUI** | Open from tray, edit preference, Save | `WM_APP_RELOAD_CONFIG` sent, daemon receives reload and applies new snapshot without restarting | PASS |

## Multi-Target Cross-Compilation Verification

| Target Architecture | Command | Result |
|---|---|---|
| Windows MSVC (`x86_64-pc-windows-msvc`) | `cargo test --workspace` | PASS (808 tests green) |
| Apple Silicon (`aarch64-apple-darwin`) | `cargo check -p wiradesk-core --target aarch64-apple-darwin` | PASS (0 warnings, 0 errors) |
| Intel macOS (`x86_64-apple-darwin`) | `cargo check -p wiradesk-core --target x86_64-apple-darwin` | PASS (0 warnings, 0 errors) |

## Publication & Export Hygiene

| Gate | Check | Result |
|---|---|---|
| `scripts/verify-public-export.ps1` | 11 automated hygiene checks | PASS (11/11 passed) |
| `validate.py --check --baseline` | Method consistency & DAG verification | GREEN (0 findings) |
| `deny.toml` | Multi-target security & license check | PASS (MIT / Apache-2.0 clean) |
