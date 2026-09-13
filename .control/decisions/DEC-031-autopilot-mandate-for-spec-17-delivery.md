---
type: mandate
id: DEC-031
status: applied
touches:
  - .control/decisions/DEC-031-autopilot-mandate-for-spec-17-delivery.md
  - .control/memlog/autopilot-DEC-031.md
  - .control/registry/decisions.yaml
  - .control/registry/defects.yaml
  - .control/registry/specs.yaml
  - .control/decisions/DEC-026-cross-monitor-switcher-shift-cycle-and-window-eligibility.md
  - .scratch/smoke-dec-031.md
  - .scratch/spec-17-blind-backward-cycling-multi-window/issues/01-defect-def-19-blind-backward-cycling-multi-window.md
  - 3p.md
  - crates/daemon/src/cycling/mod.rs
  - crates/daemon/src/worker.rs
supersedes: null
superseded_by: null
created: '2026-09-13'
accepted_by: kodesh87 (Product Owner, in session), 2026-09-13
---

# DEC-031 — Autopilot mandate for SPEC-17 delivery

## Decision

`wdi-autopilot` is mandated to carry Wira Desk from its current position — G1-G4 passed, `SPEC-1` through
`SPEC-16` closed, `SPEC-17` open with its ticket planned and peer-reviewed — through every runnable
`FR`/spec/ticket to completion, without further owner check-ins until Finish or a parked row. Parameters
live on this decision's row in `decisions.yaml` under `mandate:`.

In scope:
- `SPEC-17`: Blind backward cycling multi-window traversal (`DEF-19`, ticket `SPEC-17-01`)

### Architectural & Scope Boundaries (SPEC-17)

1. **Defect DEF-19 Blind Backward Cycling Multi-Window Traversal (SPEC-17-01):**
   - Fix oscillation between top two windows (`A <-> B`) during rapid blind backward cycling (`Alt + Shift + ~` or `Win + Shift + ~`).
   - Implement cycling session tracking (stateful visitation history/cursor) across consecutive blind backward cycle activations so repeated taps traverse all eligible windows (`A -> D -> C -> B -> A`).
   - Reconcile void planning assumption from DEC-026 §B-4 (stateless rotation without reverse) which failed under dynamic Windows Z-order remove-and-prepend behavior.
   - Maintain visual switcher backward navigation (`Direction::Backward` / `Command::SwitcherArmPrev` / `Command::SwitcherPrev`) contract and card alignment.
   - Enforce dynamic Z-order mutation test coverage in both unit and worker suites.

Execution constraints specified by owner:
1. Coding directly implemented by coordinator with mandatory self code review.
2. Code review performed 2 times: self code review (in-session default, no separate dispatch) + independent peer review shell-out via:
   `$env:ANTHROPIC_BASE_URL = $null; $env:ANTHROPIC_API_KEY = $null; $env:ANTHROPIC_AUTH_TOKEN = $null; $env:CLAUDE_CONFIG_DIR = "$HOME\.claude"; claude --model claude-sonnet-5 --effort high --dangerously-skip-permissions -p "<prompt>"`
3. Automated smoke testing executed by agent.
4. Peer review for coding / document review / peer analysis and document repair delegated to Claude Sonnet 5 shell-out — mandate: if draft/code touches architecture spine, SRS, SDD, or SPEC, run `wdi-review` and edit documents directly without prompting.
5. Application build/run: one shared worktree used across all build/run operations in this run (`worktree: .`). Before peer review joins build/run, inform it of the current worktree path — peer MUST NOT create its own worktree, and MUST NOT build/run concurrently while coordinator is building/running.
