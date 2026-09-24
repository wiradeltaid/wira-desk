# Smoke Test — SPEC-31 (FR-24, FR-25)

## Scope
- `FR-24`: About pane copy polish, natural word wrapping, non-breaking space for Windows 11, three-line license block, and duplicate GitHub link removal.
- `FR-25`: Differentiated update check HTTP status error message (`wiradelta.id answered with status {code}. You can try again.`) from installer binary download errors (`The download server answered with status {code}.`).

## Automated Smoke Verification
1. **About Pane UI Layout & Typography (A1, A2, A3, A4)**:
   - `settings::app::tests::about_pane_contains_exact_approved_copy`: PASS
   - `settings::app::tests::about_pane_has_no_duplicate_github_button`: PASS
   - `settings::app::tests::about_pane_has_non_breaking_space_for_windows_11`: PASS
   - `settings::app::tests::about_pane_license_block_has_three_distinct_lines`: PASS
   - `settings::app::tests::about_pane_retains_source_code_link_and_action_buttons`: PASS
   - `settings::app::tests::about_pane_card_hierarchy_and_divider_structure`: PASS
   - `settings::shortcut_row_slint_snapshot::tests::about_pane_merged_troubleshooting_card_renders_cleanly`: PASS
2. **Update Check Error Message Differentiation (A5)**:
   - `settings::update::tests::describe_http_status_uses_wiradelta_endpoint_for_checks`: PASS
   - `settings::update::tests::describe_install_preserves_download_server_wording`: PASS

## Result
- `FR-24`: PASS (automated test suite and layout verification green)
- `FR-25`: PASS (automated unit test and error differentiation green)
