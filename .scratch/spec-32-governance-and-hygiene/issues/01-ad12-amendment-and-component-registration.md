---
id: SPEC-32-01
title: Ratify DEC-046 amending AD-12 and register core component
spec: SPEC-32
status: closed
depends_on: []
---

# SPEC-32-01 — Ratify DEC-046 amending AD-12 and register core component

## Scope
1. Draft and record `DEC-046` in `.control/decisions/` formally amending `AD-12` ("Cargo Workspace: Three Crates") to a Four-Crates architecture introducing `crates/core` (`wiradesk-core`).
2. Add explicit **Relicense Authority** section in `DEC-046` stating the sole copyright holder (Wira Sihombing / wiradeltaid, covering all 506 commits) authorizes licensing the extracted domain engine under the MIT license without copyleft contamination.
3. Register `core` in `.control/registry/components.yaml` with `mode: deep` and `risk_accepted: medium`, along with container `core` and logical components (`LC-geometry-engine`, `LC-cycling-state-machine`, `LC-switcher-layout-model`, `LC-input-sink`).
4. Update `.how/_platform/ARCHITECTURE-SPINE.md` and `.constitution/project/codebase-stack-guide.md` with explicit architectural boundaries forbidding Win32 FFI and OS I/O inside `crates/core`.

## Verification
- `uv run .constitution/method/scripts/validate.py --baseline` passes with 0 findings.
- `components.yaml` matches registry schema and has clean container mappings.
