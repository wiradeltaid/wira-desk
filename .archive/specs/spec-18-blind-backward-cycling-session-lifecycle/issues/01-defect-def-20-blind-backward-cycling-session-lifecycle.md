# 01: Defect DEF-20 — Blind backward cycling session is reset on every tap by SwitcherArm and SwitcherDisarm

**What to build:** Fix `DEF-20` (`.control/registry/defects.yaml`). During live desktop testing of SPEC-17-01, rapid blind backward cycling (`Shift + Alt + ~` or `Shift + Win + ~`) continues to oscillate between the top two windows (`A <-> B`). While `BackwardCycleSession` functions correctly in isolation, in the actual runtime with `visual_enabled = true` (the default), `crates/daemon/src/worker.rs` calls `reset_backward_cycle_session()` on `Command::SwitcherArm | Command::SwitcherArmPrev` (line 428) and `Command::SwitcherDisarm` (line 462). Because the Option A timing decoupling enqueues `SwitcherArmPrev` on keydown and `SwitcherDisarm` on keyup immediately preceding `CyclePrev`, the session is wiped out on every single tap, causing every backward cycle to restart a fresh session from `Z=1`.

**Blocked by:** None (can start immediately)

**Status:** closed

## Acceptance Criteria

- [x] **Decouple BackwardCycleSession reset from SwitcherArm and SwitcherDisarm**:
      In `crates/daemon/src/worker.rs:428` and `:462`, remove `reset_backward_cycle_session()` from `Command::SwitcherArm | Command::SwitcherArmPrev` and `Command::SwitcherDisarm`. These commands represent the transient hold-delay arming window of a blind tap and MUST NOT wipe out an active backward cycle session.
- [x] **Preserve legitimate session reset boundaries**:
      Ensure `reset_backward_cycle_session()` is strictly bounded to legitimate session termination events:
      1. Forward cycling (`Command::Cycle`, `worker.rs:800`).
      2. Visual switcher overlay commit or cancel (`Command::SwitcherCommit`, `worker.rs:489` and `Command::SwitcherCancel`, `worker.rs:527`). Note: `open_visual_switcher` does not need a reset call because `set_switcher_active(true)` blocks `CyclePrev` commands until commit/cancel.
      3. Focus departure to an untracked window or different application (enforced structurally by `BackwardCycleSession::is_valid()`).
      4. Session timeout exceeding `BACKWARD_CYCLE_SESSION_TIMEOUT_MS = 2000` ms (enforced structurally by `BackwardCycleSession::is_valid()`).
- [x] **End-to-End Worker integration tests with SwitcherArm/Disarm lifecycle**:
      Add worker test `worker::tests::backward_blind_cycle_survives_rapid_switcher_arm_disarm_taps` and `worker::tests::drain_commands_consecutive_backward_taps_traverse_all_windows` asserting that pumping consecutive rapid backward tap sequences through `drain_commands()` (where each tap enqueues `SwitcherArmPrev`, `SwitcherDisarm`, and `CyclePrev` into the ring) against a dynamic self-mutating Z-order with 4 windows visits all windows in reverse order (`2, 3, 4, 1, 2, 3`) without oscillating between two windows.
- [x] **Visual Switcher Invariance**:
      Verify holding `Win + Shift + ~` past the hold threshold still opens the visual switcher overlay with the last card highlighted (`len - 1`) and commits properly on modifier release.
- [x] **Workspace Test Suite**:
      All workspace tests pass green (`WIRADESK_SKIP_MANIFEST=1 cargo test --workspace`).
