---
type: mandate
id: DEC-036
status: accepted
touches:
  - .control/decisions/DEC-036-autopilot-mandate-for-spec-22-delivery.md
  - .control/memlog/autopilot-DEC-036.md
  - .control/registry/decisions.yaml
  - .control/registry/specs.yaml
  - .scratch/spec-22-installer-dialog-formatting-and-clean-config/issues/01-defect-def-24-installer-dialog-formatting-and-clean-config.md
  - packaging/wiradesk.iss
  - scripts/verify-installer-safety.ps1
  - 3p.md
  - docs/3p.md
supersedes: null
superseded_by: null
created: '2026-09-14'
accepted_by: kodesh87 (Product Owner, in session), 2026-09-14
---

# DEC-036 — Autopilot mandate for SPEC-22 delivery

## Decision

`wdi-autopilot` is mandated to carry Wira Desk from its current position — G1-G4 passed, `SPEC-1` through
`SPEC-21` closed, `SPEC-22` open with its ticket planned and peer-reviewed — through every runnable
`FR`/spec/ticket to completion, without further owner check-ins until Finish or a parked row. Parameters
live on this decision's row in `decisions.yaml` under `mandate:`.

In scope:
- `SPEC-22`: Installer dialog text formatting and clean-installation configuration clarity (`DEF-24`, ticket `SPEC-22-01`)

### Architectural & Scope Boundaries (SPEC-22)

1. **Defect DEF-24 Installer Dialog Text Formatting and Clean-Installation Configuration Clarity (SPEC-22-01):**
   - In `packaging/wiradesk.iss`:
     - Eliminate premature `#13#10` linebreaks that orphan trailing words onto separate lines in `InitializeSetup`.
     - Downgrade refusal joined string: `'If you wish to install an older version, please uninstall the current version first.'` without `#13#10` before `'version first.'`.
     - Invalid version refusal joined string: `'Setup cannot verify version compatibility. Please uninstall the current version before continuing.'` without `#13#10` before `'version before continuing.'`.
     - Update `UpdateReadyMemo` to state that `%APPDATA%\WiraDesk` is preserved if present and that Setup never bundles or overwrites user state.
     - Ensure copy distinguishes runtime initialization boundary: missing `config.toml` opens first-run onboarding, completion writes default config, and `wiradesk.log` is created only when the daemon writes a log entry (neither file bundled or eagerly created by Setup).
     - Ensure `[Files]` section continues to install zero bundled user configuration or log files.
   - In `scripts/verify-installer-safety.ps1`:
     - Update test harness to assert joined dialog sentence patterns, absence of orphaned linebreaks, Ready-page preservation disclosure, and `[Files]` zero-bundling invariant for `config.toml` and `wiradesk.log`.

Execution constraints specified by owner:
1. Coding directly implemented by coordinator with mandatory self code review.
2. Code review performed 2 times: self code review (in-session default, no separate dispatch) + independent peer review shell-out via:
   `kiro-cli chat --model gpt-5.6-terra --effort high -a --no-interactive "<prompt>"`
3. Automated smoke testing executed by agent.
4. Peer review for coding / review dokumen / peer analisa and perbaikan dokumen serahkan ke Kiro GPT-5.6 Terra shell-out — mandat: if draft/code touches architecture spine, SRS, SDD, or SPEC, run `wdi-review` and edit documents directly without prompting.
5. Application build/run: one shared worktree used across all build/run operations in this run (`worktree: .`). Before peer review joins build/run, inform it of the current worktree path — peer MUST NOT create its own worktree, and MUST NOT build/run concurrently while coordinator is building/running.

## Why

Unattended delivery of SPEC-22 defect remediation across the full cycle from ticket implementation, testing, code review, documentation drift reconciliation, and release gating.

## Cost if wrong

Iteration stops at capacity or blocked state; changes roll back or park; owner intervention required at finish.
