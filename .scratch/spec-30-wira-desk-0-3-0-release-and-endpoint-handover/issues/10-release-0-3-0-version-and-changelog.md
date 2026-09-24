---
id: SPEC-30-10
component: settings
satisfies: [FR-24, FR-25]
blocked_by:
  - SPEC-30-05
status: ready-for-agent
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

**Status:** ready-for-agent

## Implementation Details

1. **Owner-Gated Version Bump:** Upon receiving explicit owner permission in session, execute `./scripts/bump-version.ps1 -Set 0.3.0 -Owner` to bump workspace crates and update `Cargo.lock`.
2. **Changelog Consolidation:** Replace `## [0.2.4] - 2026-09-15` with `## [0.3.0] - YYYY-MM-DD` using the concrete ISO date of the release, merging entries from `## [Unreleased]`, and adding SPEC-29 custom-percentage snap defaults (implemented on `main` in commit `5df5b49` but omitted from the 0.2.4 section) without internal identifiers.
3. **C1 Same-Day Go-Live Cutover Gate:** Release 0.3.0 and website go-live occur on the same day per owner Q1 decision. Before tagging, verify that `https://wiradelta.id/api/v1/update/wira-desk/` is live, deployed, and responds without HTTP 401 Basic Auth challenge or redirects, ensuring smooth user experience.
4. **Publication Standards:** Ensure changelog complies with publication guidelines: pure English, no unresolved placeholders (`<release-date>`, `<TANGGAL>`), no internal tracking IDs (`SPEC-`, `DEC-`, `PW-`, `WDK-`), no em-dashes, and no internal chronology framing 0.2.4 as a released version.
5. **Final Facts:** Finalize `product_version` and installer size measurements in `docs/public-facts.yaml`.

## Acceptance Criteria

- [ ] Version bump is performed only after explicit owner confirmation in the active session.
- [ ] Release date in `CHANGELOG.md` is populated with a concrete ISO date matching the website go-live date with zero `<release-date>` placeholders remaining.
- [ ] Pre-release cutover smoke test confirms `https://wiradelta.id/api/v1/update/wira-desk/` returns HTTP 200 with the 0.3.0 descriptor without HTTP 401 challenge or redirect hops.
- [ ] `release.yml` tag-versus-crate check and CHANGELOG extraction steps run locally and pass cleanly for `v0.3.0`.
- [ ] `verify-public-facts.ps1`, `verify-installer-safety.ps1`, and `verify-release-artifacts.ps1` pass 100% green.
- [ ] Workspace verification succeeds: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, and `$env:WIRADESK_SKIP_MANIFEST = '1'; cargo test --workspace --no-fail-fast`.
