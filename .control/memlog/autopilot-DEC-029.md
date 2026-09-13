---
artifact: .control/decisions/DEC-029-autopilot-mandate-for-spec-15-delivery.md
skill: wdi-autopilot
date: 2026-09-13
---

# Memlog — autopilot run DEC-029

## Resume

Iteration: 2 (boundary: HEAD)
Run branch: autopilot/DEC-029, draft PR not yet opened
Stopped at: Capacity / Next ticket (SPEC-15-02 delivered, moving to SPEC-15-04)
Blocked: —
Parked: —
Next: SPEC-15-04 (Cycle activation timing and hold threshold decoupling)

## Decisions

| When | Where | Decided | Instead of | Cost if wrong | Landed in |
|---|---|---|---|---|---|
| Preflight | Mandate acceptance | Accepted DEC-029 for SPEC-15 delivery per owner confirmation | Stopping for manual gate check-ins | Supersede DEC-029 | .control/decisions/DEC-029-autopilot-mandate-for-spec-15-delivery.md, decisions.yaml |
| Preflight | Build safety | Pinned single target dir in main checkout to prevent race conditions and thrashing | Per-worktree target dir | Workspace crate fingerprints thrash | decisions.yaml |
| Preflight | Peer review | Configured Claude Sonnet 5 shell-out for independent code and doc review | Single-agent self-review | Reviewer independence lost | decisions.yaml |
| Iter 1 | SPEC-15-01 | Inlined \n before bounds in hold delay caption and expanded container padding/min-height, added \n before GitHub disclosure | Awkward single-line wrapping and cramped card bottom spacing | Sub-optimal UI typography clearance | crates/settings/ui/panes/general_pane.slint, crates/settings/ui/panes/about_pane.slint, crates/settings/src/app.rs |
| Iter 2 | SPEC-15-02 | Excluded PopupHost & Xaml_WindowedPopupClass with HelperSurface precedence #3, captured title/extent/owner on WindowFacts without branching | Immediate exclusion of undiagnosed title/extent/owner | False positive exclusion of valid canvas or in-flight windows | crates/daemon/src/cycling/mod.rs, crates/daemon/src/cycling/eligibility.rs, crates/daemon/src/cycling/source.rs, assumptions.md |


