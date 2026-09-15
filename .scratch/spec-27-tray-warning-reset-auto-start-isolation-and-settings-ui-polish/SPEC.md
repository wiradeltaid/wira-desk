# SPEC-27: Tray Warning Real-time Reset, General Auto-Start Isolation, About Card Polish, and Modal Redesign

## Problem Statement

Following manual testing of SPEC-26 delivery, five user-facing defects and polish issues were identified:
1. The daemon's system tray icon displays a red warning dot when a Tier-2 warning occurs (such as a rejected configuration reload), but never resets back to normal in real time once the user resolves the collision in Settings and saves a valid configuration.
2. In About pane Card 3, the `[Reset all settings…]` action button overflows beyond the bottom edge of the card due to Slint's preferred-height calculation under-measuring card height when word-wrapped description text and multiple stacked headings are present.
3. In General pane, `↺ Defaults` button visibility checks `draft.general.auto_start` against `false`, causing the button to appear whenever auto-start is enabled. Clicking `↺ Defaults` turns off auto-start, unexpectedly disabling the user's OS-level logon integration when they only intended to reset switcher preferences.
4. The main Settings window height (610px) is unnecessarily tall. The Factory Reset modal confirmation dialog has an unpolished layout with an awkward 2px blue focus outline around the Cancel button that does not match any other button in the application, whereas the Onboarding tutorial has a well-received, cohesive design system.
5. In About pane Card 3, the heading `Troubleshooting & Recovery` is redundant with the `Restore all preferences to defaults` title directly below it and should be removed.

## Solution

1. **Selective Real-time Tray Warning Reset (`crates/daemon/src/tray.rs`)**:
   - Model warning causes explicitly in `TrayData`: track distinct warning sources (e.g., `config_rejected: bool` vs. `acl_insecure: bool` vs. `simulated: bool`).
   - On `WM_APP_RELOAD_CONFIG`: when `crate::config::handle_reload_message` returns `ReloadOutcome::Applied { .. }`, clear `config_rejected = false`.
   - Update `data.warning_latched`: latched warning remains true if and only if at least one warning cause remains active (e.g. an insecure non-admin executable ACL warning persists even if a new configuration is applied).
   - If no warning cause remains active and `data.state == TrayState::Warning`, immediately transition `data.state` to `TrayState::Normal` via `set_state(data, TrayState::Normal)`.
   - Preserve Critical state precedence: if the hook is dead (`TrayState::Critical`), a successful config reload must never downgrade Critical to Normal.

2. **General Pane `auto_start` Isolation (`crates/settings/src/app.rs`, `general_pane.slint`)**:
   - Update `general_differs_from_default`: compare only `switcher.visual_enabled` and `switcher.visual_hold_delay_ms` against `Config::default()`. `auto_start` is excluded from difference detection.
   - Update `restore_general_defaults`: restore `switcher.visual_enabled` and `switcher.visual_hold_delay_ms` while preserving `draft.general.auto_start`.
   - Retain full reset in `factory_reset_defaults`: full factory reset sets `Config::default()` (resetting `auto_start` to `false`), maintaining the intentional distinction between pane-scoped defaults and full factory reset.
   - Update existing regression tests (`general_pane_defaults_button_visibility_and_action`, `pane_defaults_differ_from_default_detection`, `restore_general_defaults_resets_only_general_preferences`, `general_defaults_preserves_check_updates_preference`) to assert this isolation contract.

3. **About Pane Card 3 Polish (`crates/settings/ui/panes/about_pane.slint`)**:
   - Remove the `Troubleshooting & Recovery` text element.
   - Retain `Restore all preferences to defaults` as the section title.
   - Update Card 3 with a stable test identifier (`accessible-label: "Support and recovery card"`) and layout padding so the `[Reset all settings…]` button is completely enclosed within the card boundaries with comfortable vertical breathing room, without any clipping or overflowing.
   - Update existing About tests (`about_pane_card_hierarchy_and_divider_structure`, `about_pane_attribution_card_at_bottom`) to reflect the removed heading.

4. **Window Height and Modal Confirmation Redesign (`crates/settings/ui/main_window.slint`)**:
   - Reduce Settings window `normal_height` from `610px` to `560px` (preserving `normal_width: 760px`), tightening vertical whitespace across all panes while maintaining the ScrollHint contract.
   - Redesign the Factory Reset confirmation dialog container and buttons to align with Onboarding design system aesthetics (`OnboardingModal`):
     - Container: 12px border radius, `Palette.bg_card`, `Palette.stroke_card` border, refined drop shadow.
     - Spacing and typography: clear title and body hierarchy.
     - Focus indicator: remove the 2px blue focus outline (`border-color: Palette.accent_primary`). Replace with a subtle, cohesive focus styling (e.g. `Palette.bg_card_hover` background tint on the selected button) indicating keyboard selection clearly without an artificial blue border.
     - Retain keyboard traversal (Tab / Backtab / arrows) and activation (Return / Space) with observable focus state on both Cancel and Confirm.

## User Stories

1. As a user who fixed a shortcut collision or invalid configuration in Settings, I want the system tray icon's warning dot to immediately disappear upon saving, so that I can see the application is healthy without restarting the daemon.
2. As a user with an insecure binary installation, I want the warning dot to persist even if I reload my shortcuts, so that an unresolved security hazard is not falsely cleared by an unrelated settings save.
3. As a user viewing the About pane, I want the `[Reset all settings…]` button to remain fully within its card boundary, so that the card layout looks clean and properly formatted.
4. As a user on the General settings pane, I want the `↺ Defaults` button to ignore whether "Launch when you log in to Windows" is enabled, so that enabling auto-start does not mark General settings as non-default.
5. As a user clicking `↺ Defaults` in the General settings pane, I want my auto-start choice to be preserved, so that restoring switcher defaults does not disable my Windows login integration.
6. As a user opening the Factory Reset confirmation dialog, I want the dialog window and button styling to match the clean aesthetic of the Onboarding tutorial, so that the application feels cohesive and polished.
7. As a keyboard user navigating the Factory Reset dialog, I want buttons to indicate focus cleanly without a jarring blue outline, so that the visual styling matches the rest of the application.
8. As a user reviewing the About pane, I do not want redundant `Troubleshooting & Recovery` and `Restore all preferences to defaults` headings stacked together, so that the recovery card is concise.
9. As a desktop user, I want the Settings window height to be more compact, so that it fits comfortably on my screen without unnecessary empty vertical space.

## Implementation Decisions

- In `crates/daemon/src/tray.rs`:
  - Track structured warning causes in `TrayData`:
    ```rust
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct WarningCauses {
        pub config_rejected: bool,
        pub acl_insecure: bool,
        pub simulated: bool,
    }
    ```
  - When `WM_APP_RELOAD_CONFIG` is handled:
    - If `matches!(outcome, ReloadOutcome::Applied { .. })`, set `data.warning_causes.config_rejected = false`.
    - If `matches!(outcome, ReloadOutcome::Rejected(_))`, set `data.warning_causes.config_rejected = true`.
    - `data.warning_latched = data.warning_causes.any_active()`.
    - If `!data.warning_latched && data.state == TrayState::Warning`, call `set_state(data, TrayState::Normal)`.
  - When `data.state == TrayState::Critical`, do not downgrade to Normal upon reload.
- In `crates/settings/src/app.rs`:
  - Modify `SettingsModel::general_differs_from_default(&self) -> bool` to exclude `auto_start`.
  - Modify `SettingsModel::restore_general_defaults(&mut self)` to preserve `self.draft.general.auto_start`.
  - Update previous tests that asserted `auto_start` in General defaults.
- In `crates/settings/ui/panes/about_pane.slint`:
  - Remove `Troubleshooting & Recovery` `Text` node.
  - Enclose the recovery section within a well-padded layout that guarantees the `Reset all settings…` button does not extend past the card border.
- In `crates/settings/ui/main_window.slint`:
  - Update `normal_height: 560px`.
  - Style the confirmation dialog `Rectangle` with `border-radius: 12px` and padding similar to `OnboardingModal`.
  - Replace button outline `border-color: modal_focus.focus_button == 0 ? Palette.accent_primary : Palette.stroke_card` with `Palette.stroke_card` and use `Palette.bg_card_hover` background when focused to cleanly indicate keyboard selection without blue border outlines.

## Testing Decisions

- Daemon unit / integration tests:
  - Verify that a rejected configuration reload sets `config_rejected` and transitions to `TrayState::Warning`.
  - Verify that subsequent valid reload clears `config_rejected` and restores `TrayState::Normal`.
  - Verify that if `acl_insecure` is active, a valid configuration reload does NOT clear `warning_latched` and leaves `TrayState::Warning` active.
  - Verify that when `data.state` is `TrayState::Critical`, a successful reload does not downgrade the icon to `TrayState::Normal`.
- Settings model tests:
  - Replace `pane_defaults_differ_from_default_detection` and `general_pane_defaults_button_visibility_and_action` assertions to verify that `auto_start: true` does not trigger `↺ Defaults`.
  - Update `restore_general_defaults_resets_only_general_preferences` to verify that `auto_start` remains `true` after restore.
  - Update `about_pane_card_hierarchy_and_divider_structure` to assert absence of `Troubleshooting & Recovery`.
  - Verify `factory_reset_defaults` still resets `auto_start` to `false`.
- Slint snapshot tests:
  - Geometry test at 760x560: scroll to Card 3 and assert that all four bounds of `[Reset all settings…]` lie within Card 3 content boundaries.
  - Verify `Troubleshooting & Recovery` text is absent from Card 3.
  - Verify modal confirmation dialog renders with 12px radius, compact height, and focus selection styling without 2px blue outline on both Cancel and Confirm.
  - Verify window normal height is 560px.

## Out of Scope

- Changing the configuration file schema or on-disk format.
- Modifying Onboarding step content or flows.
- Changing shortcut chord capture or daemon keyboard hook logic.

## Further Notes

All UI changes remain staged in the in-memory draft until the user explicitly clicks **Save Changes** in the footer.
