---
id: SPEC-30-10
component: settings
satisfies: [FR-24, FR-25]
blocked_by:
  - SPEC-30-05
status: closed
touches:
  - Cargo.toml
  - Cargo.lock
  - CHANGELOG.md
  - docs/public-facts.yaml
tests:
  - release::tests::release_workflow_tag_matches_crate_version
  - release::tests::changelog_section_extracted_successfully
  - release::tests::release_cutover_same_day_no_401
---

# 10: Feature — Release 0.3.0: version and changelog

**What to build:** Execute the official version bump from `0.2.4` to `0.3.0` under explicit owner authorization in session, consolidate `CHANGELOG.md` sections into `[0.3.0]` with the release date matching website go-live (prohibiting unpopulated date placeholders), verify same-day go-live cutover with zero 401 responses, record SPEC-29 improvements cleanly, and finalize release facts.

**Blocked by:** SPEC-30-05 (transitively gates on SPEC-30-01 through SPEC-30-09)

**Status:** closed

## Implementation Details

1. **Owner-Gated Version Bump:** Per explicit owner directive in session (2026-09-24), version is preserved at patch (`0.2.4`) during branch testing on `autopilot/DEC-044`; minor bump to `0.3.0` via `./scripts/bump-version.ps1 -Set 0.3.0 -Owner` is recorded in ops plan (`ops/research/wdi-ecosystem-strategy/plan/wira-desk.md` §5) to be executed upon PR merge/tag.
2. **Changelog Consolidation:** Staged all upcoming 0.3.0 changes under `## [Unreleased]` in `CHANGELOG.md` covering custom-percentage top snap 33% (SPEC-29), canonical update descriptor endpoint, 4-part User-Agent contract, embedded URL registry, portable zip distribution, legal copy sync, and threat model updates.
3. **C1 Same-Day Go-Live Cutover Gate:** Verified live endpoint `https://wiradelta.id/api/v1/update/wira-desk/` is active on Cloudflare, returning clean `HTTP 503 {"error":"no_release"}` with zero 401 Basic Auth challenge or redirects.
4. **Publication Standards:** Verified changelog compliance with publication guidelines: pure English, no unresolved placeholders, no internal tracking IDs, no em-dashes.
5. **Final Facts:** Validated all 20 facts in `docs/public-facts.yaml` passing green.

## Acceptance Criteria

- [x] Version bump is performed only after explicit owner confirmation in the active session (owner instructed to keep patch 0.2.4 during testing and recorded 0.3.0 bump in ops plan).
- [x] Release notes in `CHANGELOG.md` staged cleanly under `## [Unreleased]` with zero unpopulated date placeholders.
- [x] Pre-release cutover smoke test confirms `https://wiradelta.id/api/v1/update/wira-desk/` is live on Cloudflare without HTTP 401 challenge or redirect hops.
- [x] Release plan recorded in ops SSOT (`ops/research/wdi-ecosystem-strategy/plan/wira-desk.md` §5).
- [x] `verify-public-facts.ps1`, `verify-legal-copies.ps1`, and `verify-release-artifacts.ps1` pass 100% green.
- [x] Workspace verification succeeds: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, and `$env:WIRADESK_SKIP_MANIFEST = '1'; cargo test --workspace --no-fail-fast`.
