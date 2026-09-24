---
id: SPEC-31-01
component: settings
satisfies: [UC-8, FR-24]
blocked_by: []
status: ready-for-dev
touches:
  - crates/settings/ui/panes/about_pane.slint
  - crates/settings/src/app.rs
  - crates/settings/src/shortcut_row_slint_snapshot.rs
tests:
  - settings::app::tests::about_pane_contains_exact_approved_copy
  - settings::app::tests::about_pane_has_no_duplicate_github_button
  - settings::app::tests::about_pane_has_non_breaking_space_for_windows_11
  - settings::app::tests::about_pane_license_block_has_three_distinct_lines
  - settings::app::tests::about_pane_retains_source_code_link_and_action_buttons
  - settings::app::tests::about_pane_card_hierarchy_and_divider_structure
---

# 01: Fix — About pane layout, word-wrapping, non-breaking space, and duplicate GitHub button removal

**What to build:** Refine the layout of `AboutPane` in `crates/settings/ui/panes/about_pane.slint` according to `ops/research/wdi-ecosystem-strategy/wira-desk/spec-about-tab-layout.md` items A1 through A4:

1. **A1 (Natural Word-Wrap for Update Check Disclosure):** Remove artificial mid-sentence newlines in `SettingToggleRow`'s description string (specifically between "newer" and "version", and between "processor" and "architecture"). Ensure the text flows as a single paragraph wrapped naturally by the UI (`wrap: word-wrap`), preserving only the single newline preceding `"No account, no analytics, no crash reporting."`.
2. **A2 (Non-breaking space for Windows 11):** Join "Windows" and "11" with a non-breaking space (U+00A0: `\u{00A0}`) in the description and wherever it appears in the About tab, preventing awkward line splits.
3. **A3 (Three-line License Block):** Move `"Full terms: LICENSE.txt in the install folder."` to its own distinct line directly below `"Free software under the GNU General Public License v3.0 only."`, creating a three-line sequence: GPL statement, Full terms line, and Third-party notices line.
4. **A4 (Remove Duplicate GitHub Button):** Remove the `[GitHub]` button from the action row in Card 3, leaving only `[Send a tip]` and `[Issue Tracker]`. The `"Source: github.com/wiradeltaid/wira-desk"` text link with its `OpenLinkIcon` and touch target is strictly retained in Card 1 to fulfill GPL-3.0 source access obligations.

**Test Updates (App & Snapshot Tests):**
- Update existing contradictory tests in `crates/settings/src/app.rs`:
  - `about_pane_contains_exact_approved_copy`: update expected copy to assert three-line license text, non-breaking space in `Windows 11`, single-paragraph update description without mid-sentence newlines, and absence of `[GitHub]` button.
  - `about_pane_card_hierarchy_and_divider_structure`: remove assertion for "GitHub repository" button, assert action row contains exactly "Send a tip" and "Issue Tracker", and assert source code text link is retained.
- Add regression test `about_pane_retains_source_code_link_and_action_buttons` proving the source code text link, `OpenLinkIcon`, and touch callback are preserved.
- Update `crates/settings/src/shortcut_row_slint_snapshot.rs` snapshots.

**Blocked by:** none

**Status:** ready-for-dev
