---
type: mandate
id: DEC-035
status: applied
touches:
  - .control/decisions/DEC-035-autopilot-mandate-for-spec-21-delivery.md
  - .control/memlog/autopilot-DEC-035.md
  - .control/registry/decisions.yaml
  - .control/registry/specs.yaml
  - .scratch/spec-21-installer-hardening-and-downgrade-prevention/SPEC.md
  - .scratch/spec-21-installer-hardening-and-downgrade-prevention/issues/01-defect-def-23-installer-downgrade-prevention-and-safety.md
  - packaging/wiradesk.iss
  - scripts/verify-installer-safety.ps1
  - .github/workflows/ci.yml
  - .how/settings/SDD-settings.md
  - .what/settings/SRS-settings.md
  - 3p.md
  - docs/3p.md
supersedes: null
superseded_by: null
created: '2026-09-14'
accepted_by: kodesh87 (Product Owner, in session), 2026-09-14
---

# DEC-035 — Autopilot mandate for SPEC-21 delivery

## Decision

`wdi-autopilot` is mandated to carry Wira Desk from its current position — G1-G4 passed, `SPEC-1` through
`SPEC-20` closed, `SPEC-21` open with its ticket planned and peer-reviewed — through every runnable
`FR`/spec/ticket to completion, without further owner check-ins until Finish or a parked row. Parameters
live on this decision's row in `decisions.yaml` under `mandate:`.

In scope:
- `SPEC-21`: Installer hardening — downgrade prevention, installation path confirmation, and fail-closed process termination (`DEF-23`, ticket `SPEC-21-01`)

### Architectural & Scope Boundaries (SPEC-21)

1. **Defect DEF-23 Installer Downgrade Prevention, Installation Path Confirmation, and Fail-Closed Process Shutdown (SPEC-21-01):**
   - In `packaging/wiradesk.iss`:
     - Compile-time version guard via ISPP validating `{#AppVersion}` strictly against decimal `major.minor.patch` (rejecting 4-component, empty/non-decimal, `v`, `-`, or `+` metadata).
     - Downgrade prevention in `InitializeSetup`: query `HKLM\Software\Microsoft\Windows\CurrentVersion\Uninstall\{7E4F9C21-6B3D-4A88-9F14-2C5E8D0A1B73}_is1` in explicit 64-bit registry view (`KEY_WOW64_64KEY`), restoring view after. Compare parsed decimal numeric triplets. Reject newer installed version or blank/malformed version before extraction. Silent mode logs and exits non-zero without modal dialogs. Allow missing key, older version, equal version.
     - Transparent path confirmation in `UpdateReadyMemo`: format `{app}`, `{userappdata}\WiraDesk` for configs/logs, and optional `WiraDesk` task, stating clearly that Setup neither creates nor enables auto-start, and not calling it a service.
     - Fail-closed process termination in `PrepareToInstall`: ensure daemon polite `WM_CLOSE` + bounded `/F` fallback and Settings non-forced close verify processes have actually exited. If either executable remains, return descriptive error from `PrepareToInstall` to abort before `[Files]` extraction.
   - In `scripts/verify-installer-safety.ps1`:
     - Test harness invoked by CI build job running real compiled installer against isolated `HKLM` AppId fixtures, verifying exit codes and filesystem states across decision table (no-key, installed-older, installed-equal, installed-newer, malformed) plus ISPP compile-time rejections of invalid versions, cleaning up all fixtures in `finally`.
   - In `.github/workflows/ci.yml`:
     - Run `scripts/verify-installer-safety.ps1` in the CI build job after ordinary installer compilation.

Execution constraints specified by owner:
1. Coding directly implemented by coordinator with mandatory self code review.
2. Code review performed 2 times: self code review (in-session default, no separate dispatch) + independent peer review shell-out via:
   `kiro-cli chat --model gpt-5.6-terra --effort high -a --no-interactive "<prompt>"`
3. Automated smoke testing executed by agent.
4. Peer review for coding / review dokumen / peer analisa and perbaikan dokumen serahkan ke Kiro GPT-5.6 Terra shell-out — mandate: if draft/code touches architecture spine, SRS, SDD, or SPEC, run `wdi-review` and edit documents directly without prompting.
5. Application build/run: one shared worktree used across all build/run operations in this run (`worktree: .`). Before peer review joins build/run, inform it of the current worktree path — peer MUST NOT create its own worktree, and MUST NOT build/run concurrently while coordinator is building/running.

## Why

Unattended delivery of SPEC-21 defect remediation across the full cycle from ticket implementation, testing, code review, documentation drift reconciliation, and release gating.

## Cost if wrong

Iteration stops at capacity or blocked state; changes roll back or park; owner intervention required at finish.
