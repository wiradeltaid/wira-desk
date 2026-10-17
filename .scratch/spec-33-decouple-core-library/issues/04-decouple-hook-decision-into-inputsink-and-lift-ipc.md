---
id: SPEC-33-04
title: Decouple hook decisions into InputSink and lift all IPC dispatching
spec: SPEC-33
status: open
depends_on: [SPEC-33-03]
---

# SPEC-33-04 — Decouple hook decisions into InputSink and lift all IPC dispatching

## Scope
1. In `crates/core/src/input.rs`:
   - Introduce `trait InputSink`, `enum Effect`, `enum Disposition`, `struct ModifierState`, and `enum InputEvent`.
   - Provide bounded microsecond input processing without heap allocation, OS syscalls, or lock contention.
2. In `crates/daemon/src/hook.rs`:
   - Lift all 8 `PostMessageW(WM_APP_COMMAND_READY)` invocations out of internal decision paths (line numbers indicative as of current HEAD):
     - 6 keyboard dispatch sites in `handle_key_event_with_sink` (indicative lines 457, 465, 489, 563, 664, 707).
     - 2 mouse dispatch sites in `handle_mouse_event_with_sink` (indicative lines 1320, 1357).
   - Route worker wake-up requests through `wake_worker` boolean flag in `KeyHandleOutcome` and `MouseHandleOutcome` to outer wrappers (`handle_key_event`, `handle_mouse_event`).
3. Re-export `ModifierState` in `daemon::hook` to maintain 100% test compatibility.

## Verification
- Hook latency benchmark profiles show zero regression compared to `release-metrics` baseline.
- All keyboard and mouse unit tests pass cleanly across `crates/daemon`.
