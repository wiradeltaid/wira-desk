---
artifact: .control/decisions/DEC-033-autopilot-mandate-for-spec-19-delivery.md
skill: wdi-autopilot
date: 2026-09-14
---

# Memlog — autopilot run DEC-033

## Resume

Iteration: 1 (boundary: HEAD)
Run branch: autopilot/DEC-033, PR opened for owner review
Stopped at: Done (all specifications closed: SPEC-19 delivered, promise progress 100%, 84/84 counted RTM rows green)
Blocked: —
Parked: —
Next: —

## Decisions

| When | Where | Decided | Instead of | Cost if wrong | Landed in |
|---|---|---|---|---|---|
| Preflight | Mandate acceptance | Accepted DEC-033 for SPEC-19 delivery per owner confirmation | Stopping for manual gate check-ins | Supersede DEC-033 | .control/decisions/DEC-033-autopilot-mandate-for-spec-19-delivery.md, decisions.yaml |
| Preflight | Build safety | Pinned single target dir in main checkout to prevent race conditions and thrashing | Per-worktree target dir | Workspace crate fingerprints thrash | decisions.yaml |
| Preflight | Peer review | Configured Claude Sonnet 5 shell-out for independent code and doc review | Single-agent self-review | Reviewer independence lost | decisions.yaml |
| Iter 1 | SPEC-19-01 | Delegated SwitcherController selection and candidates_from_selection to SwitcherOverlay | Retaining duplicate desynchronized selected_index state | Mouse click/hover fails to activate target window | crates/daemon/src/switcher/mod.rs, crates/daemon/src/switcher/overlay.rs |
| Iter 1 | SPEC-19-01 | Raised default visual switcher hold delay threshold from 150 ms to 300 ms | Keeping aggressive 150 ms default | Rapid blind cycling inadvertently triggers visual overlay | crates/shared/src/config.rs, crates/settings/ui/panes/general_pane.slint, crates/settings/ui/main_window.slint |
| Iter 1 | SPEC-19-01 | Removed dead write-only selected_index field from SwitcherController following Sonnet 5 peer review | Leaving dead field in struct | Future maintainers tempted to reintroduce desync | crates/daemon/src/switcher/mod.rs |
