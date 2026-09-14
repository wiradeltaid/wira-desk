---
artifact: .control/decisions/DEC-035-autopilot-mandate-for-spec-21-delivery.md
skill: wdi-autopilot
date: 2026-09-14
---

# Memlog — autopilot run DEC-035

## Resume

Iteration: 0 (boundary: HEAD)
Run branch: autopilot/DEC-035
Stopped at: —
Blocked: —
Parked: —
Next: SPEC-21-01

## Decisions

| When | Where | Decided | Instead of | Cost if wrong | Landed in |
|---|---|---|---|---|---|
| Preflight | Mandate acceptance | Accepted DEC-035 for SPEC-21 delivery per owner launch | Stopping for manual gate check-ins | Supersede DEC-035 | .control/decisions/DEC-035-autopilot-mandate-for-spec-21-delivery.md, decisions.yaml |
| Preflight | Build safety | Pinned single target dir in main checkout to prevent race conditions and thrashing | Per-worktree target dir | Workspace crate fingerprints thrash | decisions.yaml |
| Preflight | Peer review | Configured Kiro GPT-5.6 Terra shell-out for independent code and doc review | Single-agent self-review | Reviewer independence lost | decisions.yaml |
