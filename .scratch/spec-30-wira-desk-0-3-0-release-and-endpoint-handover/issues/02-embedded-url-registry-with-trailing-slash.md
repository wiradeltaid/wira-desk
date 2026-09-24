---
id: SPEC-30-02
component: settings
satisfies: [UC-8]
blocked_by: []
status: ready-for-agent
touches:
  - crates/settings/src/urls.rs
  - crates/settings/src/main.rs
  - crates/settings/src/update.rs
tests:
  - settings::urls::tests::all_wiradelta_urls_end_with_slash
  - settings::update::tests::browser_allowlist_rejects_url_without_trailing_slash
  - settings::update::tests::browser_allowlist_handles_approved_vectors
---

# 02: Feature — Embedded URL registry with trailing slash

**What to build:** Create a single embedded URL registry module for Settings where all `wiradelta.id` URLs terminate with a trailing slash (`/`), and restrict the external browser opening allowlist strictly to the registry forms plus the GitHub repository root and issues endpoints.

**Blocked by:** None (can start immediately, in parallel with SPEC-30-01 and SPEC-30-07)

**Status:** ready-for-agent

## Implementation Details

1. **URL Registry Module:** Create `crates/settings/src/urls.rs` containing public constants for `https://wiradelta.id/`, `https://wiradelta.id/wira-desk/`, and `https://wiradelta.id/wira-desk/privacy/`. Document why GitHub URLs (`https://github.com/wiradeltaid/wira-desk/`, `.../issues`) are exempt from trailing slashes.
2. **Settings Call Sites:** Replace all inline literal URLs across `crates/settings/src/main.rs` and other Settings modules with references to the registry.
3. **Browser Opening Allowlist:** Tighten `url_is_acceptable_to_open_in_browser` in `crates/settings/src/update.rs` to validate strictly against the exact registry URLs (rejecting non-trailing-slash variants like `https://wiradelta.id/wira-desk`) and explicitly approved GitHub prefixes (`https://github.com/wiradeltaid/wira-desk/` and `https://github.com/wiradeltaid/wira-desk/issues`).

## Acceptance Criteria

- [ ] Every guard and test is observed red on today's codebase before implementation, then green.
- [ ] All embedded `wiradelta.id` URLs in the registry end with `/`.
- [ ] Scanning non-test files in `crates/*/src` reveals zero literal `https://wiradelta.id` strings outside the registry and descriptor constants.
- [ ] Browser allowlist rejects `https://wiradelta.id/wira-desk` (without trailing slash) and accepts `https://wiradelta.id/wira-desk/`.
- [ ] Unit test tests approved vectors: accepts registry constants, `https://github.com/wiradeltaid/wira-desk/`, and `https://github.com/wiradeltaid/wira-desk/issues`; rejects non-allowlisted domains and non-approved subpaths.
- [ ] Workspace verification succeeds: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, and `$env:WIRADESK_SKIP_MANIFEST = '1'; cargo test --workspace --no-fail-fast`.
