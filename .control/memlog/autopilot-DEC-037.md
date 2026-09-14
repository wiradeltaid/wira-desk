---
artifact: .control/decisions/DEC-037-autopilot-mandate-for-spec-23-delivery.md
skill: wdi-autopilot
date: 2026-09-14
---

# Memlog — autopilot run DEC-037

## Resume

Iteration: 1 (boundary: HEAD)
Run branch: autopilot/DEC-037, PR ready for owner review
Stopped at: Done (all specifications closed: SPEC-23 delivered, promise progress 100%, 90/90 counted RTM rows green)
Blocked: —
Parked: —
Next: —

## Decisions

| When | Where | Decided | Instead of | Cost if wrong | Landed in |
|---|---|---|---|---|---|
| Preflight | Mandate acceptance | Accepted DEC-037 for SPEC-23 delivery per owner launch | Stopping for manual gate check-ins | Supersede DEC-037 | .control/decisions/DEC-037-autopilot-mandate-for-spec-23-delivery.md, decisions.yaml |
| Preflight | Build safety | Pinned single target dir in main checkout to prevent race conditions and thrashing | Per-worktree target dir | Workspace crate fingerprints thrash | decisions.yaml |
| Preflight | Peer review | Configured Kiro GPT-5.6 Terra shell-out for independent code and doc review | Single-agent self-review | Reviewer independence lost | decisions.yaml |
| Iter 1 | SPEC-23-01 | Streamlined UpdateReadyMemo copy to 'Preserved across updates; clean installs start fresh.' | Leaving verbose runtime mechanics | Cluttered memo with inaccurate unconditional claims | packaging/wiradesk.iss |
| Iter 1 | SPEC-23-01 | Completely deleted legacy WinTick migration shims (M-01, M-02, M-03) and startup invocations | Retaining legacy migration routines | Resurrecting obsolete pre-DEC-008 shortcuts and July 2026 logs | crates/shared/src/lib.rs, crates/daemon/src/main.rs, crates/settings/src/main.rs |
| Iter 1 | SPEC-23-01 | Added injectable AppData path helpers and isolated test verifying modern defaults under legacy residue | Ad hoc environment mutation | Non-deterministic tests and untested legacy residue | crates/shared/src/config.rs, crates/shared/src/lib.rs |
| Iter 1 | Peer review | Updated README.md, uninstaller comments, and verify-settings-runtime.ps1 to remove stale WinTick promises | Leaving obsolete public docs | Misleading users on factory reset behavior | README.md, packaging/wiradesk.iss, scripts/verify-settings-runtime.ps1 |
| Iter 1 | Peer review | Incorporated Kiro GPT-5.6 Terra review feedback on clean default configuration test isolation | Leaving unanchored temp directory assertion | Untested runtime AppData resolution | crates/shared/src/config.rs |
