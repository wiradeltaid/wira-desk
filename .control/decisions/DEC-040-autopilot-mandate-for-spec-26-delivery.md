---
type: mandate
id: DEC-040
status: accepted
touches:
  - .control/decisions/DEC-040-autopilot-mandate-for-spec-26-delivery.md
  - .control/memlog/autopilot-DEC-040.md
  - .control/registry/decisions.yaml
  - .control/registry/specs.yaml
  - .scratch/spec-26-header-defaults-and-about-card-hierarchy/SPEC.md
  - .scratch/spec-26-header-defaults-and-about-card-hierarchy/issues/01-pane-header-defaults-button-and-conditional-visibility.md
  - .scratch/spec-26-header-defaults-and-about-card-hierarchy/issues/02-about-pane-card-hierarchy-and-troubleshooting-redesign.md
  - crates/settings/ui/panes/shortcuts_pane.slint
  - crates/settings/ui/panes/general_pane.slint
  - crates/settings/ui/panes/mouse_pane.slint
  - crates/settings/ui/panes/about_pane.slint
  - crates/settings/ui/main_window.slint
  - crates/settings/src/app.rs
  - crates/settings/src/main.rs
  - crates/settings/src/shortcut_row_slint_snapshot.rs
  - 3p.md
  - docs/3p.md
supersedes: null
superseded_by: null
created: '2026-09-15'
accepted_by: kodesh87 (Product Owner, in session), 2026-09-15
---

# DEC-040 — Autopilot mandate for SPEC-26 delivery

## Decision

`wdi-autopilot` is mandated to carry Wira Desk from its current position — G1-G4 passed, `SPEC-1` through
`SPEC-25` closed, `SPEC-26` open with its tickets planned and peer-reviewed — through every runnable
`FR`/spec/ticket to completion, without further owner check-ins until Finish or a parked row. Parameters
live on this decision's row in `decisions.yaml` under `mandate:`.

In scope:
- `SPEC-26`: Header Defaults Buttons, Conditional Visibility, and About Pane Card Hierarchy Polish
  - `SPEC-26-01`: Shortcuts, General, and Mouse panes header Defaults button with conditional visibility (`UC-4`, `UC-14`)
  - `SPEC-26-02`: About pane card hierarchy restructuring and Troubleshooting button redesign

### Architectural & Scope Boundaries (SPEC-26)

1. **Header Defaults buttons and conditional visibility (SPEC-26-01):**
   - Provide consistent `↺ Defaults` button in Shortcuts, General, and Mouse pane headers.
   - Visibility is strictly derived from whether pane-owned draft values differ from `Config::default()`.
   - General owns `auto_start`, `switcher.visual_enabled`, and `switcher.visual_hold_delay_ms` (`check_updates` is preserved).
   - Mouse owns `enabled` and all four presets (`thumb_back`, `thumb_forward`, `tilt_left`, `tilt_right`).
   - Shortcuts owns chords, enable flags, four snap percentages, and `stack_width_percent`.
   - Restores are in-memory draft mutations only, clearing pending percentage text and incrementing `revert_generation`.

2. **About pane card hierarchy & Troubleshooting redesign (SPEC-26-02):**
   - Card 3 groups Support/Source action buttons followed by a full-bleed `CardDivider` and Troubleshooting & Recovery.
   - Troubleshooting reset action retains subtle secondary card styling (`Palette.bg_subtle`, `Palette.stroke_card`, text in `Palette.signal_error`) without an error-filled background.
   - Card 4 is the final About card, containing publisher attribution, link, GPL-3.0 notice, in-process update/privacy disclosure, copyright, and NOTICE pointer.
   - Confirmation overlay modal contract preserved with background interaction blocking.

3. **Coordination & Review:**
   - In-session default coordinator executes coding and self-review.
   - Shelled-out peer review via Kiro GPT-5.6 Terra (`kiro-cli chat --model gpt-5.6-terra --effort high -a --no-interactive "<prompt>"`).
   - Single shared worktree (`.`) and unified build target.

## Why

Unattended delivery loop authorised by the product owner under WDI Method G5 Release gate.

## Cost

One autopilot cycle of unattended implementation and peer review.
