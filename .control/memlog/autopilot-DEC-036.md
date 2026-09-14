---
artifact: .control/decisions/DEC-036-autopilot-mandate-for-spec-22-delivery.md
skill: wdi-autopilot
date: 2026-09-14
---

# Memlog — autopilot run DEC-036

## Resume

Iteration: 0 (boundary: HEAD)
Run branch: autopilot/DEC-036, PR not open yet
Stopped at: Preflight
Blocked: —
Parked: —
Next: SPEC-22-01 (packaging/wiradesk.iss dialog text formatting and clean config clarity)

## Decisions

| When | Where | Decided | Instead of | Cost if wrong | Landed in |
|---|---|---|---|---|---|
| Preflight | Mandate acceptance | Accepted DEC-036 for SPEC-22 delivery per owner launch | Stopping for manual gate check-ins | Supersede DEC-036 | .control/decisions/DEC-036-autopilot-mandate-for-spec-22-delivery.md, decisions.yaml |
| Preflight | Build safety | Pinned single target dir in main checkout to prevent race conditions and thrashing | Per-worktree target dir | Workspace crate fingerprints thrash | decisions.yaml |
| Preflight | Peer review | Configured Kiro GPT-5.6 Terra shell-out for independent code and doc review | Single-agent self-review | Reviewer independence lost | decisions.yaml |
