---
type: mandate
id: DEC-045
status: accepted
touches:
  - .control/decisions/DEC-045-autopilot-mandate-for-spec-31-delivery.md
  - .control/memlog/autopilot-DEC-045.md
  - .control/registry/decisions.yaml
  - .control/registry/specs.yaml
  - .scratch/spec-31-about-tab-layout-and-error-message/SPEC.md
  - .scratch/spec-31-about-tab-layout-and-error-message/issues/01-about-pane-layout-and-duplicate-link-removal.md
  - .scratch/spec-31-about-tab-layout-and-error-message/issues/02-update-check-error-message-differentiation.md
supersedes: null
superseded_by: null
created: '2026-09-24'
accepted_by: Wira (Product Owner, in session), 2026-09-24
---

# DEC-045 — Autopilot mandate for SPEC-31 delivery

## Decision

`wdi-autopilot` is mandated to carry Wira Desk from its current position — G1-G4 passed, `SPEC-1` through
`SPEC-30` closed, `SPEC-31` open with its 2 tickets planned and reviewed — through every runnable
`FR`/spec/ticket to completion, without further owner check-ins until Finish or a parked row. Parameters
live on this decision's row in `decisions.yaml` under `mandate:`.

In scope:
- `SPEC-31`: About Tab Layout Polish, Duplicate Link Removal, and Update Error Differentiation
  - `SPEC-31-01`: About pane layout: word wrapping, non-breaking space, license lines, and GitHub button removal (FR-24, UC-8)
  - `SPEC-31-02`: Update check error message: differentiate endpoint failure from download server (FR-25, UC-8)

### Architectural & Scope Boundaries (SPEC-31)

1. **About Pane UI Layout & Typography (A1, A2, A3, A4):**
   - Natural word wrapping for toggle description; join `"Windows\u{00A0}11"` with non-breaking space.
   - Three distinct license lines: GPL statement, Full terms line, and Third-party notices line.
   - Remove redundant `[GitHub]` button from Card 3 action row while strictly preserving `"Source"` text link for GPL source access.

2. **Update Error Differentiation (A5):**
   - Separate descriptor endpoint failure message (`"wiradelta.id answered with status {code}. You can try again."`) from installer binary download failure (`"The download server answered with status {code}."`).

3. **Coordination & Review:**
   - In-session coordinator executes coding, self-review, and TDD red-to-green verification directly in the run worktree.
   - Shelled-out peer review via Kiro GPT-5.6 Terra (`kiro-agent chat --model gpt-5.6-terra --effort high --trust-tools=fs_read --no-interactive`).
   - Deep analyst: none (coordinator self-review for docs/architecture).
   - Single worktree (`.`) and unified build target.

## Why

Unattended delivery loop authorised by the product owner under WDI Method G5 Release gate.

## Cost

Autonomous autopilot cycles of unattended implementation, verification, and peer review.
