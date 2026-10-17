---
id: SPEC-32
title: Architecture Governance, AD-12 Amendment, DCO 1.1, and Hygiene Gate Extension
status: closed
size: S
release: "0.4.0"
created: '2026-10-17'
author: Wira Delta Indonesia
---

# SPEC-32 — Architecture Governance, AD-12 Amendment, DCO 1.1, and Hygiene Gate Extension

## Context & Objectives
This specification establishes the architectural governance and legal foundations required for the four-crate workspace refactoring. It formally amends `AD-12` in `.how/_platform/ARCHITECTURE-SPINE.md` via `DEC-046`, registers `core` as a Product Component in `.control/registry/components.yaml`, establishes Developer Certificate of Origin (DCO 1.1) in `CONTRIBUTING.md`, and extends the automated export hygiene verification gate in `scripts/verify-public-export.ps1` to prevent leakage of private repository names and commercial monetization vocabulary.

## Invariants & Guardrails
1. **Single Source of Truth:** `DEC-046` is recorded with explicit relicense authority by the sole copyright holder (Wira Sihombing).
2. **Inbound = Outbound Licensing:** Contributions to `crates/core/` are licensed strictly under MIT, while `daemon`, `settings`, and `shared` remain `GPL-3.0-only`.
3. **Publication Hygiene:** `scripts/verify-public-export.ps1` must block private commercial vocabulary and pass all checks on clean repository tree.

## Tracer-Bullet Tickets
- **SPEC-32-01:** Ratify DEC-046 amending AD-12 to Four Crates, register `core` in `components.yaml` and `codebase-stack-guide.md`, and record relicense authority clause.
- **SPEC-32-02:** Adopt DCO 1.1 and per-crate *inbound = outbound* licensing in `CONTRIBUTING.md` and root `DCO`.
- **SPEC-32-03:** Extend `scripts/verify-public-export.ps1` with `$checkPrivateCommercial` hygiene gate and verify against test fixtures.
