---
type: mandate
id: DEC-032
status: accepted
touches:
  - .control/decisions/DEC-032-autopilot-mandate-for-spec-18-delivery.md
  - .control/memlog/autopilot-DEC-032.md
  - .control/registry/decisions.yaml
  - .control/registry/defects.yaml
  - .control/registry/specs.yaml
  - .scratch/smoke-dec-032.md
  - .scratch/spec-18-blind-backward-cycling-session-lifecycle/issues/01-defect-def-20-blind-backward-cycling-session-lifecycle.md
  - 3p.md
  - crates/daemon/src/worker.rs
supersedes: null
superseded_by: null
created: '2026-09-14'
accepted_by: kodesh87 (Product Owner, in session), 2026-09-14
---

# DEC-032 — Autopilot mandate for SPEC-18 delivery

## Decision

`wdi-autopilot` is mandated to carry Wira Desk from its current position — G1-G4 passed, `SPEC-1` through
`SPEC-17` closed, `SPEC-18` open with its ticket planned and peer-reviewed — through every runnable
`FR`/spec/ticket to completion, without further owner check-ins until Finish or a parked row. Parameters
live on this decision's row in `decisions.yaml` under `mandate:`.

In scope:
- `SPEC-18`: Blind backward cycling session lifecycle and arm/disarm decoupling (`DEF-20`, ticket `SPEC-18-01`)

### Architectural & Scope Boundaries (SPEC-18)

1. **Defect DEF-20 Blind Backward Cycling Session Lifecycle & Arm/Disarm Decoupling (SPEC-18-01):**
   - Decouple `BackwardCycleSession` reset from `Command::SwitcherArm | Command::SwitcherArmPrev` and `Command::SwitcherDisarm` in `crates/daemon/src/worker.rs`.
   - Preserve legitimate session termination boundaries: forward cycling (`Command::Cycle`), visual switcher overlay commit or cancel (`Command::SwitcherCommit` and `Command::SwitcherCancel`), focus departure to untracked window/app, and 2000 ms session timeout.
   - Add end-to-end worker integration tests verifying consecutive rapid backward taps (`SwitcherArmPrev` -> `SwitcherDisarm` -> `CyclePrev`) across dynamic self-mutating Z-order.
   - Maintain visual switcher overlay invocation and modifier release commit behavior.

Execution constraints specified by owner:
1. Coding directly implemented by coordinator with mandatory self code review.
2. Code review performed 2 times: self code review (in-session default, no separate dispatch) + independent peer review shell-out via:
   `$env:ANTHROPIC_BASE_URL = $null; $env:ANTHROPIC_API_KEY = $null; $env:ANTHROPIC_AUTH_TOKEN = $null; $env:CLAUDE_CONFIG_DIR = "$HOME\.claude"; claude --model claude-sonnet-5 --effort high --dangerously-skip-permissions -p "<prompt>"`
3. Automated smoke testing executed by agent.
4. Peer review for coding / document review / peer analysis and document repair delegated to Claude Sonnet 5 shell-out — mandate: if draft/code touches architecture spine, SRS, SDD, or SPEC, run `wdi-review` and edit documents directly without prompting.
5. Application build/run: one shared worktree used across all build/run operations in this run (`worktree: .`). Before peer review joins build/run, inform it of the current worktree path — peer MUST NOT create its own worktree, and MUST NOT build/run concurrently while coordinator is building/running.
