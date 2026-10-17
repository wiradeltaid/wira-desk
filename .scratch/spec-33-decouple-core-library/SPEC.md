---
id: SPEC-33
title: Decouple Platform-Agnostic Core Library and InputSink Protocol
status: closed
size: M
release: "0.4.0"
created: '2026-10-17'
author: Wira Delta Indonesia
---

# SPEC-33 — Decouple Platform-Agnostic Core Library and InputSink Protocol

## Context & Objectives
This specification governs the architectural decoupling of approximately 4,400 lines of pure domain logic out of Windows-specific FFI modules in `crates/daemon` and `crates/shared` into `crates/core` (`wiradesk-core`). It extracts mathematics, geometry layout planners, switcher layout calculations, cycling selection state machines, and shortcut matching into a pure sans-IO engine compiled with `#![forbid(unsafe_code)]` under the MIT license, establishing a platform-agnostic, sans-IO domain engine for headless testing and multi-target compilation.

## Invariants & Guardrails
1. **Zero Regression Guarantee:** Every change MUST keep the exact pre-refactor test baseline of 775 unit tests (458 daemon, 244 settings, 73 shared) 100% green without deletion or modification to expected test outputs, via backward-compatible facades in `crates/daemon` and `crates/shared`:
   `$env:WIRADESK_SKIP_MANIFEST = '1'; cargo test --workspace --no-fail-fast`
2. **Strict Sans-IO Boundary:** `crates/core` MUST NOT perform filesystem, registry, network, or direct OS clock calls, and MUST NOT import `windows-sys` or native window handles (`HWND`).
3. **One-Way Dependency:** `crates/daemon` and `crates/settings` depend on `crates/core`. `crates/core` MUST NOT depend on `daemon`, `settings`, or `shared`.
4. **Clean Update Split:** `update::decide` and `split_https` are extracted to `core`, leaving `windows-sys` WinHttp, `std::env::var`, and network IO in `crates/shared`.
5. **Bounded Input Latency:** `InputSink::on_input` must execute within microsecond bounded time without heap allocation, OS syscalls, or lock contention.
6. **Windows Runtime Parity:** Post-implementation daemon and settings must function identically to the pre-refactor build in all user-facing operations (snapping, cycling, switcher overlay, tray icon, settings dialog).

## Tracer-Bullet Tickets
- **SPEC-33-01:** Initialize `crates/core` (`wiradesk-core`) workspace member, MIT license, and extract pure geometry planners (`arrangement/{mod, snap, thirds, stack, monitor}`) and `commands.rs`.
- **SPEC-33-02:** Extract switcher layout calculations (`switcher/{layout, selection}`) and clean split of descriptor parsing (`update::decide`, `split_https`).
- **SPEC-33-03:** Extract cycling selection state machine (`cycling/{eligibility, selection, mod}`), `WindowFacts`, spatial evaluation, and `BypassPolicy` (`context/mod.rs`).
- **SPEC-33-04:** Decouple hook decisions into `InputSink` protocol (`Disposition`, `Effect`, `ModifierState`), and lift all 8 `PostMessageW(WM_APP_COMMAND_READY)` invocations (6 keyboard + 2 mouse) out of decision paths to outer shell via `wake_worker` signal.
