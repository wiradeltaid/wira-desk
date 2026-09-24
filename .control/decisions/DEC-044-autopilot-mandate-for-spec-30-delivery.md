---
type: mandate
id: DEC-044
status: applied
touches:
  - .control/decisions/DEC-044-autopilot-mandate-for-spec-30-delivery.md
  - .control/memlog/autopilot-DEC-044.md
  - .control/registry/decisions.yaml
  - .control/registry/specs.yaml
  - .scratch/spec-30-wira-desk-0-3-0-release-and-endpoint-handover/SPEC.md
supersedes: null
superseded_by: null
created: '2026-09-24'
accepted_by: Wira (Product Owner, in session), 2026-09-24
---

# DEC-044 — Autopilot mandate for SPEC-30 delivery

## Decision

`wdi-autopilot` is mandated to carry Wira Desk from its current position — G1-G4 passed, `SPEC-1` through
`SPEC-29` closed, `SPEC-30` open with its 10 tickets planned and reviewed — through every runnable
`FR`/spec/ticket to completion, without further owner check-ins until Finish or a parked row. Parameters
live on this decision's row in `decisions.yaml` under `mandate:`.

In scope:
- `SPEC-30`: Release 0.3.0 Endpoint and Distribution Handover
  - `SPEC-30-01`: Update check to the wiradelta.id endpoint (H-01)
  - `SPEC-30-02`: Embedded URL registry with trailing slash (H-02)
  - `SPEC-30-03`: About and General pane copy polish (H-03)
  - `SPEC-30-04`: Legal copies with stamp synchronization (H-04)
  - `SPEC-30-05`: Threat model update for 0.3.0 (H-05)
  - `SPEC-30-06`: README, Scoop template, and WinGet generator (H-06)
  - `SPEC-30-07`: Portable zip packaging and release artifact guard (H-07)
  - `SPEC-30-08`: Public facts synchronization (H-08)
  - `SPEC-30-09`: Corpus alignment for update check (H-09)
  - `SPEC-30-10`: Release 0.3.0 version bump and changelog (H-10)

### Architectural & Scope Boundaries (SPEC-30)

1. **Update Subsystem & Network Isolation:**
   - Migrate descriptor URL to `https://wiradelta.id/api/v1/update/wira-desk/`.
   - Implement User-Agent pure function strictly formatted as `WiraDesk/<version> (Windows <major>.<minor>.<build>; <arch>)`.
   - Prevent descriptor HTTP redirects (`WINHTTP_OPTION_REDIRECT_POLICY_NEVER`), while preserving redirects for GitHub installer downloads.

2. **Embedded URLs & Copy Alignment:**
   - Centralize all `wiradelta.id` URLs with trailing slashes into an allowlisted registry.
   - Align About and General pane copy word-for-word with approved ops fixtures.

3. **Coordination & Review:**
   - In-session coordinator executes coding, self-review, and TDD red-to-green verification directly in the run worktree.
   - Shelled-out peer review via Kiro GPT-5.6 Terra (`kiro-agent chat --model gpt-5.6-terra --effort high --trust-tools=fs_read --no-interactive`).
   - Deep analyst: none (coordinator self-review for docs/architecture).
   - Single worktree (`.`) and unified build target.

## Why

Unattended delivery loop authorised by the product owner under WDI Method G5 Release gate.

## Cost

Autonomous autopilot cycles of unattended implementation, verification, and peer review.
