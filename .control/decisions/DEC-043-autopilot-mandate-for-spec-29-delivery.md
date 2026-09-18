---
type: mandate
id: DEC-043
status: accepted
touches:
  - .control/decisions/DEC-043-autopilot-mandate-for-spec-29-delivery.md
  - .control/memlog/autopilot-DEC-043.md
  - .control/registry/decisions.yaml
  - .control/registry/specs.yaml
  - .scratch/spec-29-snap-custom-top-default-to-33/SPEC.md
  - .scratch/spec-29-snap-custom-top-default-to-33/issues/01-update-default-snap-custom-top-percentage-to-33.md
  - crates/shared/src/config.rs
  - crates/shared/src/constants.rs
  - crates/settings/src/persistence.rs
  - README.md
  - docs/CONFIGURATION.md
  - .control/registry/requirements-wira-desk.yaml
  - .what/window-management/04-usecases/UC-9-snap-window-custom-percentage.md
  - 3p.md
  - docs/3p.md
supersedes: null
superseded_by: null
created: '2026-09-18'
accepted_by: kodesh87 (Product Owner, in session), 2026-09-18
---

# DEC-043 — Autopilot mandate for SPEC-29 delivery

## Decision

`wdi-autopilot` is mandated to carry Wira Desk from its current position — G1-G4 passed, `SPEC-1` through
`SPEC-28` closed, `SPEC-29` open with its ticket planned and peer-reviewed — through every runnable
`FR`/spec/ticket to completion, without further owner check-ins until Finish or a parked row. Parameters
live on this decision's row in `decisions.yaml` under `mandate:`.

In scope:
- `SPEC-29`: Default Custom-Percentage Top Edge Snap to 33%
  - `SPEC-29-01`: Update default snap custom top percentage to 33%

### Architectural & Scope Boundaries (SPEC-29)

1. **Shared Constants & Configuration Defaults (`crates/shared/`):**
   - Introduce `pub const DEFAULT_SNAP_PERCENT_TOP: u32 = 33;` in `crates/shared/src/constants.rs`.
   - Retain `pub const DEFAULT_SNAP_PERCENT: u32 = 67;` as the default for left, right, and bottom edges (`percent_left`, `percent_right`, `percent_bottom`).
   - Retain `MIN_SNAP_PERCENT = 1` and `MAX_SNAP_PERCENT = 99`.
   - In `SnappingConfig::default()`, set `percent_top: crate::constants::DEFAULT_SNAP_PERCENT_TOP` (33), while `percent_left`, `percent_right`, and `percent_bottom` remain at `DEFAULT_SNAP_PERCENT` (67).
   - In TOML deserialization, preserve explicit `percent_top` values if present on disk, while omitted `percent_top` defaults in memory to 33.

2. **Settings UI & Persistence (`crates/settings/`):**
   - In `persistence::tests::default_config_uses_frozen_shortcuts`, assert `percent_top == 33` and other directional edges `== 67`.
   - Verify Slint settings view dynamically respects runtime `Config::default().snapping.percent_top` without hardcoding.

3. **Coordination & Review:**
   - In-session default coordinator executes coding, self-review, and TDD red-to-green verification.
   - Shelled-out peer review via Kiro GPT-5.6 Terra (`kiro-cli chat --model gpt-5.6-terra --effort high --trust-tools=fs_read --no-interactive`).
   - Deep analyst: none (coordinator self-analysis).
   - Single shared worktree (`.`) and unified build target.

## Why

Unattended delivery loop authorised by the product owner under WDI Method G5 Release gate.

## Cost

One autopilot cycle of unattended implementation and peer review.
