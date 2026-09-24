---
id: SPEC-30-06
component: settings
satisfies: [FR-24, FR-25]
blocked_by: [SPEC-30-07]
status: closed
touches:
  - README.md
  - README.id.md
  - README.zh-CN.md
  - README.ja.md
  - README.de.md
  - README.es.md
  - README.fr.md
  - README.ko.md
  - README.pt-BR.md
  - README.ru.md
  - packaging/scoop-bucket/bucket/wiradesk.json
  - scripts/generate-winget-manifest.ps1
tests:
  - scripts::tests::verify_public_facts_readme_and_scoop_license
  - scripts::tests::verify_all_readme_locales_cleaned
---

# 06: Feature — README, Scoop template, and WinGet generator

**What to build:** Synchronize `README.md` and all nine localized translations with reality (accurate auto-start onboarding behavior, single portable zip, removal of competitor RAM numbers across all locales, updated privacy text), update the Scoop bucket template to `GPL-3.0-only`, and update the WinGet manifest generator.

**Blocked by:** SPEC-30-07 (and requires approved ops `about.md` Section A)

**Status:** closed

## Implementation Details

1. **README & Localized Versions:**
   - Update auto-start description to reflect opt-out in onboarding (checkbox defaults to checked in setup).
   - Update portable download instructions from loose binaries to single `WiraDesk-<version>-x64-portable.zip`.
   - Remove unverified competitor RAM figures ("150-500 MB of RAM" / "150–500 MB") across English and all 9 localized translations.
   - Replace "zero telemetry" claims with "no analytics, no account, no crash reporting" and mention update checks to `wiradelta.id`.
   - Ensure all `wiradelta.id` links have trailing slashes.
   - Omit WinGet instructions until the 0.3.0 package is accepted upstream.
   - Apply identical updates across English and all 9 localized README files (`README.md`, `docs/README.id.md`, `docs/README.zh-CN.md`, `docs/README.ja.md`, `docs/README.de.md`, `docs/README.es.md`, `docs/README.fr.md`, `docs/README.ko.md`, `docs/README.pt-BR.md`, `docs/README.ru.md`) in one PR.
2. **Scoop Template:** Update `packaging/scoop-bucket/bucket/wiradesk.json` with `"license": "GPL-3.0-only"`, description matching `about.md` §A verbatim (`Same-app window cycling, one-key snapping, and mouse button mapping for Windows 11.`), and `https://wiradelta.id/wira-desk/` homepage.
3. **WinGet Generator:** In `scripts/generate-winget-manifest.ps1`, update description regarding onboarding auto-start choice, set `ShortDescription` to match `about.md` §A, set `License: GPL-3.0-only`, and verify generated manifest output.

## Acceptance Criteria

- [x] Every guard and documentation check is observed red on today's codebase before implementation, then green.
- [x] `verify-public-facts.ps1` confirms `autostart_onboarding_default` is satisfied and stale "Auto-start is opt-in" phrasing is eliminated.
- [x] Automated scan over `README.md` and all 9 localized `docs/README.*.md` files proves complete absence of competitor RAM figures ("150-500", "150–500"), loose exe instructions, and unapproved telemetry claims.
- [x] Scoop bucket template specifies `"license": "GPL-3.0-only"` and description matches `about.md` §A verbatim.
- [x] Executing `scripts/generate-winget-manifest.ps1 -Version 0.3.0` generates a valid manifest containing `License: GPL-3.0-only` and `ShortDescription` matching `about.md` §A.
- [x] `verify-public-export.ps1` passes cleanly with no unapproved product claims.
- [x] Workspace verification succeeds: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, and `$env:WIRADESK_SKIP_MANIFEST = '1'; cargo test --workspace --no-fail-fast`.
