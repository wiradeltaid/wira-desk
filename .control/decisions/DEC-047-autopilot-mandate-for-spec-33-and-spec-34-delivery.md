---
type: mandate
id: DEC-047
status: applied
touches:
  - .control/decisions/DEC-047-autopilot-mandate-for-spec-33-and-spec-34-delivery.md
  - .control/memlog/autopilot-DEC-047.md
  - .control/registry/decisions.yaml
  - .control/registry/specs.yaml
  - .scratch/spec-33-decouple-core-library/SPEC.md
  - .scratch/spec-33-decouple-core-library/issues/01-initialize-core-and-extract-geometry.md
  - .scratch/spec-33-decouple-core-library/issues/02-extract-switcher-layout-and-update-decision.md
  - .scratch/spec-33-decouple-core-library/issues/03-extract-cycling-selection-and-context-policy.md
  - .scratch/spec-33-decouple-core-library/issues/04-decouple-hook-decision-into-inputsink-and-lift-ipc.md
  - .scratch/spec-34-multitarget-and-core-release/SPEC.md
  - .scratch/spec-34-multitarget-and-core-release/issues/01-deny-toml-multitarget-and-apple-silicon-check.md
  - .scratch/spec-34-multitarget-and-core-release/issues/02-full-workspace-regression-and-version-bump.md
supersedes: null
superseded_by: null
created: '2026-10-17'
accepted_by: Wira (Product Owner, in session), 2026-10-17
---

# DEC-047 — Autopilot mandate for SPEC-33 and SPEC-34 delivery

## Decision

`wdi-autopilot` is mandated to carry Wira Desk from its current position — G1-G4 passed, `SPEC-1` through
`SPEC-32` closed, `SPEC-33` open with its 4 tickets planned and reviewed, `SPEC-34` open with its 2 tickets
planned and reviewed — through every runnable `FR`/spec/ticket to completion, without further owner check-ins
until Finish or a parked row. Parameters live on this decision's row in `decisions.yaml` under `mandate:`.

In scope:
- `SPEC-33`: Decouple Platform-Agnostic Core Library and InputSink Protocol
  - `SPEC-33-01`: Initialize `crates/core` (`wiradesk-core`) workspace member, MIT license, and extract pure geometry planners
  - `SPEC-33-02`: Extract switcher layout calculations (`switcher/{layout, selection}`) and clean split of descriptor parsing
  - `SPEC-33-03`: Extract cycling selection state machine (`cycling/{eligibility, selection, mod}`), `WindowFacts`, spatial evaluation, and `BypassPolicy`
  - `SPEC-33-04`: Decouple hook decisions into `InputSink` protocol (`Disposition`, `Effect`, `ModifierState`), and lift all 8 `PostMessageW(WM_APP_COMMAND_READY)` invocations to outer shell via `wake_worker` signal
- `SPEC-34`: Multi-Target Compilation, CI Matrix, and Core Crate Release
  - `SPEC-34-01`: Multi-target compile check across Windows, macOS (Apple Silicon), and Linux in CI matrix, with `cargo-deny` configuration
  - `SPEC-34-02`: Full workspace regression verification against 776 unit tests, package metadata, and version bump preparation

### Architectural & Scope Boundaries

1. **Zero Regression Invariant:**
   - Every ticket MUST preserve 100% green status across all existing unit tests in `crates/daemon`, `crates/settings`, and `crates/shared` (776 tests).
   - Use backward-compatible facades and re-exports in `crates/daemon` and `crates/shared` to prevent breaking existing call sites.

2. **Strict Sans-IO Boundary:**
   - `crates/core` (`wiradesk-core`) MUST compile with `#![forbid(unsafe_code)]` under the MIT license.
   - `crates/core` MUST NOT import `windows-sys`, native handles (`HWND`), filesystem, registry, network, or direct OS clock calls.

3. **Coordination, Verification & Review Panel:**
   - In-session coordinator (`ag/gemini-3.8-flash-high`) executes coding, self-review, and TDD red-to-green verification directly in the run worktree.
   - Independent peer review: Primary peer review via `terra` (Codex GPT-5.6-Terra), plus additional peer review via `deepseekpro` (CodeBuddy CN DeepSeek-V4-Pro).
   - Deep analyst: `none` (coordinator self-review for architecture/spec documents).
   - Run branch: `autopilot/DEC-047`.
   - Single worktree (`.`) and unified build target.

## Why

Autonomous delivery loop authorised by the product owner under WDI Method G5 Release gate.

## Cost

Autonomous autopilot cycles of unattended implementation, verification, and independent peer review.
