---
id: SPEC-32-02
title: Adopt DCO 1.1 and per-crate inbound=outbound licensing
spec: SPEC-32
status: closed
depends_on: [SPEC-32-01]
---

# SPEC-32-02 — Adopt DCO 1.1 and per-crate inbound=outbound licensing

## Scope
1. Update `CONTRIBUTING.md` to introduce Developer Certificate of Origin 1.1 (`Signed-off-by` required via `git commit -s`).
2. Establish explicit per-path *inbound = outbound* licensing rules:
   - `crates/core/`: MIT.
   - `crates/daemon/`, `crates/settings/`, `crates/shared/`, and root: `GPL-3.0-only`.
   - Contributions to `crates/core/` must be free of copyleft restrictions to guarantee universal reusability across projects.
3. Place official Linux Foundation DCO 1.1 text verbatim in root file `DCO`.

## Verification
- `CONTRIBUTING.md` contains clear DCO sign-off guidelines, amend/rebase instructions, and per-path licensing table.
- Root `DCO` file exists and matches official DCO 1.1 specification.
