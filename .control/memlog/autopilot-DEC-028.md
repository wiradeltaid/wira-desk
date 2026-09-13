---
artifact: .control/decisions/DEC-028-autopilot-mandate-for-spec-14-delivery.md
skill: wdi-autopilot
date: 2026-09-13
---

# Memlog — autopilot run DEC-028

## Resume

Iteration: 2 (boundary: HEAD)
Run branch: autopilot/DEC-028, PR not opened yet
Stopped at: Capacity (completed SPEC-14-03 and SPEC-14-05; next iteration continues with SPEC-14-04 and SPEC-14-06)
Blocked: —
Parked: —
Next: SPEC-14-04 (visual switcher Shift-held backward cycling)

## Decisions

| When | Where | Decided | Instead of | Cost if wrong | Landed in |
|---|---|---|---|---|---|
| Preflight | Mandate acceptance | Accepted DEC-028 for SPEC-14 delivery per owner instruction | Stopping for manual gate check-ins | Supersede DEC-028 | .control/decisions/DEC-028-autopilot-mandate-for-spec-14-delivery.md, decisions.yaml |
| Preflight | Build safety | Pinned single target dir in main checkout to prevent race conditions and thrashing | Per-worktree target dir | Workspace crate fingerprints thrash | decisions.yaml |
| Preflight | Peer review | Configured Claude Sonnet 5 shell-out for independent code and doc review | Single-agent self-review | Reviewer independence lost | decisions.yaml |
| Iter 1 | SPEC-14-01 | Inlined publisher link into attribution line, removed standalone button row, structured Card 3 with padding 0px for full-bleed dividers | Separate button row and unpadded dividers | Stale UI hierarchy and clipped dividers | crates/settings/ui/panes/about_pane.slint, crates/settings/src/app.rs |
| Iter 1 | SPEC-14-02 | Added wrap: word-wrap to hold delay caption and description in GeneralPane, preventing horizontal scrollbar emergence | Unwrapped single-line Text | Window horizontal scrollbar visible when visual switcher enabled | crates/settings/ui/panes/general_pane.slint, crates/settings/src/app.rs |
| Iter 2 | SPEC-14-03 | Parameterized evaluate_spatial with SpatialScope to collect all physical monitors on visual switcher while keeping blind cycle monitor-locked | Single global spatial policy | Breaks Spatial Preservation Invariant or keeps switcher single-monitor | crates/daemon/src/context/mod.rs, crates/daemon/src/worker.rs, crates/daemon/src/switcher/mod.rs |
| Iter 2 | SPEC-14-05 | Extended WorkerSnapshot with visual_enabled and visual_hold_delay_ms, gating timer arming and in-flight cancellation on reload | Unchecked Worker timer arming | Disabling switcher in UI does not stop overlay from opening | crates/daemon/src/config.rs, crates/daemon/src/worker.rs, crates/daemon/src/hook.rs |
