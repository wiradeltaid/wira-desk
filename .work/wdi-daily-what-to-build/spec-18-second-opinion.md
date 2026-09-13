# Second Opinion Review Packet — SPEC-18 / DEF-20 (Blind Backward Cycling Session Reset on Arm/Disarm)

## 1. Path to the Drafted Spec / Ticket
- Specification Registry: `.control/registry/specs.yaml` (`SPEC-18`)
- Defect Registry: `.control/registry/defects.yaml` (`DEF-20`)
- Ticket File: `.scratch/spec-18-blind-backward-cycling-session-lifecycle/issues/01-defect-def-20-blind-backward-cycling-session-lifecycle.md`

## 2. Original Raw Notes from Owner (Verbatim)
```
Window Management / Blind Backward Cycling (SPEC-17-01 / DEF-19)
1. Multi-Window Blind Backward Traversal
   [ ] Rapidly tap Shift + Alt + ~ (or Shift + Win + ~) multiple times in succession -> masih tetap dia back and forth hanya di 2 window saja
   [ ] Verify focus traverses backward through all eligible windows (A -> D -> C -> B -> A -> D ...) rather than ping-ponging back and forth between only the top two windows -> masih tetap dia back and forth hanya di 2 window saja

2. Session Timeout & Reset -> saya gak paham, dan gak bisa di test gara2 masalah di atas  -> masih tetap dia back and forth hanya di 2 window saja
   [ ] Tap Shift + Alt + ~ once to switch backward from window 1 to window 2
   [ ] Pause for more than 2 seconds (exceeding the 2000 ms session timeout)
   [ ] Tap Shift + Alt + ~ again and verify navigation treats window 2 as the new origin and cycles to window 3
   [ ] Tap Alt + ~ (forward cycle) to verify forward cycling resets the backward session cleanly

3. Visual Switcher Invariance
   [x] Press and hold Win + Shift + ~ (or Alt + Shift + ~) past the hold threshold until the visual switcher overlay appears
   [x] Verify the overlay highlights the last card (len - 1) and commits properly on modifier release without glitches or Start Menu activation
```

## 3. Standing Mandate
"If this draft touches the architecture spine, an SRS, an SDD, or a SPEC, you're authorized to run `wdi-review` on it yourself and edit the document directly to apply its stamp — no need to ask first, that permission is already given for this dispatch."

## 4. Problem Analysis & Review Request
Review the drafted ticket `.scratch/spec-18-blind-backward-cycling-session-lifecycle/issues/01-defect-def-20-blind-backward-cycling-session-lifecycle.md` and registry entries (`SPEC-18` in `specs.yaml`, `DEF-20` in `defects.yaml`).

### Root Cause Analysis:
During the live desktop verification of SPEC-17-01, the owner confirmed that item 3 (visual switcher hold/commit) works cleanly, but rapid blind backward cycling (items 1 and 2) still ping-pongs back and forth between only two windows (`A <-> B`).

Investigation reveals the exact mechanical fault in `crates/daemon/src/worker.rs`:
1. In `worker.rs:428` and `:462`:
   ```rust
   Command::SwitcherArm | Command::SwitcherArmPrev => {
       reset_backward_cycle_session(); // Line 428!
       suppress_start_menu();
       ...
   }
   ...
   Command::SwitcherDisarm => {
       reset_backward_cycle_session(); // Line 462!
       crate::hook::set_switcher_active(false);
       ...
   }
   ```
2. When the visual switcher is enabled (`visual_enabled = true`, the shipped default), the Option A timing decoupling (SPEC-15-04) enqueues `SwitcherArmPrev` on chord keydown.
3. When the user rapidly releases `~` before the hold deadline, the hook enqueues `SwitcherDisarm` immediately followed by `CyclePrev`.
4. When `drain_commands()` executes these queued commands in order:
   - `SwitcherArmPrev` runs -> calls `reset_backward_cycle_session()`.
   - `SwitcherDisarm` runs -> calls `reset_backward_cycle_session()`.
   - `CyclePrev` runs -> `BACKWARD_CYCLE_SESSION` has just been wiped clean!
5. Because the session was wiped twice during the keypress before `CyclePrev` even ran, `BackwardCycleSession::cycle_order` sees `self.windows.is_empty()`, treats every tap as a brand-new session, and picks `Z=1` (the window immediately following active in live Z-order).
6. Target `Z=1` is activated and raised to `Z=0`, pushing the previous foreground to `Z=1`.
7. On the very next tap, the exact same wipe occurs, picking `Z=1` (the previous foreground) again, locking the user into an infinite two-window ping-pong oscillation!
8. Why did unit tests pass? Because the unit tests called `run_context_safe_cycle` directly in a loop, never pumping `SwitcherArmPrev` and `SwitcherDisarm` through the worker event loop.

### Review Tasks:
1. Verify whether this root cause diagnosis explains the observed failure on live desktop with complete fidelity.
2. Confirm the exact lifecycle boundaries for `BackwardCycleSession` reset:
   - Must NOT reset on `SwitcherArm`, `SwitcherArmPrev`, or `SwitcherDisarm` (transient blind cycling states).
   - MUST reset on: forward cycle (`Command::Cycle`), actual visual switcher overlay opening (`open_visual_switcher`), overlay commit (`SwitcherCommit`), focus departure to another app/untracked window, or session timeout (2000 ms).
3. Check the drafted ticket scope, acceptance criteria, and proposed tests in `.scratch/spec-18-blind-backward-cycling-session-lifecycle/issues/01-defect-def-20-blind-backward-cycling-session-lifecycle.md`.
4. Apply the `spec_reviewed` stamp to `SPEC-18` in `.control/registry/specs.yaml` using date `2026-09-13` and lenses `[edge-case-hunter]` per the standing mandate.
