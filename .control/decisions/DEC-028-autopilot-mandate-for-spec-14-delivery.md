---
type: mandate
id: DEC-028
status: applied
touches:
  - .control/decisions/DEC-028-autopilot-mandate-for-spec-14-delivery.md
  - .control/memlog/autopilot-DEC-028.md
  - .control/registry/decisions.yaml
  - .control/registry/specs.yaml
  - .scratch/smoke-dec-028.md
  - .scratch/spec-14-visual-switcher-refinements-and-settings-polish/SPEC.md
  - 3p.md
  - crates/daemon/src/arrangement/mod.rs
  - crates/daemon/src/arrangement/win32.rs
  - crates/daemon/src/config.rs
  - crates/daemon/src/context/mod.rs
  - crates/daemon/src/context/virtual_desktop.rs
  - crates/daemon/src/cycling/eligibility.rs
  - crates/daemon/src/cycling/mod.rs
  - crates/daemon/src/hook.rs
  - crates/daemon/src/ring.rs
  - crates/daemon/src/switcher/layout.rs
  - crates/daemon/src/switcher/mod.rs
  - crates/daemon/src/switcher/overlay.rs
  - crates/daemon/src/switcher/selection.rs
  - crates/daemon/src/switcher/thumbnail.rs
  - crates/daemon/src/worker.rs
  - crates/settings/src/app.rs
  - crates/settings/src/persistence.rs
  - crates/settings/ui/panes/about_pane.slint
  - crates/settings/ui/panes/general_pane.slint
  - crates/shared/src/commands.rs
  - crates/shared/src/config.rs
  - crates/shared/src/shortcut.rs
supersedes: null
superseded_by: null
created: '2026-09-13'
accepted_by: kodesh87 (Product Owner, in session), 2026-09-13
---

# DEC-028 — Autopilot mandate for SPEC-14 delivery

## Decision

`wdi-autopilot` is mandated to carry Wira Desk from its current position — G1-G4 passed, `SPEC-1` through
`SPEC-13` closed, `SPEC-14` open with its tickets planned — through every runnable
`FR`/spec/ticket to completion, without further owner check-ins until Finish or a parked row. Parameters
live on this decision's row in `decisions.yaml` under `mandate:`.

In scope:
- `SPEC-14`: Visual switcher refinements and settings polish (6 tickets: SPEC-14-01 through SPEC-14-06)

Execution constraints specified by owner:
1. Coding directly implemented by coordinator with mandatory self code review.
2. Code review performed 2 times: self code review (in-session default, no separate dispatch) + independent peer review shell-out via:
   `$env:ANTHROPIC_BASE_URL = $null; $env:ANTHROPIC_API_KEY = $null; $env:ANTHROPIC_AUTH_TOKEN = $null; $env:CLAUDE_CONFIG_DIR = "$HOME\.claude"; claude --model claude-sonnet-5 --effort high --dangerously-skip-permissions -p "<prompt>"`
3. Automated smoke testing executed by agent.
4. Peer review for coding / document review / peer analysis and document repair delegated to Claude Sonnet 5 shell-out — mandate: if draft/code touches architecture spine, SRS, SDD, or SPEC, run `wdi-review` and edit documents directly without prompting.
5. Application build/run: one shared worktree used across all build/run operations in this run (`worktree: .`). Before peer review joins build/run, inform it of the current worktree path — peer MUST NOT create its own worktree, and MUST NOT build/run concurrently while coordinator is building/running.

## Why

The owner requested all open FR/ticket/specs to be completed unattended via loop with automated smoke testing,
dual code review (self + Claude Sonnet 5 high-effort peer review), and collision-safe builds.

## Cost

If the split turns out wrong, the mandate is superseded through a new `DEC-` — one setting changes, not
the whole run. Until Finish, the run branch is the only place ticket work lands, and `main` is reached
through exactly one PR the owner merges.
