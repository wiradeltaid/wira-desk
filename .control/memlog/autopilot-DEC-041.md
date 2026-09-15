---
artifact: .control/decisions/DEC-041-autopilot-mandate-for-spec-27-delivery.md
skill: wdi-autopilot
date: 2026-09-15
---

# Memlog — autopilot run DEC-041

## Resume

Iteration: 1 (boundary: in flight)
Run branch: autopilot/DEC-041, PR not open yet
Stopped at: Done — SPEC-27 delivered and closed under mandate DEC-041
Blocked: —
Parked: —
Next: § Finish (smoke test, validate, push, open PR)

## Decisions

| When | Where | Decided | Instead of | Cost if wrong | Landed in |
|---|---|---|---|---|---|
| Preflight | Mandate acceptance | Accepted DEC-041 for SPEC-27 delivery per owner launch | Stopping for manual gate check-ins | Supersede DEC-041 | .control/decisions/DEC-041-autopilot-mandate-for-spec-27-delivery.md, decisions.yaml |
| Preflight | Build safety | Pinned single target dir in main checkout to prevent race conditions and thrashing | Per-worktree target dir | Workspace crate fingerprints thrash | decisions.yaml |
| Preflight | Peer review | Configured Kiro GPT-5.6 Terra shell-out for independent code and doc review | Single-agent self-review | Reviewer independence lost | decisions.yaml |
| Iteration 1 | SPEC-27-01 implementation | Implemented structured warning causes and synchronous reload warning reset | Asynchronous queue-only warning dispatch | Win32 message race condition re-latches warning | tray.rs, config.rs, log.rs, autostart.rs |
| Iteration 1 | Peer review adjudication | Accepted Kiro GPT-5.6 Terra review: routed config warning solely through synchronous outcome handler and added interleaving regression test | Retaining asynchronous WM_APP_LOG_WARNING on reload | Stale warning re-latches icon after valid reload | tray.rs, config.rs |
| Iteration 1 | SPEC-27-02 implementation | Isolated auto_start from General defaults, polished Card 3, reduced normal_height to 560px, and redesigned modal without blue outline | Coupling auto_start or retaining blue button outlines | False non-default state or visual inconsistency | app.rs, about_pane.slint, main_window.slint, shortcut_row_slint_snapshot.rs |
| Iteration 1 | Peer review adjudication | Accepted Kiro GPT-5.6 Terra review: strengthened Card 3 bounds check to strict exclusive inequality with click invocation, and expanded modal style assertions | Inclusive bounds or partial declaration slicing | Boundary clipping or undetected outline regression | shortcut_row_slint_snapshot.rs |
