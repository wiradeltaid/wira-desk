---
artifact: .control/decisions/DEC-030-autopilot-mandate-for-spec-16-delivery.md
skill: wdi-autopilot
date: 2026-09-13
---

# Memlog — autopilot run DEC-030

## Resume

Iteration: 3 (boundary: HEAD)
Run branch: autopilot/DEC-030, PR not open
Stopped at: SPEC-16-03 delivered, all SPEC-16 tickets complete
Blocked: —
Parked: —
Next: SPEC-16 spec close and cycle-end finish

## Decisions

| When | Where | Decided | Instead of | Cost if wrong | Landed in |
|---|---|---|---|---|---|
| Preflight | Mandate acceptance | Accepted DEC-030 for SPEC-16 delivery per owner confirmation | Stopping for manual gate check-ins | Supersede DEC-030 | .control/decisions/DEC-030-autopilot-mandate-for-spec-16-delivery.md, decisions.yaml |
| Preflight | Build safety | Pinned single target dir in main checkout to prevent race conditions and thrashing | Per-worktree target dir | Workspace crate fingerprints thrash | decisions.yaml |
| Preflight | Peer review | Configured Claude Sonnet 5 shell-out for independent code and doc review | Single-agent self-review | Reviewer independence lost | decisions.yaml |
| Iter 1 | SPEC-16-01 | Balanced hold delay padding to 12px/12px, vertically centered stepper in VerticalLayout, replaced 6 user-visible em dashes with pinned copy | Changing main-axis alignment or rewriting comments/en-dashes | Misaligned layout or breaking unchanged copy and comments | crates/settings/ui/panes/general_pane.slint, crates/settings/ui/panes/about_pane.slint, crates/settings/ui/components/key_check.slint, crates/settings/ui/panes/shortcuts_pane.slint, crates/settings/src/app.rs |
| Iter 2 | SPEC-16-02 | Injected start menu suppression at arm entry and open_visual_switcher entry, removed dead release-edge call in commit, built testable suppression seam | Release-edge injection or SendInput in hook thread | Start Menu opens on hold/commit or focus activation races | crates/daemon/src/worker.rs, crates/daemon/src/hook.rs |
| Iter 3 | SPEC-16-03 | Added CLASS_POPUP_WINDOW_SITE_BRIDGE to HELPER_SURFACE_CLASSES and frozen fixtures, kept title/extent/owner contract untouched | Broad title/extent/owner contract edits | False positive window exclusions and contract drift | crates/daemon/src/cycling/mod.rs, crates/daemon/src/cycling/eligibility.rs |



