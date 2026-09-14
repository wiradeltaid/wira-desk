---
artifact: .control/decisions/DEC-035-autopilot-mandate-for-spec-21-delivery.md
skill: wdi-autopilot
date: 2026-09-14
---

# Memlog — autopilot run DEC-035

## Resume

Iteration: 1 (boundary: HEAD)
Run branch: autopilot/DEC-035, PR ready for owner review
Stopped at: Done (all specifications closed: SPEC-21 delivered, promise progress 100%, 90/90 counted RTM rows green)
Blocked: —
Parked: —
Next: —

## Decisions

| When | Where | Decided | Instead of | Cost if wrong | Landed in |
|---|---|---|---|---|---|
| Preflight | Mandate acceptance | Accepted DEC-035 for SPEC-21 delivery per owner launch | Stopping for manual gate check-ins | Supersede DEC-035 | .control/decisions/DEC-035-autopilot-mandate-for-spec-21-delivery.md, decisions.yaml |
| Preflight | Build safety | Pinned single target dir in main checkout to prevent race conditions and thrashing | Per-worktree target dir | Workspace crate fingerprints thrash | decisions.yaml |
| Preflight | Peer review | Configured Kiro GPT-5.6 Terra shell-out for independent code and doc review | Single-agent self-review | Reviewer independence lost | decisions.yaml |
| Iter 1 | SPEC-21-01 | Implemented ISPP compile-time version guard accepting decimal major.minor.patch | Permitting metadata or missing components | Invalid versions slip into release | packaging/wiradesk.iss |
| Iter 1 | SPEC-21-01 | Implemented 64-bit HKLM downgrade rejection and strict no-trim version parsing | Allowing older installers to overwrite newer installs | Binary and schema corruption | packaging/wiradesk.iss |
| Iter 1 | SPEC-21-01 | Replaced cmd pipeline in CheckProcessState with robust tasklist probe to fail closed | Blindly treating probe failure as process absence | Locked files cause copy failures | packaging/wiradesk.iss |
| Iter 1 | SPEC-21-01 | Updated Ready memo with explicit app, config, and task paths without service claims | Leaving persistent paths undisclosed | User confusion on auto-start | packaging/wiradesk.iss |
| Iter 1 | SPEC-21-01 | Added verify-installer-safety.ps1 testing real packaging/wiradesk.iss and 64-bit HKLM fixtures | Testing reimplemented parser | Drift between test and production installer | scripts/verify-installer-safety.ps1, .github/workflows/ci.yml |
