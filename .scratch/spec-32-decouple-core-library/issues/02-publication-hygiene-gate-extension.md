---
id: SPEC-32-02
title: Extend verify-public-export.ps1 with private commercial hygiene check
spec: SPEC-32
status: open
depends_on: [SPEC-32-01]
---

# SPEC-32-02 — Extend verify-public-export.ps1 with private commercial hygiene check

## Scope
1. Update `scripts/verify-public-export.ps1` to introduce `$checkPrivateCommercial`.
2. Block accidental leakage into the public repository of:
   - Private repository names (e.g. `wira-desk-mac`).
   - Commercial licensing vocabulary (e.g. `Lemon Squeezy`, `Paddle`, `serial_key`, `entitlement_token`).
   - Commercial pricing patterns (e.g. `$5`, `$9.99`, `Rp`).
3. Verify the gate against a known failing test fixture (red-to-green verification).

## Verification
- Gate correctly fails when private keywords are introduced into committed tracked files.
- Gate passes cleanly on baseline tree.
