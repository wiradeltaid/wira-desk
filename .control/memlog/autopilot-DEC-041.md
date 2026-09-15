---
artifact: .control/decisions/DEC-041-autopilot-mandate-for-spec-27-delivery.md
skill: wdi-autopilot
date: 2026-09-15
---

# Memlog — autopilot run DEC-041

## Resume

Iteration: 1 (boundary: in flight)
Run branch: autopilot/DEC-041, PR not open yet
Stopped at: —
Blocked: —
Parked: —
Next: SPEC-27-02: General auto-start defaults isolation, About card 3 polish, and modal confirmation redesign

## Decisions

| When | Where | Decided | Instead of | Cost if wrong | Landed in |
|---|---|---|---|---|---|
| Preflight | Mandate acceptance | Accepted DEC-041 for SPEC-27 delivery per owner launch | Stopping for manual gate check-ins | Supersede DEC-041 | .control/decisions/DEC-041-autopilot-mandate-for-spec-27-delivery.md, decisions.yaml |
| Preflight | Build safety | Pinned single target dir in main checkout to prevent race conditions and thrashing | Per-worktree target dir | Workspace crate fingerprints thrash | decisions.yaml |
| Preflight | Peer review | Configured Kiro GPT-5.6 Terra shell-out for independent code and doc review | Single-agent self-review | Reviewer independence lost | decisions.yaml |
| Iteration 1 | SPEC-27-01 implementation | Implemented structured warning causes and synchronous reload warning reset | Asynchronous queue-only warning dispatch | Win32 message race condition re-latches warning | tray.rs, config.rs, log.rs, autostart.rs |
| Iteration 1 | Peer review adjudication | Accepted Kiro GPT-5.6 Terra review: routed config warning solely through synchronous outcome handler and added interleaving regression test | Retaining asynchronous WM_APP_LOG_WARNING on reload | Stale warning re-latches icon after valid reload | tray.rs, config.rs |
