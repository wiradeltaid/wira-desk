---
type: decision
id: DEC-046
status: accepted
touches:
  - .control/decisions/DEC-046-workspace-four-crates-and-core-decoupling.md
  - .control/registry/decisions.yaml
  - .control/registry/components.yaml
  - .control/registry/specs.yaml
  - .how/_platform/ARCHITECTURE-SPINE.md
  - .constitution/project/codebase-stack-guide.md
  - CONTRIBUTING.md
  - Cargo.toml
supersedes: null
superseded_by: null
created: '2026-10-16'
accepted_by: Wira (Product Owner, in session), 2026-10-16
---

# DEC-046 — AD-12 amended from Three Crates to Four Crates: Decoupling Platform-Agnostic Core into `crates/core`

## Context
`AD-12` in `.how/_platform/ARCHITECTURE-SPINE.md` currently binds the project to a 3-crate architecture: `daemon` (`wiradesk.exe`), `settings` (`wiradesk-settings.exe`), and `shared` (`windows-sys` and configuration types).

Over time, ~4.400 lines of pure domain logic (geometry planners for halves/thirds/stacking, cycling selection state machines, switcher layout calculations, and command representations) have become entangled with Win32 FFI calls in `daemon` and `shared`. This entanglement prevents headless unit testing, couples pure geometry to Windows OS APIs, and prevents reuse as a platform-agnostic library.

## Decision
1. **Amend AD-12 to a Four-Crate Workspace:**
   - `crates/core` (`wiradesk-core`): Pure domain logic, `#![forbid(unsafe_code)]`, zero OS I/O, licensed under MIT.
   - `crates/daemon` (`wiradesk.exe`): Windows background daemon, Win32 FFI hooks, tray, and DWM integration, licensed under GPL-3.0-only.
   - `crates/settings` (`wiradesk-settings.exe`): Settings GUI built with Slint, licensed under GPL-3.0-only.
   - `crates/shared`: Windows-specific shared utilities (WinHttp updater, DPAPI, installer helpers), licensed under GPL-3.0-only.
2. **Component Registry:** Register `core` in `.control/registry/components.yaml` with `mode: deep` and `risk_accepted: medium`.
3. **Inbound Licensing & DCO:** Update `CONTRIBUTING.md` with DCO 1.1 (*Developer Certificate of Origin*) enforcing *inbound = outbound per-crate* (`core` = MIT, `daemon`/`settings`/`shared` = GPL-3.0-only).

## Why
Decoupling pure mathematics and domain selection algorithms from OS-specific hooks enforces clean hexagonal architecture (*sans-IO*), enables headless testing across multiple compilation targets (including `aarch64-apple-darwin`), and ensures zero regression for the existing Windows application.

## Cost
Refactoring workspace boundaries across 6 phased tracer-bullet tickets (SPEC-32-01 to SPEC-32-06).
