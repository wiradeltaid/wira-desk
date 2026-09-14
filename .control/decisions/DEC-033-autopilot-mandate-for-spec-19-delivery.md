---
type: mandate
id: DEC-033
status: accepted
touches:
  - .control/decisions/DEC-033-autopilot-mandate-for-spec-19-delivery.md
  - .control/memlog/autopilot-DEC-033.md
  - .control/registry/decisions.yaml
  - .control/registry/specs.yaml
supersedes: null
superseded_by: null
created: '2026-09-14'
accepted_by: kodesh87 (Product Owner, in session), 2026-09-14
---

# DEC-033 — Autopilot mandate for SPEC-19 delivery

## Decision

`wdi-autopilot` is mandated to carry Wira Desk from its current position — G1-G4 passed, `SPEC-1` through
`SPEC-18` closed, `SPEC-19` open with its ticket planned and peer-reviewed — through every runnable
`FR`/spec/ticket to completion, without further owner check-ins until Finish or a parked row. Parameters
live on this decision's row in `decisions.yaml` under `mandate:`.

In scope:
- `SPEC-19`: Visual switcher mouse click selection synchronization & default hold delay threshold (`DEF-21`, ticket `SPEC-19-01`)

### Architectural & Scope Boundaries (SPEC-19)

1. **Defect DEF-21 Visual Switcher Mouse Selection Synchronization & Default Hold Delay Threshold (SPEC-19-01):**
   - Synchronize visual switcher candidate selection so mouse hover and mouse click interactions on the overlay window properly update the controller's candidate selection.
   - When handling `WM_MOUSEMOVE` or `WM_LBUTTONDOWN` in the overlay, or on `candidates_from_selection()`, guarantee that `candidates_from_selection()` returns candidates starting with the overlay's selected card index.
   - Update default visual hold delay threshold from 150 ms to 300 ms across shared configuration, Settings UI defaults, daemon, and test assertions.

Execution constraints specified by owner:
1. Coding directly implemented by coordinator with mandatory self code review.
2. Code review performed 2 times: self code review (in-session default, no separate dispatch) + independent peer review shell-out via:
   `$env:ANTHROPIC_BASE_URL = $null; $env:ANTHROPIC_API_KEY = $null; $env:ANTHROPIC_AUTH_TOKEN = $null; $env:CLAUDE_CONFIG_DIR = "$HOME\.claude"; claude --model claude-sonnet-5 --effort high --dangerously-skip-permissions -p "<prompt>"`
3. Automated smoke testing executed by agent.
4. Peer review for coding / document review / peer analysis and document repair delegated to Claude Sonnet 5 shell-out — mandate: kalau draf/kode menyentuh architecture spine, SRS, SDD, atau SPEC, boleh langsung jalankan wdi-review dan edit dokumennya sendiri, tidak perlu tanya dulu.
5. Mengenai build aplikasi/jalankan aplikasi: satu worktree yang sama dipakai bersama untuk semua build/run di run ini (`worktree: .`). Sebelum peer review ikut build/run, kasih tahu dia path worktree yang sedang dipakai coordinator sekarang — peer MUST NOT membuat worktree sendiri, dan MUST NOT build/run bersamaan selagi coordinator sendiri sedang build/run.

## Why

Unattended delivery of SPEC-19 defect remediation across the full cycle from ticket implementation, unit and integration testing, code review, documentation drift reconciliation, and release gating.

## Cost if wrong

Iteration stops at capacity or blocked state; changes roll back or park; owner intervention required at finish.
