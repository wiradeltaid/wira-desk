---
id: SPEC-27-02
component: settings
satisfies: [UC-4, UC-14]
blocked_by: []
status: closed
tests:
  - app::tests::general_differs_from_default_ignores_auto_start_state
  - app::tests::restore_general_defaults_preserves_auto_start_preference
  - app::tests::factory_reset_defaults_still_resets_auto_start_to_false
  - shortcut_row_slint_snapshot::tests::about_pane_card_3_geometry_encloses_reset_button_without_overflow
  - shortcut_row_slint_snapshot::tests::about_pane_omits_troubleshooting_header
  - shortcut_row_slint_snapshot::tests::modal_reset_dialog_matches_onboarding_styling_without_blue_outline
  - shortcut_row_slint_snapshot::tests::settings_normal_window_height_is_560px
---

# 02: General auto-start defaults isolation, About card 3 polish, and modal confirmation redesign

**What to build:**
1. **General Pane `auto_start` Isolation (`app.rs`, `general_pane.slint`)**:
   - `SettingsModel::general_differs_from_default`: compare only `switcher.visual_enabled` and `switcher.visual_hold_delay_ms` against `Config::default()`. Remove `draft.general.auto_start` comparison.
   - `SettingsModel::restore_general_defaults`: restore `switcher.visual_enabled` and `switcher.visual_hold_delay_ms` to defaults while leaving `draft.general.auto_start` unchanged.
   - Retain full reset behavior in `factory_reset_defaults`: full factory reset sets `Config::default()` (resetting `auto_start` to `false`).
   - Replace/update existing tests asserting the previous behavior (`general_pane_defaults_button_visibility_and_action`, `pane_defaults_differ_from_default_detection`, `restore_general_defaults_resets_only_general_preferences`, `general_defaults_preserves_check_updates_preference`).
2. **About Pane Card 3 Polish (`about_pane.slint`)**:
   - Delete the `Text` element displaying `"Troubleshooting & Recovery"`.
   - Retain `"Restore all preferences to defaults"` as the card section title.
   - Give Card 3 a stable test identifier (`accessible-label: "Support and recovery card"`).
   - Ensure the card layout properly bounds all child elements so `[Reset all settings…]` is fully enclosed within the card rectangle without overflowing the bottom border.
   - Add a geometry test at `760 × 560` scrolling to Card 3 and asserting that all four edges of the Reset button are strictly within the Card 3 boundary.
   - Update existing About tests (`about_pane_card_hierarchy_and_divider_structure`, `about_pane_attribution_card_at_bottom`) to reflect the removed heading.
3. **Window Height and Modal Confirmation Redesign (`main_window.slint`)**:
   - Change `normal_height` from `610px` to `560px` (keeping `normal_width: 760px`).
   - Redesign the Factory Reset confirmation dialog container:
     - 12px border radius, `Palette.bg_card`, `Palette.stroke_card` border, refined drop shadow.
     - Clean padding and typography matching `OnboardingModal`.
     - Remove the 2px blue focus outline (`border-color: modal_focus.focus_button == 0 ? Palette.accent_primary : ...`) on Cancel and Confirm buttons.
     - Replace with an observable non-outline focus state (`Palette.bg_card_hover` background tint on the focused button) for both Cancel and Confirm while retaining keyboard traversal (Tab/Backtab/arrows) and activation (Return/Space).

**Blocked by:** None.

**Status:** closed

## Acceptance Criteria

- [x] Toggling `auto_start` on/off does not cause the General pane `↺ Defaults` button to appear when switcher preferences are at defaults.
- [x] Clicking `↺ Defaults` in General pane restores `visual_enabled` and `visual_hold_delay_ms` while preserving `auto_start`.
- [x] Full factory reset confirmation still resets all configuration fields including `auto_start` to `false`.
- [x] In About pane Card 3, the heading `"Troubleshooting & Recovery"` is removed.
- [x] Card 3 geometry test at 760x560 proves that all four bounds of `[Reset all settings…]` lie strictly within the card boundary without clipping or overflowing, and the button remains clickable.
- [x] Settings window normal height is 560px.
- [x] The Factory Reset modal dialog features 12px rounded corners, consistent Onboarding-like card styling, and buttons without 2px blue focus outlines, displaying observable non-outline focus on both Cancel and Confirm buttons.
- [x] Existing tests asserting previous behavior are updated, and all tests named in frontmatter pass cleanly.
