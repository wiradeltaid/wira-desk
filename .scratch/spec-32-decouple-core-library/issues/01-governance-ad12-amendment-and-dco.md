---
id: SPEC-32-01
title: Governance, AD-12 amendment DEC-046, and DCO 1.1 in CONTRIBUTING.md
spec: SPEC-32
status: open
depends_on: []
---

# SPEC-32-01 — Governance, AD-12 amendment DEC-046, and DCO 1.1 in CONTRIBUTING.md

## Scope
1. Draft and record `DEC-046` in `.control/decisions/` formally amending `AD-12` ("Cargo Workspace: Three Crates") to a Four-Crates architecture introducing `crates/core` (`wiradesk-core`).
2. Register `core` in `.control/registry/components.yaml` with `mode: deep` and `risk_accepted: medium`.
3. Update `.constitution/project/codebase-stack-guide.md` with explicit architectural boundaries forbidding Win32 FFI and OS I/O inside `crates/core`.
4. Add DCO 1.1 (*Developer Certificate of Origin*) clause to `CONTRIBUTING.md` with explicit *inbound = outbound per-crate* licensing rules (`core` = MIT, `daemon`/`settings` = GPL-3.0-only).

## Verification
- `python .control/validate.py --baseline` passes with 0 findings.
- `CONTRIBUTING.md` contains clear DCO sign-off guidelines.
