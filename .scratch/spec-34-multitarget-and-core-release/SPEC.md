---
id: SPEC-34
title: Multi-Target Verification and Core Release 0.4.0 Readiness
status: closed
size: S
release: "0.4.0"
created: '2026-10-17'
author: Wira Delta Indonesia
---

# SPEC-34 — Multi-Target Verification and Core Release 0.4.0 Readiness

## Context & Objectives
This specification closes Phase 1 by enforcing multi-target cross-compilation verification on `wiradesk-core` for Apple Silicon (`aarch64-apple-darwin`), updating supply-chain dependency auditing in `deny.toml`, passing the complete physical Windows regression test suite, and declaring the minor version bump to `0.4.0` in `CHANGELOG.md` to verify that `wiradesk-core` is cleanly decoupled from Windows OS APIs and ready for multi-target downstream library consumption.

## Invariants & Guardrails
1. **Apple Silicon Compilation:** `cargo check -p wiradesk-core --target aarch64-apple-darwin` MUST compile clean with 0 warnings and 0 errors.
2. **License Isolation:** `deny.toml` must enforce multi-target checks and guarantee zero copyleft dependencies on `crates/core`.
3. **Full Workspace Pass:** All 775+ unit tests across the 4-crate workspace must pass without regression (zero test deletion, zero modification to expected assertions).
4. **Physical Windows Smoke Test:** A manual/scripted smoke test must verify runtime parity: tray icon launch, snapping execution, same-app cycling, visual switcher overlay, and settings dialog.
5. **Versioning Authority:** Declare minor version bump to `0.4.0` under `CHANGELOG.md ## [Unreleased]` for owner authorization. Tagging and release remain strictly owner actions.

## Tracer-Bullet Tickets
- **SPEC-34-01:** Update `deny.toml` to multi-target (`x86_64-pc-windows-msvc` and `aarch64-apple-darwin`) and verify Apple Silicon compilation.
- **SPEC-34-02:** Execute full physical Windows regression harness, execute runtime smoke test checklist, verify publication export hygiene, and declare version bump to `0.4.0`.
