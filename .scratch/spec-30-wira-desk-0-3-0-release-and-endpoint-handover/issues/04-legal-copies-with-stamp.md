---
id: SPEC-30-04
component: settings
satisfies: [FR-24, FR-25]
blocked_by: [SPEC-30-09]
status: closed
touches:
  - PRIVACY.id.md
  - PRIVACY.md
  - SECURITY.id.md
  - SECURITY.md
  - scripts/verify-legal-copies.ps1
tests:
  - scripts::tests::verify_legal_copies_and_stamps
---

# 04: Feature — Legal copies with stamp

**What to build:** Synchronize repository legal documents with the approved upstream ops sources, introducing `PRIVACY.id.md` and `SECURITY.id.md` alongside updated `PRIVACY.md` and `SECURITY.md`, each marked with the official copy stamp.

**Blocked by:** SPEC-30-09 (and requires upstream ops placeholders filled, effective date set to 0.3.0 release date, and GitHub private vulnerability reporting enabled)

**Status:** closed

## Implementation Details

1. **Prerequisites & Readiness:** Ensure all prerequisites are fulfilled prior to landing: H-01 merged (behavior accurate), ops endpoint deployed, upstream placeholders filled, and GitHub private vulnerability reporting enabled.
2. **Indonesian Canonical Documents:** Add `PRIVACY.id.md` and `SECURITY.id.md` containing the body text copied from ops sources, preceded by the official English copy stamp comment block:
   ```markdown
   <!-- Copied from the Wira Delta Indonesia legal source (wira-desk/privacy.id.md) on YYYY-MM-DD.
        Edit the source, then copy it here again. -->
   ```
3. **English Translation Documents:** Update `PRIVACY.md` and `SECURITY.md` with official English translations, including the translation header linking to the `.id.md` versions and the official copy stamp.
4. **Verification Script:** Implement `scripts/verify-legal-copies.ps1` to assert that all four files exist, begin with the required stamp, link appropriately between English and Indonesian versions, contain no unresolved placeholders (`<UPDATE-ENDPOINT`, `<TANGGAL`, `<GO-LIVE`), and contain zero local machine paths.

## Acceptance Criteria

- [x] Every guard and verification script is observed red on today's codebase before implementation, then green.
- [x] `PRIVACY.id.md`, `PRIVACY.md`, `SECURITY.id.md`, and `SECURITY.md` are present in repo root.
- [x] Each file carries the standardized copy stamp comment immediately below the top heading.
- [x] No unpopulated placeholders (`<UPDATE-ENDPOINT`, `<TANGGAL`, `<GO-LIVE`) remain.
- [x] `scripts/verify-legal-copies.ps1` passes cleanly in CI.
- [x] `scripts/verify-public-export.ps1` runs clean without findings.
- [x] Workspace verification succeeds: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, and `$env:WIRADESK_SKIP_MANIFEST = '1'; cargo test --workspace --no-fail-fast`.
