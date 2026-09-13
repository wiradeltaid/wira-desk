---
type: mandate
id: DEC-029
status: applied
touches:
  - .control/decisions/DEC-029-autopilot-mandate-for-spec-15-delivery.md
  - .control/memlog/autopilot-DEC-029.md
  - .control/registry/decisions.yaml
  - .control/registry/specs.yaml
  - .control/questions/assumptions.md
  - .what/_prd/wira-desk/prd.md
  - .scratch/smoke-dec-029.md
  - .scratch/spec-15-visual-switcher-polish-and-lifecycle/SPEC.md
  - .scratch/spec-15-visual-switcher-polish-and-lifecycle/issues/01-settings-general-and-about-panes-layout-polish.md
  - .scratch/spec-15-visual-switcher-polish-and-lifecycle/issues/02-helper-and-popup-host-window-eligibility-sanitization.md
  - .scratch/spec-15-visual-switcher-polish-and-lifecycle/issues/03-active-window-card-inclusion-and-single-window-switcher-triggering.md
  - .scratch/spec-15-visual-switcher-polish-and-lifecycle/issues/04-cycle-activation-timing-and-hold-threshold-decoupling.md
  - 3p.md
  - crates/daemon/src/arrangement/mod.rs
  - crates/daemon/src/cycling/eligibility.rs
  - crates/daemon/src/cycling/mod.rs
  - crates/daemon/src/cycling/source.rs
  - crates/daemon/src/hook.rs
  - crates/daemon/src/switcher/mod.rs
  - crates/daemon/src/worker.rs
  - crates/settings/src/app.rs
  - crates/settings/ui/panes/about_pane.slint
  - crates/settings/ui/panes/general_pane.slint
  - crates/shared/src/commands.rs
  - crates/shared/src/config.rs
supersedes: null
superseded_by: null
created: '2026-09-13'
accepted_by: kodesh87 (Product Owner, in session), 2026-09-13
---

# DEC-029 — Autopilot mandate for SPEC-15 delivery

## Decision

`wdi-autopilot` is mandated to carry Wira Desk from its current position — G1-G4 passed, `SPEC-1` through
`SPEC-14` closed, `SPEC-15` open with its tickets planned — through every runnable
`FR`/spec/ticket to completion, without further owner check-ins until Finish or a parked row. Parameters
live on this decision's row in `decisions.yaml` under `mandate:`.

In scope:
- `SPEC-15`: Visual switcher polish and lifecycle (4 tickets: SPEC-15-01 through SPEC-15-04)

### Architectural Timing Contract & Lifecycle Boundaries (SPEC-15-04)

1. **Option A Decoupled Timing Contract:**
   - When `visual_enabled` is true, blind cycle activation moves from chord keydown to the chord's committing edge: main-key release below `visual_hold_delay_ms`. Zero `Command::Cycle` is enqueued at keydown; holding past the threshold opens the visual switcher overlay while the pre-hold foreground window remains foreground.
   - When `visual_enabled` is false, blind cycling continues to fire on keydown; latency profile and behavior are completely untouched.
2. **SM-1 Measurement Point Restatement:**
   - The success metric SM-1 in `.what/_prd/wira-desk/prd.md` is restated so that focus transfer latency (< 1 ms) is measured following chord release (the chord's committing edge below hold threshold when visual switcher is enabled, or keypress when visual switcher is disabled).
3. **Auto-Repeat Suppression on Chord Main Key:**
   - The low-level hook suppresses synthetic auto-repeat `WM_KEYDOWN` events for `switcher_main_vk` while the key is already latched down, preventing runaway selection advancement when holding the cycle chord with the overlay open.
4. **Single Source for Hold Threshold Derivation:**
   - Both Hook and Worker threads derive hold threshold via `shared::config::SwitcherConfig::clamp_hold_delay`, preventing timing skew or race conditions between actors.

Execution constraints specified by owner:
1. Coding directly implemented by coordinator with mandatory self code review.
2. Code review performed 2 times: self code review (in-session default, no separate dispatch) + independent peer review shell-out via:
   `$env:ANTHROPIC_BASE_URL = $null; $env:ANTHROPIC_API_KEY = $null; $env:ANTHROPIC_AUTH_TOKEN = $null; $env:CLAUDE_CONFIG_DIR = "$HOME\.claude"; claude --model claude-sonnet-5 --effort high --dangerously-skip-permissions -p "<prompt>"`
3. Automated smoke testing executed by agent.
4. Peer review for coding / document review / peer analysis and document repair delegated to Claude Sonnet 5 shell-out — mandate: if draft/code touches architecture spine, SRS, SDD, or SPEC, run `wdi-review` and edit documents directly without prompting.
5. Application build/run: one shared worktree used across all build/run operations in this run (`worktree: .`). Before peer review joins build/run, inform it of the current worktree path — peer MUST NOT create its own worktree, and MUST NOT build/run concurrently while coordinator is building/running.
