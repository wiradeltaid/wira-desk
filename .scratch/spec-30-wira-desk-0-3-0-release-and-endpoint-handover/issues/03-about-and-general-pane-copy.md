---
id: SPEC-30-03
component: settings
satisfies: [UC-8]
blocked_by: [SPEC-30-02]
status: closed
touches:
  - crates/settings/ui/panes/about_pane.slint
  - crates/settings/ui/panes/general_pane.slint
  - crates/settings/src/app.rs
  - crates/settings/src/shortcut_row_slint_snapshot.rs
  - crates/settings/src/main.rs
tests:
  - settings::app::tests::about_pane_contains_exact_approved_copy
  - settings::app::tests::general_pane_has_no_slop_or_unapproved_dashes
  - settings::app::tests::settings_tabs_and_legal_labels_locked
---

# 03: Feature — About and General pane copy

**What to build:** Synchronize About and General pane copy in the Settings UI with approved legal and brand specifications, replacing promotional slop and British spellings, renaming "Support development" to "Send a tip", and locking the six critical legal labels and three tab titles.

**Blocked by:** SPEC-30-02 (and requires approved upstream ops `about.md` Section C)

**Status:** closed

## Implementation Details

1. **About Pane Copy Fixture:** Update `crates/settings/ui/panes/about_pane.slint` word-for-word against the approved ops `about.md` Section C:
   ```
   Wira Desk <version>
   Copyright (c) 2026 Wira Delta Indonesia
   Same-app window cycling, one-key snapping, and mouse button mapping for Windows 11.

   Free software under the GNU General Public License v3.0 only. Full terms: LICENSE.txt in the install folder.
   Third-party components and their licenses: NOTICE.txt in the install folder.
   Source: github.com/wiradeltaid/wira-desk
   What it stores and sends: wiradelta.id/wira-desk/privacy/

   [x] Check for updates automatically
       Once a day, and when you press Check for updates, Wira Desk asks wiradelta.id whether a newer
       version exists. The request names Wira Desk, its version, your Windows version, and the processor
       architecture. Nothing else is attached.
   No account, no analytics, no crash reporting.

   Questions: support@wiradelta.id
   Security reports: GitHub Security Advisories on the repository
   [Send a tip]  [Issue Tracker]  [GitHub]
   ```
2. **Send a Tip Action:** Rename button and accessible-label from "Support development" to "Send a tip", bound to the URL registry constant `https://wiradelta.id/wira-desk/`.
3. **General Pane Cleanups:** In `crates/settings/ui/panes/general_pane.slint`, replace en-dash with "(100 to 500 ms)" and remove "(UX Honesty)".
4. **Legal Labels & Tabs Guard:** Preserve exact strings: "Check for updates automatically", "Check for updates", "Download and install", "Enable Mouse Navigation", "Enable Visual Switcher Overlay", "Reset all settings…", and tab titles "About", "Mouse", "General".
5. **Snapshot & Unit Tests:** Update snapshot and UI tests in `app.rs` and `shortcut_row_slint_snapshot.rs` to reflect the approved copy. Capture visual proof (screenshots attached to PR).

## Acceptance Criteria

- [x] Every guard and test is observed red on today's codebase before implementation, then green.
- [x] Source scan over `crates/settings/ui/**/*.slint` confirms absence of "GitHub Releases", "Support development", "UX Honesty", "accelerates", "invisible, fast", "Nothing about you is sent", "licence", and Unicode dash characters U+2013 / U+2014.
- [x] Unit tests assert full text equality of About pane contents against the approved Section C fixture.
- [x] Unit tests assert that all six legal labels and all three tab names ("About", "Mouse", "General") are present verbatim.
- [x] Existing snapshot tests are updated and pass cleanly.
- [x] Light and dark mode screenshots of About and General panes captured and included in PR description.
- [x] Workspace verification succeeds: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, and `$env:WIRADESK_SKIP_MANIFEST = '1'; cargo test --workspace --no-fail-fast`.
