# Peer Review Request: SPEC-19 & DEF-21

## Standing Mandate
If this draft touches the architecture spine, an SRS, an SDD, or a SPEC, you're authorized to run `wdi-review` on it yourself and edit the document directly to apply its stamp — no need to ask first, that permission is already given for this dispatch.
(Note: Shared worktree in main checkout is in use; do NOT create a new worktree and do NOT run concurrent cargo build/run).

---

## 1. Drafted Spec and Ticket Paths
- Spec document: `.scratch/spec-19-visual-switcher-mouse-selection-and-hold-threshold/SPEC.md`
- Issue ticket: `.scratch/spec-19-visual-switcher-mouse-selection-and-hold-threshold/issues/01-defect-def-21-visual-switcher-mouse-selection-and-hold-threshold.md`
- Spec registry: `.control/registry/specs.yaml` (entry `SPEC-19`)
- Defect registry: `.control/registry/defects.yaml` (entry `DEF-21`)

---

## 2. Original Raw Notes (Verbatim)

```
Catatan:
1. Yang jadi masalah adalah kalau Ketika overlay saya klik pakai cursor mouse untuk window yang saya pilih, dia tidak konsisten memunculkan / memilih window tersebut sebagai aktif focus. Kalau via overlay lalu navigasi pakai keyboard  (apakah pakai hotkey lagi atau arrow) itu berjalan dengan baik
2. Default hold delay threshold buat aja 300ms

Window Management & DEF-20 (SPEC-18-01)
  1. Blind Backward Cycling Multi-Window Traversal
     [x] Open 3 or more windows of the same application (e.g. 4 Notepad windows: A, B, C, D) on the same monitor with Window A in the foreground.
     [x] Rapidly tap Shift + Alt + ~ (or Shift + Win + ~) consecutively (< 2000 ms between taps).
     [x] Verify focus visits windows in reverse circular sequence (A -> D -> C -> B -> A) across consecutive taps rather than oscillating back and forth between A and B.

  2. Session Reset Boundaries
     [x] Perform a backward cycle to an older window in the stack, then tap forward cycle (Alt + ~ or Win + ~).
     [x] Verify forward cycling resets the backward cycle session and advances forward in the dynamic Z-order.
     [x] Perform a backward cycle, pause without input for longer than 2000 ms, then tap backward cycle again.
     [x] Verify session times out after 2000 ms of inactivity and starts a fresh backward cycle from the current foreground window.
     [x] Perform a backward cycle, click an untracked window/app on another monitor, and return to cycle backward.
     [x] Verify external focus departure cleanly resets the backward session.

  3. Visual Switcher Invariance
     [x] Press and hold Win + Shift + ~ (or Alt + Shift + ~) past the hold delay threshold (> 150 ms).
     [x] Verify the visual switcher overlay opens with the last card highlighted (len - 1) and zero premature focus jumps on keydown.
     [x] Release the modifier keys while the backward target is selected.
     [x] Verify focus commits to the selected target window and cleanly resets the cycle session.
```

---

## 3. Review Instructions
Please review the drafted `SPEC-19` and ticket `SPEC-19-01` against the codebase, verifying:
1. Does the problem statement and root cause analysis accurately reflect why clicking with the mouse on the visual switcher overlay fails to activate the clicked window (specifically: `SwitcherController.selected_index` remaining independent and un-synced from `SwitcherOverlay.selected_index` during `WM_MOUSEMOVE` / `WM_LBUTTONDOWN`, so `candidates_from_selection()` yields the initial keyboard index)?
2. Are the proposed changes to default hold delay (from 150 ms to 300 ms) fully and correctly specified across `crates/shared/src/config.rs`, Slint UI, and tests?
3. Are there any edge cases with mouse click handling, modifier release races, or pagination that the acceptance criteria should address?
4. Run review (lenses: structure, prose, edge-case-hunter) and apply the `spec_reviewed` trace stamp to `SPEC-19` in `.control/registry/specs.yaml` (date: 2026-09-14, sha: `385007333d4743541ef815877be50f87cfc1b3d3` or current commit).
5. Output your review verdict, edge-case findings, and any requested adjustments.
