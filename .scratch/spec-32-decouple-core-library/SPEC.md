---
id: SPEC-32
title: Architectural Decoupling — Extract Platform-Agnostic Core Library
status: open
size: L
release: "0.4.0"
created: '2026-10-16'
author: Wira Delta Indonesia
---

# SPEC-32 — Architectural Decoupling: Extract Platform-Agnostic Core Library

## Context & Objectives
This specification governs the architectural decoupling of Wira Desk from a 3-crate Windows-coupled workspace (`daemon`, `settings`, `shared`) to a 4-crate modular architecture introducing `crates/core` (`wiradesk-core`).

`crates/core` serves as the pure domain engine containing mathematics, geometry planners, cycling state machines, and configuration parsing, strictly compiled with `#![forbid(unsafe_code)]` and zero I/O. This decoupling enables headless testing, accelerates CI, cleanly isolates Win32 FFI into adapter boundaries, and establishes a reusable platform-agnostic library licensed under MIT.

## Invariants & Guardrails
1. **Zero Regression Guarantee:** Every change MUST keep the existing 591+ Windows unit test suite 100% green:
   `$env:WIRADESK_SKIP_MANIFEST = '1'; cargo test --workspace --no-fail-fast`
2. **Strict Sans-IO Boundary:** `crates/core` MUST NOT perform filesystem, registry, network, or direct OS clock calls, and MUST NOT import `windows-sys` or native window handles (`HWND`).
3. **Dual-Target Verification:** `crates/core` MUST cross-compile cleanly for Apple Silicon:
   `cargo check -p wiradesk-core --target aarch64-apple-darwin`
4. **License Boundary:** `crates/core` is licensed under MIT. `daemon` and `settings` remain `GPL-3.0-only`.
5. **DCO & Inbound Licensing:** `CONTRIBUTING.md` is updated with Developer Certificate of Origin 1.1 (`Signed-off-by`).

## Tracer-Bullet Tickets
- **SPEC-32-01 (WDK-CORE-01):** Governance & AD-12 amendment: Draft `DEC-046`, register `core` in `components.yaml`, update `codebase-stack-guide.md`, and add DCO 1.1 to `CONTRIBUTING.md`.
- **SPEC-32-02 (WDK-CORE-02):** Publication hygiene gate: Add commercial keyword and private repo prevention check in `scripts/verify-public-export.ps1`.
- **SPEC-32-03 (WDK-CORE-03):** Initialize `crates/core` and extract geometry planners (`arrangement/{mod, snap, thirds, stack, monitor}`) and `commands.rs`.
- **SPEC-32-04 (WDK-CORE-04):** Extract cycling selection state machine, facts evaluation, and `context/mod.rs` (BypassPolicy).
- **SPEC-32-05 (WDK-CORE-05):** Decouple hook decisions: lift `PostMessageW` calls to outer shell, extract shortcut matching and mouse resolver into `core::InputSink`.
- **SPEC-32-06 (WDK-CORE-06):** Multi-target verification (`aarch64-apple-darwin`), `deny.toml` update, full physical Windows regression suite pass, and minor bump declaration to `0.4.0`.
