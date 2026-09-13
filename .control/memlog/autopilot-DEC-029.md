---
artifact: .control/decisions/DEC-029-autopilot-mandate-for-spec-15-delivery.md
skill: wdi-autopilot
date: 2026-09-13
---

# Memlog — autopilot run DEC-029

## Resume

Iteration: 4 (boundary: HEAD)
Run branch: autopilot/DEC-029, PR opened for owner review
Stopped at: Done (all specifications closed: SPEC-15 delivered, promise progress 100%, 72/72 counted RTM rows green)
Blocked: —
Parked: —
Next: —

## Decisions

| When | Where | Decided | Instead of | Cost if wrong | Landed in |
|---|---|---|---|---|---|
| Preflight | Mandate acceptance | Accepted DEC-029 for SPEC-15 delivery per owner confirmation | Stopping for manual gate check-ins | Supersede DEC-029 | .control/decisions/DEC-029-autopilot-mandate-for-spec-15-delivery.md, decisions.yaml |
| Preflight | Build safety | Pinned single target dir in main checkout to prevent race conditions and thrashing | Per-worktree target dir | Workspace crate fingerprints thrash | decisions.yaml |
| Preflight | Peer review | Configured Claude Sonnet 5 shell-out for independent code and doc review | Single-agent self-review | Reviewer independence lost | decisions.yaml |
| Iter 1 | SPEC-15-01 | Inlined \n before bounds in hold delay caption and expanded container padding/min-height, added \n before GitHub disclosure | Awkward single-line wrapping and cramped card bottom spacing | Sub-optimal UI typography clearance | crates/settings/ui/panes/general_pane.slint, crates/settings/ui/panes/about_pane.slint, crates/settings/src/app.rs |
| Iter 2 | SPEC-15-02 | Excluded PopupHost & Xaml_WindowedPopupClass with HelperSurface precedence #3, captured title/extent/owner on WindowFacts without branching | Immediate exclusion of undiagnosed title/extent/owner | False positive exclusion of valid canvas or in-flight windows | crates/daemon/src/cycling/mod.rs, crates/daemon/src/cycling/eligibility.rs, crates/daemon/src/cycling/source.rs, assumptions.md |
| Iter 3 | SPEC-15-04 | Implemented Option A deferred cycle timing on keyup below threshold, SwitcherArm opcodes, repeat latch, and restated SM-1 | Immediate keydown cycle execution causing premature jump | Pre-mature cycle jump occurs prior to overlay open | crates/daemon/src/hook.rs, crates/daemon/src/worker.rs, crates/shared/src/commands.rs, crates/shared/src/config.rs, prd.md, DEC-029.md |
| Iter 4 | SPEC-15-03 | Included active window at card index 0 followed by LRU order, forward initial_index 1 (len>1) / 0 (len==1), backward len-1, armed hold on single-window desktops | Omitting active window and early-exiting on single window | Active window missing from switcher cards and switcher failing to open on single-window desktop | crates/daemon/src/switcher/mod.rs, crates/daemon/src/worker.rs |
| Iter 4 | Test hygiene | Marked live desktop SendInput/arrangement tests #[ignore] per owner direction to avoid disrupting interactive work | Running desktop keystroke tests on every unit test iteration | Unwanted task view/virtual desktop jumps during developer computer usage | crates/daemon/src/worker.rs |



