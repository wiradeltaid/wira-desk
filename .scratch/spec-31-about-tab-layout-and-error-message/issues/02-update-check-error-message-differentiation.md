---
id: SPEC-31-02
component: settings
satisfies: [UC-8, FR-25]
blocked_by: [SPEC-31-01]
status: ready-for-dev
touches:
  - crates/settings/src/update.rs
  - crates/settings/src/app.rs
tests:
  - settings::update::tests::describe_http_status_uses_wiradelta_endpoint_for_checks
  - settings::update::tests::describe_install_preserves_download_server_wording
---

# 02: Fix — Differentiate update check error message from installer download errors

**What to build:** In `crates/settings/src/update.rs`, explicitly split the error formatting paths between descriptor check queries (contacting `https://wiradelta.id/api/v1/update/wira-desk/`) and installer binary downloads (contacting GitHub Releases).

Per `ops/research/wdi-ecosystem-strategy/wira-desk/spec-about-tab-layout.md` item A5:
1. **Explicit Formatting Split:** Rather than modifying the shared `describe_http(&HttpError)` in a way that regresses installer download error descriptions, separate the formatting paths:
   - For update checks (`spawn_check` / check error path): when `HttpError::Status(code)` is encountered, format the message as:
     `"wiradelta.id answered with status {code}. You can try again."`
   - For installer downloads (`describe_install` / install error path): format `HttpError::Status(code)` as:
     `"The download server answered with status {code}."`
2. **Tests:**
   - Add unit test `describe_http_status_uses_wiradelta_endpoint_for_checks` asserting that status errors during checks cite `wiradelta.id` and include `"You can try again."`.
   - Add unit test `describe_install_preserves_download_server_wording` asserting that status errors during installation downloads cite `"The download server answered with status {code}."`.
   - Update UI tests in `crates/settings/src/app.rs` where mock status strings are asserted.

**Blocked by:** SPEC-31-01

**Status:** ready-for-dev
