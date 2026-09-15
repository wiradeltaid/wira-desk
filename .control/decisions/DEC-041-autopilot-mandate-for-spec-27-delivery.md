---
type: mandate
id: DEC-041
status: accepted
touches:
  - .control/decisions/DEC-041-autopilot-mandate-for-spec-27-delivery.md
  - .control/memlog/autopilot-DEC-041.md
  - .control/registry/decisions.yaml
  - .control/registry/specs.yaml
  - .scratch/spec-27-tray-warning-reset-auto-start-isolation-and-settings-ui-polish/SPEC.md
  - .scratch/spec-27-tray-warning-reset-auto-start-isolation-and-settings-ui-polish/issues/01-real-time-tray-warning-reset-on-config-reload.md
  - .scratch/spec-27-tray-warning-reset-auto-start-isolation-and-settings-ui-polish/issues/02-general-auto-start-isolation-about-card-and-modal-polish.md
  - crates/daemon/src/tray.rs
  - crates/daemon/src/worker.rs
  - crates/daemon/src/config.rs
  - crates/settings/src/app.rs
  - crates/settings/ui/panes/about_pane.slint
  - crates/settings/ui/panes/general_pane.slint
  - crates/settings/ui/main_window.slint
  - crates/settings/src/shortcut_row_slint_snapshot.rs
  - 3p.md
  - docs/3p.md
supersedes: null
superseded_by: null
created: '2026-09-15'
accepted_by: kodesh87 (Product Owner, in session), 2026-09-15
---

# DEC-041 — Autopilot mandate for SPEC-27 delivery

## Decision

`wdi-autopilot` is mandated to carry Wira Desk from its current position — G1-G4 passed, `SPEC-1` through
`SPEC-26` closed, `SPEC-27` open with its tickets planned and peer-reviewed — through every runnable
`FR`/spec/ticket to completion, without further owner check-ins until Finish or a parked row. Parameters
live on this decision's row in `decisions.yaml` under `mandate:`.

In scope:
- `SPEC-27`: Tray Warning Real-time Reset, General Auto-Start Isolation, About Card Polish, and Modal Redesign
  - `SPEC-27-01`: Real-time tray warning reset upon successful configuration reload
  - `SPEC-27-02`: General auto-start defaults isolation, About card 3 polish, and modal confirmation redesign

### Architectural & Scope Boundaries (SPEC-27)

1. **Selective Real-time Tray Warning Reset (SPEC-27-01):**
   - Track warning causes in `TrayData` explicitly (`config_rejected`, `acl_insecure`, `simulated`).
   - On successful config reload (`ReloadOutcome::Applied`), clear `config_rejected = false`.
   - Transition to `TrayState::Normal` immediately if no other warning cause remains active and state is `TrayState::Warning`.
   - Critical state (`TrayState::Critical`, e.g. dead hook) is preserved and never downgraded by config reload.

2. **General Auto-Start Isolation (SPEC-27-02):**
   - `general_differs_from_default` compares only `switcher.visual_enabled` and `switcher.visual_hold_delay_ms` against `Config::default()`. `auto_start` is excluded from difference detection.
   - `restore_general_defaults` preserves `draft.general.auto_start` while resetting switcher preferences.
   - Full factory reset (`factory_reset_defaults`) retains complete default reset including `auto_start: false`.

3. **About Card 3 & Modal Redesign (SPEC-27-02):**
   - Remove redundant `Troubleshooting & Recovery` heading in About pane Card 3; retain `Restore all preferences to defaults`.
   - Card 3 layout padding adjusted so `[Reset all settings…]` is enclosed cleanly without clipping or overflow.
   - Reduce Settings window `normal_height` from 610px to 560px.
   - Redesign Factory Reset modal dialog: 12px border radius, `Palette.bg_card`, `Palette.stroke_card`, drop shadow, removing 2px blue focus outline in favor of cohesive styling matching Onboarding aesthetic.

4. **Coordination & Review:**
   - In-session default coordinator executes coding and self-review.
   - Shelled-out peer review via Kiro GPT-5.6 Terra (`kiro-cli chat --model gpt-5.6-terra --effort high -a --no-interactive "<prompt>"`).
   - Single shared worktree (`.`) and unified build target.

## Why

Unattended delivery loop authorised by the product owner under WDI Method G5 Release gate.

## Cost

One autopilot cycle of unattended implementation and peer review.
