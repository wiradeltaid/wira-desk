---
type: mandate
id: DEC-038
status: accepted
touches:
  - .control/decisions/DEC-038-autopilot-mandate-for-spec-24-delivery.md
  - .control/memlog/autopilot-DEC-038.md
  - .control/registry/decisions.yaml
  - .control/registry/specs.yaml
  - .control/registry/requirements-wira-desk.yaml
  - .scratch/spec-24-snap-to-edge-default-percentage-to-67/issues/01-feature-default-snap-percentage-to-67.md
  - crates/shared/src/constants.rs
  - crates/shared/src/config.rs
  - .what/window-management/04-usecases/UC-9-snap-window-custom-percentage.md
  - .how/settings/05-model/data-model.md
  - 3p.md
  - docs/3p.md
supersedes: null
superseded_by: null
created: '2026-09-14'
accepted_by: kodesh87 (Product Owner, in session), 2026-09-14
---

# DEC-038 — Autopilot mandate for SPEC-24 delivery

## Decision

`wdi-autopilot` is mandated to carry Wira Desk from its current position — G1-G4 passed, `SPEC-1` through
`SPEC-23` closed, `SPEC-24` open with its ticket planned and peer-reviewed — through every runnable
`FR`/spec/ticket to completion, without further owner check-ins until Finish or a parked row. Parameters
live on this decision's row in `decisions.yaml` under `mandate:`.

In scope:
- `SPEC-24`: Update custom-percentage edge snap default percentage to 67% (`FR-26`, ticket `SPEC-24-01`)

### Architectural & Scope Boundaries (SPEC-24)

1. **Feature Update custom-percentage edge snap default percentage to 67% (SPEC-24-01):**
   - Update shared default constant `DEFAULT_SNAP_PERCENT` from `50` to `67` in `crates/shared/src/constants.rs` and update field comments in `crates/shared/src/config.rs`.
   - Preserve `MIN_SNAP_PERCENT = 1` and `MAX_SNAP_PERCENT = 99`.
   - Maintain full backwards compatibility for existing `config.toml` files with custom/omitted values.
   - Align corpus specifications (`UC-9`, `FR-26`, and settings data-model) to reflect 67% fresh default.
   - Run full workspace verification (format, clippy, unit and integration tests).

## Why

Unattended delivery loop authorised by the product owner under WDI Method G5 Release gate.

## Cost

One autopilot cycle of unattended implementation and peer review.
