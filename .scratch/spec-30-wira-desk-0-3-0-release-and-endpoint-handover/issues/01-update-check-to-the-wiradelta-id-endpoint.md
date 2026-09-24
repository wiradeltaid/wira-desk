---
id: SPEC-30-01
component: settings
satisfies: [UC-8, FR-24, FR-25]
blocked_by: []
status: ready-for-agent
touches:
  - crates/shared/src/update.rs
  - crates/shared/src/https.rs
  - crates/daemon/src/updatecheck.rs
  - crates/shared/Cargo.toml
tests:
  - shared::update::tests::descriptor_url_is_wiradelta_endpoint
  - shared::https::tests::user_agent_matches_platform_contract
  - shared::https::tests::descriptor_fetch_disallows_redirects
  - shared::https::tests::installer_and_descriptor_share_identical_user_agent
  - shared::update::tests::ignored_live_debug_override_fails_on_redirect
---

# 01: Feature — Update check to the wiradelta.id endpoint

**What to build:** Migrate update descriptor checking to the canonical self-hosted endpoint `https://wiradelta.id/api/v1/update/wira-desk/`, format User-Agent strictly as `WiraDesk/<version> (Windows <major>.<minor>.<build>; <arch>)`, enforce no-redirect policy for descriptor fetches while maintaining redirect following for installer downloads, ensure both descriptor and installer downloads share the identical User-Agent, and preserve legacy 0.2.x upgrade path via GitHub Releases.

**Blocked by:** None (can start immediately, in parallel with SPEC-30-02 and SPEC-30-07)

**Status:** ready-for-agent

## Implementation Details

1. **Descriptor URL:** Update `latest_json_url()` in `crates/shared/src/update.rs` to return `https://wiradelta.id/api/v1/update/wira-desk/` in release builds. Retain `WIRADESK_DEV_LATEST_JSON_URL` override under `#[cfg(debug_assertions)]`.
2. **User-Agent Pure Function & Platform Detection:**
   - Implement a pure function that takes product name, version, OS version `(major, minor, build)`, and architecture string, producing strictly `WiraDesk/<version> (Windows <major>.<minor>.<build>; <arch>)`.
   - Read OS version via `RtlGetVersion` (without UBR revision number).
   - Read native processor architecture via `IsWow64Process2` (`x64`, `arm64`, `x86`, lowercase; an x64 binary running under WOW64 emulation on ARM64 must report `arm64`).
   - If either API call fails, fallback uses the target compilation architecture and available version numbers, never appending extra fields or delimiters.
   - Add required `windows-sys` 0.52 feature flags to `crates/shared/Cargo.toml` (`Win32_System_SystemInformation`, `Win32_System_Threading`).
   - Ensure all `unsafe` blocks carry explicit `SAFETY:` comments. Use this exact User-Agent for both descriptor fetch and installer download in `crates/shared/src/https.rs`.
3. **Redirect Policy Enforcement:** Configure WinHTTP request option `WINHTTP_OPTION_REDIRECT_POLICY_NEVER` for descriptor requests so that HTTP 3xx responses yield `HttpError::Status(3xx)` rather than following redirects. Retain redirect following for installer binary streaming downloads.
4. **Legacy Compatibility:** Retain `latest.json` upload in `.github/workflows/release.yml` so 0.2.x clients continue discovering new releases.

## Acceptance Criteria

- [ ] Every guard and test is observed red on today's codebase before implementation, then green.
- [ ] Release builds return `https://wiradelta.id/api/v1/update/wira-desk/` without environment variables set.
- [ ] User-Agent matches the exact product format `^WiraDesk/(\d{1,4}\.\d{1,4}\.\d{1,6}) \(Windows (\d{1,3}\.\d{1,3}\.\d{1,6}); (x64|arm64|x86)\)$` and passes shared test vectors (including `WiraDesk/0.3.0 (Windows 10.0.22631; arm64)` and `WiraDesk/0.3.0 (Windows 10.0.26100; x64)`).
- [ ] Unit test explicitly validates that x64 process under WOW64 on ARM64 outputs `arm64`.
- [ ] Unit test validates graceful fallback to target architecture when platform version APIs fail, with zero additional parts.
- [ ] Existing `https.rs` test asserting absence of spaces in User-Agent is seen failing, then replaced with format specification tests.
- [ ] Unit test explicitly asserts descriptor requests and installer streaming downloads send the exact same computed User-Agent.
- [ ] Descriptor fetch requests `Redirects::Never` and non-200 / 3xx responses fail without following redirects; installer downloads request `Redirects::Follow`.
- [ ] An `#[ignore]` integration test exercising debug override to a 302 endpoint (e.g. GitHub Releases latest download) confirms failure with status 302 instead of successful redirect traversal.
- [ ] Runtime User-Agent check confirms no `COMPUTERNAME` or `USERNAME` leakage.
- [ ] Workspace verification succeeds: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, and `$env:WIRADESK_SKIP_MANIFEST = '1'; cargo test --workspace --no-fail-fast`.
