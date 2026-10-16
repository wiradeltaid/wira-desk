---
id: SPEC-32-05
title: Decouple shortcut decisions from Win32 hook and wire InputSink
spec: SPEC-32
status: open
depends_on: [SPEC-32-04]
---

# SPEC-32-05 — Decouple shortcut decisions from Win32 hook and wire InputSink

## Scope
1. In `crates/daemon/src/hook.rs`:
   - Identify the 6 `PostMessageW(WM_APP_COMMAND_READY)` invocations inside `handle_key_event_with_sink`.
   - Lift IPC dispatching out of the decision path into the outer hook wrapper.
2. In `crates/core`:
   - Introduce `trait InputSink` and `enum Effect`.
   - Move shortcut matching (`match_shortcut`, `ModifierState`, `throttle_allows`) and mouse event resolution into `core`.
3. Ensure `InputSink::on_input` executes strictly within microsecond bounded time (no allocation, no I/O, no lock contention).

## Verification
- Hook latency benchmark profiles show zero regression compared to `release-metrics` baseline.
- Keyboard and mouse unit tests pass cleanly.
