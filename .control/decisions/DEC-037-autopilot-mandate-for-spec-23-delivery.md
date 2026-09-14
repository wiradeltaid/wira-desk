---
type: mandate
id: DEC-037
status: accepted
touches:
  - .control/decisions/DEC-037-autopilot-mandate-for-spec-23-delivery.md
  - .control/memlog/autopilot-DEC-037.md
  - .control/registry/decisions.yaml
  - .control/registry/specs.yaml
  - .scratch/spec-23-ready-memo-brevity-and-legacy-migration-retirement/issues/01-defect-def-25-ready-memo-brevity-and-legacy-migration-retirement.md
  - packaging/wiradesk.iss
  - scripts/verify-installer-safety.ps1
  - crates/shared/src/migrate.rs
  - crates/daemon/src/main.rs
  - crates/settings/src/main.rs
  - 3p.md
  - docs/3p.md
supersedes: null
superseded_by: null
created: '2026-09-14'
accepted_by: kodesh87 (Product Owner, in session), 2026-09-14
---

# DEC-037 — Autopilot mandate for SPEC-23 delivery

## Decision

`wdi-autopilot` is mandated to carry Wira Desk from its current position — G1-G4 passed, `SPEC-1` through
`SPEC-22` closed, `SPEC-23` open with its ticket planned and peer-reviewed — through every runnable
`FR`/spec/ticket to completion, without further owner check-ins until Finish or a parked row. Parameters
live on this decision's row in `decisions.yaml` under `mandate:`.

In scope:
- `SPEC-23`: Ready memo brevity and legacy WinTick migration retirement (`DEF-25`, ticket `SPEC-23-01`)

### Architectural & Scope Boundaries (SPEC-23)

1. **Defect DEF-25 Ready memo brevity and legacy WinTick migration retirement (SPEC-23-01):**
   - Streamline `UpdateReadyMemo` copy in `packaging/wiradesk.iss` to be concise, clear, and focused.
   - Retire legacy WinTick migration code from `crates/shared/src/migrate.rs`, `crates/daemon/src/main.rs`, and `crates/settings/src/main.rs` as superseded by modern config defaults and packaging clean-install lifecycle.
   - Verify installer safety script `scripts/verify-installer-safety.ps1` remains green.

## Why

Unattended delivery loop authorised by the product owner under WDI Method G5 Release gate.

## Cost

One autopilot cycle of unattended implementation and peer review.
