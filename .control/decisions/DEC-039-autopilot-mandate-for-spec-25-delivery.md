---
type: mandate
id: DEC-039
status: accepted
touches:
  - .control/decisions/DEC-039-autopilot-mandate-for-spec-25-delivery.md
  - .control/memlog/autopilot-DEC-039.md
  - .control/registry/decisions.yaml
  - .control/registry/specs.yaml
  - .scratch/spec-25-restore-defaults-and-about-shortcuts-polish/SPEC.md
  - .scratch/spec-25-restore-defaults-and-about-shortcuts-polish/issues/01-shortcuts-restore-conflict-banner-and-tip-cleanup.md
  - .scratch/spec-25-restore-defaults-and-about-shortcuts-polish/issues/02-about-pane-action-buttons-and-factory-reset.md
  - 3p.md
  - docs/3p.md
supersedes: null
superseded_by: null
created: '2026-09-15'
accepted_by: kodesh87 (Product Owner, in session), 2026-09-15
---

# DEC-039 — Autopilot mandate for SPEC-25 delivery

## Decision

`wdi-autopilot` is mandated to carry Wira Desk from its current position — G1-G4 passed, `SPEC-1` through
`SPEC-24` closed, `SPEC-25` open with its tickets planned and peer-reviewed — through every runnable
`FR`/spec/ticket to completion, without further owner check-ins until Finish or a parked row. Parameters
live on this decision's row in `decisions.yaml` under `mandate:`.

In scope:
- `SPEC-25`: Restore defaults and About / Shortcuts panes polish
  - `SPEC-25-01`: Shortcuts pane — restore defaults button, conflict resolution banner, and tip removal (`UC-4`)
  - `SPEC-25-02`: About pane — three-button link row and restore-all-preferences card

### Architectural & Scope Boundaries (SPEC-25)

1. **Shortcuts pane restore defaults & banner (SPEC-25-01):**
   - Add a subtle secondary action button `Restore shortcuts` in the shortcuts pane header to stage fresh factory default shortcuts into the current draft.
   - Introduce a high-contrast inline conflict banner in the shortcuts pane for the legacy stack-chord conflict (`Alt+Up` collision) with one-click resolution.
   - Remove redundant `Tip: Click a shortcut to edit it` helper text to match clean typography standards.
   - Draft-only mutation: staging defaults marks draft dirty; reverting restores saved config.

2. **About pane actions and factory reset (SPEC-25-02):**
   - Refactor About pane navigation to three distinct action buttons (`Open config folder`, `Check for updates`, `Report an issue`) on their own row below branding.
   - Add `Restore all preferences` card at the bottom of the About pane with two-step confirmation modal/banner to stage factory defaults across all sections.
   - Draft-only mutation: factory reset stages full default configuration into draft; saved preferences are untouched until explicitly saved.

3. **Coordination & Review:**
   - In-session default coordinator executes coding and self-review.
   - Shelled-out peer review via Kiro GPT-5.6 Terra (`kiro-cli chat --model gpt-5.6-terra --effort high -a --no-interactive "<prompt>"`).
   - Single shared worktree (`.`) and unified build target.

## Why

Unattended delivery loop authorised by the product owner under WDI Method G5 Release gate.

## Cost

One autopilot cycle of unattended implementation and peer review.
