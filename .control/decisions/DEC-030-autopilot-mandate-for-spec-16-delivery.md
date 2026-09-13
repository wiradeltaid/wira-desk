---
type: mandate
id: DEC-030
status: applied
touches:
  - .control/decisions/DEC-030-autopilot-mandate-for-spec-16-delivery.md
  - .control/memlog/autopilot-DEC-030.md
  - .control/registry/decisions.yaml
  - .control/registry/specs.yaml
  - .scratch/smoke-dec-030.md
  - .scratch/spec-16-visual-switcher-lifecycle-and-settings-polish/SPEC.md
  - .scratch/spec-16-visual-switcher-lifecycle-and-settings-polish/issues/01-settings-stepper-centering-and-em-dash-removal.md
  - .scratch/spec-16-visual-switcher-lifecycle-and-settings-polish/issues/02-start-menu-suppression-on-visual-switcher-commit.md
  - .scratch/spec-16-visual-switcher-lifecycle-and-settings-polish/issues/03-modern-winui-popup-bridge-and-helper-surface-exclusion.md
  - 3p.md
  - crates/daemon/src/hook.rs
  - crates/daemon/src/worker.rs
  - crates/daemon/src/cycling/mod.rs
  - crates/daemon/src/cycling/eligibility.rs
  - crates/settings/src/app.rs
  - crates/settings/ui/components/key_check.slint
  - crates/settings/ui/panes/about_pane.slint
  - crates/settings/ui/panes/general_pane.slint
  - crates/settings/ui/panes/shortcuts_pane.slint
supersedes: null
superseded_by: null
created: '2026-09-13'
accepted_by: kodesh87 (Product Owner, in session), 2026-09-13
---

# DEC-030 — Autopilot mandate for SPEC-16 delivery

## Decision

`wdi-autopilot` is mandated to carry Wira Desk from its current position — G1-G4 passed, `SPEC-1` through
`SPEC-15` closed, `SPEC-16` open with its tickets planned and peer-reviewed — through every runnable
`FR`/spec/ticket to completion, without further owner check-ins until Finish or a parked row. Parameters
live on this decision's row in `decisions.yaml` under `mandate:`.

In scope:
- `SPEC-16`: Visual switcher lifecycle and settings polish (3 tickets: SPEC-16-01 through SPEC-16-03)

### Architectural & Scope Boundaries (SPEC-16)

1. **Settings Stepper Centering & Em-Dash Cleanliness (SPEC-16-01):**
   - In `GeneralPane`, center the stepper button cluster vertically by wrapping in `VerticalLayout { alignment: center; }` matching sibling text column layout, preserving `alignment: end` main-axis right pinning.
   - Replace all 6 user-visible em dashes (`—`, U+2014) in Slint string literals with standard hyphens/dashes or contextual placeholder copy (`"None yet"` in `key_check.slint:142`). Leave comments and the accepted en dash (`–`, U+2013 in `(100-500 ms)`) intact.
2. **Start Menu Suppression at Arm/Open Time (SPEC-16-02):**
   - Move Start Menu suppression from release/commit time to arm time (`SwitcherArm`) and to the entry of `open_visual_switcher` preceding both early returns. Delete dead release-edge call site in `Command::SwitcherCommit`.
   - Never inject `SendInput` from inside the low-level hook thread (safety invariant: avoids activation races).
3. **WinUI 3 Popup Bridge Exclusion (SPEC-16-03):**
   - Exclude Modern WinUI 3 popup bridge surfaces (`PopupWindowSiteBridge`) in `is_helper_surface()`, covering both production and reference policies without altering `WindowFacts` title/owner contract. Keep existing SPEC-15-02 deferral assertions green.

Execution constraints specified by owner:
1. Coding directly implemented by coordinator with mandatory self code review.
2. Code review performed 2 times: self code review (in-session default, no separate dispatch) + independent peer review shell-out via:
   `$env:ANTHROPIC_BASE_URL = $null; $env:ANTHROPIC_API_KEY = $null; $env:ANTHROPIC_AUTH_TOKEN = $null; $env:CLAUDE_CONFIG_DIR = "$HOME\.claude"; claude --model claude-sonnet-5 --effort high --dangerously-skip-permissions -p "<prompt>"`
3. Automated smoke testing executed by agent.
4. Peer review for coding / document review / peer analysis and document repair delegated to Claude Sonnet 5 shell-out — mandate: if draft/code touches architecture spine, SRS, SDD, or SPEC, run `wdi-review` and edit documents directly without prompting.
5. Application build/run: one shared worktree used across all build/run operations in this run (`worktree: .`). Before peer review joins build/run, inform it of the current worktree path — peer MUST NOT create its own worktree, and MUST NOT build/run concurrently while coordinator is building/running.
