---
id: SPEC-30-09
component: settings
satisfies: [FR-24, FR-25]
blocked_by:
  - SPEC-30-03
  - SPEC-30-08
status: ready-for-agent
touches:
  - .what/business-rules.md
  - .how/_platform/c4-l1-system-context.md
  - .what/_prd/wira-desk/prd.md
  - .what/settings/SRS-settings.md
  - .what/window-management/SRS-window-management.md
tests:
  - corpus::tests::validate_corpus_reconciles_cleanly
---

# 09: Feature — Corpus follows the update-check change

**What to build:** Reconcile repository documentation and specifications with the implemented code, updating business rules (`BR-8`), C4 L1 architecture diagrams, PRD promises, and SRS requirements through canonical method skills.

**Blocked by:** SPEC-30-03, SPEC-30-08

**Status:** ready-for-agent

## Implementation Details

1. **Business Rules:** Update `BR-8` in `.what/business-rules.md` to reflect update checks targeting `https://wiradelta.id/api/v1/update/wira-desk/` without redirects, carrying the four-part User-Agent, with installer downloads pinned to GitHub Releases.
2. **Architecture Diagrams:** Update C4 L1 system context diagram in `.how/_platform/c4-l1-system-context.md` to show `wiradelta.id` as the second external system alongside GitHub.
3. **PRD & SRS:** Update PRD §3.12, §7, and component SRS documents to align requirement statements with the new update check reality.
4. **Tooling & Reconcile:** Scope strictly bounded to `BR-8`, C4 L1, PRD, and SRS files. Execute via `wdi-blueprint` and `wdi-product` skills upon explicit owner authorization per repository method policy, running `wdi-reconcile` to confirm zero remaining drift.

## Acceptance Criteria

- [ ] Every guard and corpus validator is observed red on drift before update, then green.
- [ ] `validate.py` passes with zero findings across the corpus.
- [ ] `wdi-reconcile` reports zero drift regarding update checks, network boundaries, and external endpoints.
- [ ] No `DEC-` records violated or left unreconciled.
- [ ] Workspace verification succeeds: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, and `$env:WIRADESK_SKIP_MANIFEST = '1'; cargo test --workspace --no-fail-fast`.
