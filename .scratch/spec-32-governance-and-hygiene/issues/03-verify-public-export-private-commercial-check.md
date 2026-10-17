---
id: SPEC-32-03
title: Extend verify-public-export.ps1 with checkPrivateCommercial hygiene gate
spec: SPEC-32
status: closed
depends_on: [SPEC-32-02]
---

# SPEC-32-03 — Extend verify-public-export.ps1 with checkPrivateCommercial hygiene gate

## Scope
1. Update `scripts/verify-public-export.ps1` to introduce check `$checkPrivateCommercial`.
2. Block accidental leakage into public git commits of:
   - Private repository names and non-public platform variants.
   - Commercial licensing vocabulary (e.g. merchant-of-record names, serial keys, entitlement tokens).
   - Commercial currency and pricing patterns.
3. Exempt internal planning archives (`.scratch/*`, `.archive/*`, `_bmad-output/*`) and skills from the rule.
4. Verify gate against a known failing test fixture (red-to-green verification).

## Verification
- Gate correctly fails when private commercial keywords are introduced into tracked files.
- Gate passes cleanly (11/11 checks pass) on baseline tree.
