# SPEC-31: About Tab Layout Polish, Duplicate Link Removal, and Update Error Differentiation

## Problem Statement

Following the manual verification and owner review of the developer build for milestone 0.2.5/0.3.0 documented in `ops/research/wdi-ecosystem-strategy/wira-desk/spec-about-tab-layout.md` (and commit `8b394e0` in `ops`), the Settings About tab has the exact approved copy from `about.md` §C, and the "Send a tip" button is correctly installed. However, five layout and messaging defects require remediation:

1. **Mid-sentence Line Breaks (A1):** Artificial newlines copied directly from Markdown block formatting in ops (`whether a newer\nversion exists` and `the processor\narchitecture`) cause awkward wrapping in the update check toggle description. The text should be a single continuous paragraph that wraps automatically (`wrap: word-wrap`), with the only intentional newline occurring before `"No account, no analytics, no crash reporting."`.
2. **"Windows 11" Separation (A2):** "Windows" and "11" can wrap onto separate lines at the end of the description line. They must be joined with a non-breaking space (U+00A0: `\u{00A0}`) in the description and wherever else they appear in the About tab.
3. **License Terms Line Placement (A3):** `"Full terms: LICENSE.txt in the install folder."` is currently appended to the end of the GPL sentence and wraps unpredictably. It must be moved to its own distinct line directly below the GPL sentence, forming three clear lines: GPL statement, Full terms line, and Third-party notices line.
4. **Duplicate GitHub Link (A4):** GitHub appears twice in Card 3: once as a text link (`"Source: github.com/wiradeltaid/wira-desk"`) and once as a `[GitHub]` button in the action row. The `[GitHub]` button must be removed to avoid redundancy, while the `"Source"` text link is strictly retained to fulfill GPL-3.0 source code access obligations. The action row retains `[Send a tip]` and `[Issue Tracker]`.
5. **Update Check Error Message Misattribution (A5):** When an update check fails with an HTTP status code, `crates/settings/src/update.rs` currently formats the error as `"The download server answered with status {code}."`. This message misidentifies the failure, as the update check queries `wiradelta.id` for the descriptor, not the GitHub download server. The error for update check queries must be `"wiradelta.id answered with status {code}. You can try again."`, reserving "download server" strictly for failures during installer binary downloads via "Download and install".

## Proposed Solution

Deliver the five improvements across two sequential tickets in component `settings`:

1. **Ticket SPEC-31-01: About Pane Slint Layout & Duplicate Removal (A1, A2, A3, A4):**
   - Update `crates/settings/ui/panes/about_pane.slint`:
     - Join `"Windows\u{00A0}11"` with non-breaking space in description text.
     - Split license text into three distinct lines:
       ```
       Free software under the GNU General Public License v3.0 only.
       Full terms: LICENSE.txt in the install folder.
       Third-party components and their licenses: NOTICE.txt in the install folder.
       ```
     - Remove artificial mid-sentence newlines from the `SettingToggleRow` description, ensuring natural UI word wrapping.
     - Remove the `[GitHub]` button rectangle and associated `repo_touch` / `GitHubIcon` from the action row in Card 3, leaving `[Send a tip]` and `[Issue Tracker]`. Retain `OpenLinkIcon` and `source_touch` on the `"Source: github.com/wiradeltaid/wira-desk"` text link.
   - Update existing contradictory tests in `crates/settings/src/app.rs`:
     - `about_pane_contains_exact_approved_copy`: update expected strings for non-breaking space, three-line license text, single-paragraph update description, and absence of GitHub button.
     - `about_pane_card_hierarchy_and_divider_structure`: remove assertion for "GitHub repository" button, assert action row contains exactly "Send a tip" and "Issue Tracker", and assert source text link is retained.
   - Update snapshot tests in `crates/settings/src/shortcut_row_slint_snapshot.rs`.

2. **Ticket SPEC-31-02: Update Check Error Message Differentiation (A5):**
   - In `crates/settings/src/update.rs`:
     - Split error formatting into distinct paths:
       - Descriptor check (`spawn_check`): formats `HttpError::Status(code)` as `"wiradelta.id answered with status {code}. You can try again."`.
       - Installer download (`describe_install`): preserves `"The download server answered with status {code}."`.
   - Update unit tests in `crates/settings/src/update.rs` and UI tests in `crates/settings/src/app.rs` to assert the differentiated status string.

## External Release Preconditions & Non-Goals

1. **GitHub Private Vulnerability Reporting (Release Precondition):**
   - As recorded in `ops/research/wdi-ecosystem-strategy/plan/wira-desk.md`, the owner must enable private vulnerability reporting on the GitHub repository (`wiradeltaid/wira-desk`) before the 0.3.0 public release so that "Security reports: GitHub Security Advisories" operates properly for external reporters. This is an administrative repo setting, not an application code change.
2. **Website About Mockup Synchronization (Non-Goal):**
   - The website About pane mockup (`SettingsMockup.astro` in `wiradeltaid-web`) will be updated after release 0.3.0 is published (tracked in `SPEC-023` in that repository). It is out of scope for this repository.

## Upstream References

- `ops/research/wdi-ecosystem-strategy/wira-desk/spec-about-tab-layout.md` (Authoritative upstream spec, commit `8b394e0`)
- `ops/research/wdi-ecosystem-strategy/legal/wira-desk/about.md` §C (Authoritative copy fixture, commit `8b394e0`)
- `handover-0.3.0.md` WDK-H-03 ticket definition
