---
type: mandate
id: DEC-042
status: accepted
touches:
  - .control/decisions/DEC-042-autopilot-mandate-for-spec-28-delivery.md
  - .control/memlog/autopilot-DEC-042.md
  - .control/registry/decisions.yaml
  - .control/registry/specs.yaml
  - .scratch/spec-28-modal-dialog-height-and-realtime-tray-warning-cleanup/SPEC.md
  - .scratch/spec-28-modal-dialog-height-and-realtime-tray-warning-cleanup/issues/01-compact-modal-dialog-height.md
  - .scratch/spec-28-modal-dialog-height-and-realtime-tray-warning-cleanup/issues/02-realtime-tray-warning-clearance-across-config-sources.md
  - crates/settings/ui/main_window.slint
  - crates/settings/src/shortcut_row_slint_snapshot.rs
  - crates/daemon/src/tray.rs
  - crates/daemon/src/hook.rs
  - crates/daemon/src/log.rs
  - crates/daemon/src/config.rs
  - crates/daemon/src/autostart.rs
  - 3p.md
  - docs/3p.md
supersedes: null
superseded_by: null
created: '2026-09-15'
accepted_by: kodesh87 (Product Owner, in session), 2026-09-15
---

# DEC-042 — Autopilot mandate for SPEC-28 delivery

## Decision

`wdi-autopilot` is mandated to carry Wira Desk from its current position — G1-G4 passed, `SPEC-1` through
`SPEC-27` closed, `SPEC-28` open with its tickets planned and peer-reviewed — through every runnable
`FR`/spec/ticket to completion, without further owner check-ins until Finish or a parked row. Parameters
live on this decision's row in `decisions.yaml` under `mandate:`.

In scope:
- `SPEC-28`: Compact Modal Dialog Height and Comprehensive Real-Time Tray Warning Clearance
  - `SPEC-28-01`: Compact modal dialog height and content-derived container geometry
  - `SPEC-28-02`: Real-time tray warning clearance on shortcut collision repair and configuration re-evaluation

### Architectural & Scope Boundaries (SPEC-28)

1. **Compact Modal Dialog Height (SPEC-28-01):**
   - Bind Factory Reset confirmation card height directly to inner layout preferred height: `height: dialog_layout.preferred-height;`.
   - Name inner vertical layout `dialog_layout := VerticalLayout`.
   - Provide test-addressable identifier on dialog card (`accessible-role: group; accessible-label: "Factory reset confirmation card";`).
   - Add runtime Slint snapshot regression tests measuring actual rendered card geometry at normal 760×560 window size, asserting compact height (< 280px, >= 180px) and vertical centering.

2. **Synchronous & Causal Real-Time Tray Warning Clearance (SPEC-28-02):**
   - `ReloadOutcome::Applied` synchronously owns shortcut collision detection (`has_shortcut_collision: bool`) and configuration generation (`generation: u64`), updating `config_collision` in real time.
   - Tag collision log messages with `WARN_CAUSE_CONFIG_COLLISION` and drop messages with stale generation in `handle_log_warning`, guaranteeing that delayed Hook messages cannot re-latch Warning state after a clean reload.
   - Introduce structured `autostart::task_status()` distinguishing `Absent`, `Registered`, and `Unknown`. Clear `acl_insecure = false` only upon confirmed absence or admin-only path; fail-safely retain previous warning state on observation failure or unknown ACL verdict.
   - Critical hook failure state (`TrayState::Critical`) is strictly preserved across all reload outcomes.

3. **Coordination & Review:**
   - In-session default coordinator executes coding and self-review.
   - Shelled-out peer review via Kiro GPT-5.6 Terra (`kiro-cli chat --model gpt-5.6-terra --effort high -a --no-interactive "<prompt>"`).
   - Single shared worktree (`.`) and unified build target.

## Why

Unattended delivery loop authorised by the product owner under WDI Method G5 Release gate.

## Cost

One autopilot cycle of unattended implementation and peer review.
