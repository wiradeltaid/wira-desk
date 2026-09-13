# Second Opinion Review Packet — SPEC-17 / DEF-19 (Blind Backward Cycling Multi-Window Ping-Pong Defect)

## 1. Path to the Drafted Spec / Ticket
- Specification Registry: `.control/registry/specs.yaml` (`SPEC-17`)
- Defect Registry: `.control/registry/defects.yaml` (`DEF-19`)
- Ticket File: `.scratch/spec-17-blind-backward-cycling-multi-window/issues/01-defect-def-19-blind-backward-cycling-multi-window.md`

## 2. Original Raw Notes from Owner (Verbatim)
```
Catatan:
1. Untuk fitur cycle (bukan switcher) Alt+` atau Win+` begitu saya kasih Shift, dia cuma bisa mundur back and forth di 2 window saja. Harusnya dia cycle backward toh ke semuanya?

Settings UI / General Pane & About Pane (SPEC-16-01)
  1. Stepper Centering and Spacing Polish
     [x] Verify the Hold Delay Threshold row spacing is balanced with 12px top and 12px bottom padding
     [x] Verify the stepper cluster (`−`, value display, `+`) is vertically centered against the row text column while staying pinned to the right edge
  2. UI Em-Dash Removal
     [x] Verify in General pane that `(100–500 ms)` en dash (U+2013) is intact and readable
     [x] Navigate to About pane and verify in-process disclosure reads: "No telemetry, no account, no separate background service.\nUpdate checks run entirely in-process against GitHub Releases, which you can switch off." (period instead of em dash)
     [x] Open Shortcuts pane and verify Tip reads: "...Wira Desk ships on Ctrl + Alt + ..., a good family for custom actions too."
     [x] Check Key Check panel (press unregistered shortcut or view idle state):
         - Idle/unrunning state reads: "Wira Desk isn't running. Only this window is being checked."
         - Empty keycap readout displays "None yet" instead of "—"
         - Diagnostic notes contain period or comma instead of em dashes ("...still receives it before that app. This shortcut will work." / "...taking it first, often GPU software...")

Visual Switcher / Lifecycle & Start Menu Suppression (SPEC-16-02)
  1. Start Menu Suppression on Commit
     [x] Press and hold the cycle chord (`Win + ~`) past the threshold until the visual switcher overlay appears
     [x] Release the `Win` key to commit the selection
     [x] Verify the highlighted window is activated and the Windows Start Menu does NOT open
     [x] Repeat commit 10 consecutive times and verify focus moves to the highlighted window every time without the Start Menu opening
  2. Escape Cancel & Alt+~ Parity
     [x] Hold `Win + ~` to open the visual switcher overlay, then press Escape to cancel; verify overlay dismisses cleanly and Start Menu does not open
     [x] Press and hold `Alt + ~` (fallback cycle chord) past threshold; verify overlay appears and releasing Alt commits smoothly without glitches

Window Management / Cycling & WinUI 3 Helper Sanitization (SPEC-16-03)
  1. Modern WinUI 3 Popup Bridge Exclusion
     [x] Launch modern Windows 11 Notepad (`Notepad.exe`) with one or more document windows open
     [x] Hold `Win + ~` to trigger the visual switcher overlay for Notepad
     [x] Verify only real Notepad document windows appear as candidate cards in the switcher
     [x] Verify internal WinUI 3 popup bridge surfaces (`PopupWindowSiteBridge` / `Pop-upHost`) do NOT appear as ghost cards
```

## 3. Standing Mandate
"If this draft touches the architecture spine, an SRS, an SDD, or a SPEC, you're authorized to run `wdi-review` on it yourself and edit the document directly to apply its stamp — no need to ask first, that permission is already given for this dispatch."

## 4. Problem Analysis & Review Request
Review the drafted ticket `.scratch/spec-17-blind-backward-cycling-multi-window/issues/01-defect-def-19-blind-backward-cycling-multi-window.md` and registry entries (`SPEC-17` in `specs.yaml`, `DEF-19` in `defects.yaml`).
- Does the diagnosis of why blind backward cycling oscillates between two windows make mechanical sense against live Win32 Z-order behavior?
- In `crates/daemon/src/cycling/mod.rs:255`:
  ```rust
  pub fn cycle_order_directed(
      candidates: &[Candidate],
      active: &ActiveContext,
      direction: Direction,
  ) -> Vec<WindowId> {
      let ordered: Vec<WindowId> = candidates.iter().map(|c| c.facts.window).collect();
      let mut rotated: Vec<WindowId> = match ordered.iter().position(|w| *w == active.foreground) {
          Some(pos) => ordered[pos + 1..]
              .iter()
              .chain(ordered[..pos].iter())
              .copied()
              .collect(),
          None => ordered,
      };
      if direction == Direction::Forward {
          rotated.reverse();
      }
      rotated
  }
  ```
  In forward cycling, `rotated.reverse()` activates least-recently used (LRU) (`Z=last`), which brings the oldest window to `Z=0`. Consecutive forward cycles naturally rotate through all windows in order!
  In backward cycling (`Direction::Backward`), `rotated.reverse()` is skipped, meaning the first target is `rotated[0]` = `Z=1` (the window just beneath the active window). When `Z=1` is activated, it becomes `Z=0` and the previous foreground becomes `Z=1`. Tapping backward again picks `Z=1` again — creating an oscillating 2-window loop `A <-> B` forever!
- Examine the architectural fix direction: how should backward cycling traverse across 3+ windows?
  Option 1: Does stateless backward traversal have a mathematical formulation in live Z-order?
  Option 2: Or does backward cycling require a cycling session (tracking origin or sequence index while the modifier chord remains held), or does the candidate ordering need a specific structure?
- Check whether `SPEC-17-01` needs any adjustments to its scope, acceptance criteria, or architectural boundaries. Edit the files directly if improvements are needed, and provide your feedback.
