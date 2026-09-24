# SPEC-30: Release 0.3.0 Endpoint and Distribution Handover

## Problem Statement

Wira Desk is preparing for the 0.3.0 milestone and website go-live. Currently, the application and repository contain several outdated behaviors and assertions:

1. **Update Check & Network Boundary:** The application currently checks for updates directly against GitHub Releases (`latest.json`), sending a bare User-Agent string without platform or architecture dimensions. Under the WDI ecosystem strategy, update checks must move to the canonical self-hosted endpoint `https://wiradelta.id/api/v1/update/wira-desk/` with a strictly formatted 4-part User-Agent (`WiraDesk/<version> (Windows <major>.<minor>.<build>; <arch>)`) and disallow HTTP redirects for descriptor fetches, while installer downloads remain securely pinned to GitHub Releases.
2. **Embedded URLs & Copy Inconsistencies:** Embedded URLs across the Settings application lack trailing slashes and are scattered across call sites without a single source of truth. The About and General panes contain outdated disclosures (stating update checks contact GitHub Releases and send nothing), unverified marketing slop, and a "Support development" button instead of "Send a tip".
3. **Legal & Threat Model Synchronization:** In-repository legal documents (`PRIVACY.md`, `SECURITY.md`) lack the Indonesian canonical versions (`.id.md`), official English translation copy stamps, and accurate update endpoint disclosures. `docs/threat-model.md` still assumes a single GitHub network destination, omits window title inspection in the visual switcher, and fails to document existing log rotation or residual staging risks.
4. **Packaging & Release Artifacts:** Release workflows upload loose binaries instead of a single portable archive (`WiraDesk-<version>-x64-portable.zip`), while documentation claims auto-start is opt-in (when onboarding defaults to opt-out with a pre-checked box) and cites unmeasured competitor RAM numbers across localized READMEs.
5. **Corpus & Version Authority:** The core architecture documents (`BR-8`, C4 L1, PRD §3.12/§7) do not reflect the new `wiradelta.id` endpoint, and the crate version remains at unreleased `0.2.4` requiring owner-authorized bump to `0.3.0` and changelog consolidation.
6. **Release Cutover & Same-Day Go-Live (C1):** Release 0.3.0 and the website go-live occur on the same day. There must be no user-facing period where installations receive HTTP 401 responses; the self-hosted endpoint and website must be live when tag `v0.3.0` is published.

## Solution

Deliver the complete set of ten handover units (WDK-H-01 through WDK-H-10) across ten tracer-bullet tickets:

1. **Update Check Subsystem (H-01):** Migrate descriptor URL to `https://wiradelta.id/api/v1/update/wira-desk/`. Implement pure function for User-Agent formatting strictly matching `WiraDesk/<version> (Windows <major>.<minor>.<build>; <arch>)` backed by `RtlGetVersion` and `IsWow64Process2` (ensuring x64 under WOW64 on ARM64 reports `arm64`). Enforce `WINHTTP_OPTION_REDIRECT_POLICY_NEVER` on descriptor requests while allowing redirects for installer payloads on GitHub. Replace old space-disallowing User-Agent test. Ensure descriptor and installer requests share the exact same User-Agent. Add ignored integration test verifying 302 failure on debug override.
2. **Embedded URL Registry (H-02):** Centralize all embedded `wiradelta.id` URLs in a dedicated module where every URL has a trailing slash. Restrict the browser opening allowlist strictly to the registry forms plus the GitHub repository root, issues, and releases paths. Runs in parallel with H-01 and H-07.
3. **Application Copy Polish (H-03):** Align About pane text word-for-word with the approved ops `about.md` Section C fixture. Rename "Support development" to "Send a tip" pointing to `https://wiradelta.id/wira-desk/`. Clean General pane slop and unapproved dashes. Lock the six legal labels and three tab names ("About", "Mouse", "General").
4. **Legal Copies with Stamp (H-04):** Synchronize `PRIVACY.id.md`, `PRIVACY.md`, `SECURITY.id.md`, and `SECURITY.md` from upstream ops legal sources with the standard English copy stamp, sequenced after H-09 and conditioned on filled ops placeholders, effective date matching 0.3.0 release date, and enabled GitHub private vulnerability reporting.
5. **Threat Model 0.3.0 (H-05):** Update `docs/threat-model.md` sequenced after H-04 to document the dual network boundaries, switcher window title enumeration, log rotation at 1 MB, 30-day server-side aggregate log retention per server contract, and residual temp-directory execution risks matching the newly synchronized `SECURITY.md` §9 verbatim.
6. **Documentation, Scoop, and WinGet (H-06):** Update `README.md` and all 9 localized translations to declare auto-start onboarding default accurately, reference the portable zip, remove competitor RAM numbers across all locales, and align privacy claims. Update the Scoop bucket template to `GPL-3.0-only`, ShortDescription from `about.md` §A, and trailing-slash homepage. Update WinGet generator descriptions and verify output.
7. **Portable Zip & Release Artifact Guard (H-07):** Update `.github/workflows/release.yml` and `ci.yml` to package `WiraDesk-<version>-x64-portable.zip` containing `wiradesk.exe`, `wiradesk-settings.exe`, `LICENSE.txt`, and `NOTICE.txt`. Stop uploading loose executables. Remove stale WinGet PR #426321 reference in `release.yml`. Add `scripts/verify-release-artifacts.ps1` to CI. Runs in parallel with H-01 and H-02.
8. **Public Facts (H-08):** Update `docs/public-facts.yaml` for v0.3.0 tag consumption by the website: hold delay parameters (300 ms default, 100-500 ms range), snapping ratios (67% lateral/bottom, 33% top), descriptor URL, auto-start onboarding default, and license identifiers. Note memory figures are remeasured at candidate tag commit on 60-second idle via `scripts/measure-memory.ps1`.
9. **Corpus Alignment (H-09):** Bring repository architecture and promise documents (`BR-8`, C4 L1, PRD, SRS) into agreement with the implemented code and finalized public facts.
10. **Release 0.3.0 (H-10):** Execute version bump to `0.3.0` via `bump-version.ps1 -Set 0.3.0 -Owner` with owner authorization in session, consolidate changelog sections (merging unreleased changes, resolving release date to the site go-live date, and recording SPEC-29 top edge 33% defaults cleanly), verify same-day go-live cutover (no-401 live endpoint verification), and finalize release metadata.

## User Stories

1. As a Wira Desk user, I want the updater to check `https://wiradelta.id/api/v1/update/wira-desk/` using the exact `WiraDesk` User-Agent format, so that release telemetry remains privacy-respecting and aggregate without tracking me.
2. As a Wira Desk user on ARM64 Windows running x64 emulation, I want the User-Agent to report `arm64`, so that native ARM64 demand is accurately captured by server aggregates.
3. As a Wira Desk user, I want descriptor requests to never follow redirects, so that update manifests cannot be hijacked or redirected away from the canonical endpoint.
4. As a Wira Desk user, I want installer downloads to continue downloading securely from GitHub Releases with SHA-256 validation and the same User-Agent, so that update binaries remain authentic and verified.
5. As a legacy 0.2.x user, I want GitHub Releases to continue hosting `latest.json`, so that my existing installation discovers the 0.3.0 upgrade.
6. As a Settings user, I want all links to `wiradelta.id` to have proper trailing slashes, so that requests avoid redundant redirect hops.
7. As a Settings user, I want the browser opening mechanism to reject non-allowlisted URLs, so that external links cannot navigate to arbitrary sites.
8. As a Wira Desk user, I want the About pane to honestly state what information update checks send, so that I have complete transparency regarding network activity.
9. As a user wishing to support the project, I want the About pane to provide a "Send a tip" button linking to the product webpage, so that I can easily contribute to development.
10. As an assistive tech user, I want all Settings pane controls and tabs to maintain stable, recognizable accessibility labels, so that screen readers continue functioning seamlessly across text updates.
11. As an Indonesian-speaking user, I want access to `PRIVACY.id.md` and `SECURITY.id.md` directly in the repository, so that I can read the binding legal and security terms in my native language.
12. As a security researcher, I want `SECURITY.md` and `docs/threat-model.md` to accurately disclose all network boundaries, logging behaviors, and temp folder residual risks, so that I understand the security guarantees.
13. As a portable software user, I want a single `WiraDesk-<version>-x64-portable.zip` containing both executables, `LICENSE.txt`, and `NOTICE.txt`, so that I can run the application portably with all legal notices present.
14. As a package maintainer, I want Scoop bucket manifests and WinGet generator scripts to reflect `GPL-3.0-only` licensing and onboarding auto-start realities, so that package repositories remain in sync with upstream.
15. As a website visitor, I want `docs/public-facts.yaml` to publish verified 0.3.0 constants, so that marketing and download pages display accurate system requirements and defaults.
16. As a repository maintainer, I want `CHANGELOG.md` to clearly detail all 0.3.0 improvements without internal tracking IDs or slop, so that release notes are clean and informative.
17. As an installer user on launch day, I want the update endpoint to be live without 401 errors on the same day the website goes live, so that my first update check succeeds seamlessly.

## Implementation Decisions

- **Update Check Protocol:** The descriptor fetch URL is hardcoded as `https://wiradelta.id/api/v1/update/wira-desk/`. WinHTTP request options explicitly set `WINHTTP_OPTION_REDIRECT_POLICY_NEVER` for descriptor requests, while installer binary downloads use `WINHTTP_OPTION_REDIRECT_POLICY_DISCLOSURE` / follow.
- **User-Agent Pure Function:** User-Agent string construction is implemented as a pure function accepting product name, version, OS version tuple `(major, minor, build)`, and architecture string. It formats strictly as `WiraDesk/<version> (Windows <major>.<minor>.<build>; <arch>)` validated against server regex. OS version is read from `RtlGetVersion` (without UBR) and architecture from `IsWow64Process2` (`x64`, `arm64`, `x86`, lowercase; x64 process on ARM64 reports `arm64`). If either API fails, fallback uses target build architecture and available version, never emitting extra components. Required feature flags in `crates/shared/Cargo.toml` (`Win32_System_SystemInformation`, `Win32_System_Threading`) are declared.
- **URL Registry:** All in-app `wiradelta.id` URLs are extracted into a single registry module in `crates/settings/src/urls.rs`. All internal URLs end with `/`. The browser allowlist validates that any opened URL matches one of the registry constants or the approved GitHub repository paths (`https://github.com/wiradeltaid/wira-desk/` and `https://github.com/wiradeltaid/wira-desk/issues`).
- **About Copy Synchronization:** About pane copy is imported directly from the ops legal specification (`about.md` §C fixture), eliminating all promotional slop. The six legal labels ("Check for updates automatically", "Check for updates", "Download and install", "Enable Mouse Navigation", "Enable Visual Switcher Overlay", "Reset all settings…") and three tab names ("About", "Mouse", "General") are locked by unit tests.
- **Single Portable Archive:** Release staging creates `WiraDesk-<version>-x64-portable.zip` containing `wiradesk.exe`, `wiradesk-settings.exe`, `LICENSE.txt`, and `NOTICE.txt`. Loose executable uploads are eliminated from `release.yml`. `SHA256SUMS` contains only the setup installer and the portable zip.
- **Dependency Flow & Sequencing:**
  - `H-07` (portable zip), `H-02` (URL registry), and `H-01` (update check) run in parallel as the root tier.
  - `H-03` (pane copy) depends on `H-02`.
  - `H-06` (README/packaging) depends on `H-07`.
  - `H-08` (public facts) depends on `H-01`, `H-02`, and `H-06`.
  - `H-09` (corpus) depends on `H-03` and `H-08`.
  - `H-04` (legal copies) depends on `H-09` (and requires ops placeholders filled, effective date set, and private vulnerability reporting enabled).
  - `H-05` (threat model) depends on `H-04` (guaranteeing that threat model residual risks are validated against the newly synchronized `SECURITY.md` §9).
  - `H-10` (release) gates on `H-05` (transitively gating on all 9 tickets) and explicit owner session authorization, including C1 go-live cutover validation.

## Testing Decisions

- **Universal Red-to-Green Rule:** Every guard and test in each ticket must be demonstrably red on today's code before implementation, then green. Final validation across every ticket requires `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, and `$env:WIRADESK_SKIP_MANIFEST = '1'; cargo test --workspace --no-fail-fast`.
- **Network Boundary Tests:** Test descriptor fetch against unit abstraction to prove `Never` redirect policy is set and non-200 responses (e.g. 301, 302, 404) trigger `HttpError::Status`. Prove installer streaming download follows redirects and uses the exact same User-Agent. Include an `#[ignore]` integration test exercising debug override against GitHub Releases proving 302 fails as status error.
- **User-Agent Tests:** Unit tests verify exact `WiraDesk` User-Agent string formatting against vector test cases (`WiraDesk/0.3.0 (Windows 10.0.22631; arm64)` and `WiraDesk/0.3.0 (Windows 10.0.26100; x64)`) matching server regex without extra fields or spaces, including ARM64 WOW64 emulation handling.
- **URL Registry Tests:** Static inspection and unit tests ensure every registered `wiradelta.id` URL terminates in `/`. Allowlist tests confirm refusal of non-trailing-slash variants and non-approved external subpaths.
- **Copy & Label Tests:** Slint and Rust tests assert full text equality with the approved Section C fixture, verify absence of forbidden slop phrases ("GitHub Releases", "Support development", "UX Honesty", etc.), and assert all six legal labels and three tab names are present verbatim.
- **Artifact Verification Tests:** `scripts/verify-release-artifacts.ps1` tests assert that staging directories containing loose executables fail, while directories containing setup installer, portable zip, `SHA256SUMS`, and `latest.json` pass.
- **Public Facts Verification:** `scripts/verify-public-facts.ps1` runs against codebase constants to prove 100% agreement on all derived facts at candidate tag commit.

## Out of Scope

- Implementing the server endpoint (handled in `wiradeltaid-admin` / `devops` under API-01 and OPS-01).
- Cloudflare configuration rules (handled by owner under B9).
- Third-party package submissions (Scoop bucket sync and WinGet submission are manual owner actions post-tag).
- Direct code implementation, commits, or pushes in this planning turn.

## Further Notes

- All changes adhere to WDI publication hygiene: no internal requirement identifiers (`SPEC-`, `FR-`, `WDK-`) in product code or commit messages.
- Minor version bump from 0.2.4 to 0.3.0 requires explicit owner session approval per `AGENTS.md` versioning authority.
