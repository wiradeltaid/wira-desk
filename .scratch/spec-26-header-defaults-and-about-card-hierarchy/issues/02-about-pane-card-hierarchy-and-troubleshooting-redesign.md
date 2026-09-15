---
id: SPEC-26-02
component: settings
satisfies: []
blocked_by: []
status: ready-for-agent
tests:
  - app::tests::about_pane_card_hierarchy_and_divider_structure
  - app::tests::about_pane_attribution_card_at_bottom
  - shortcut_row_slint_snapshot::tests::about_pane_merged_troubleshooting_card_renders_cleanly
  - shortcut_row_slint_snapshot::tests::factory_reset_confirmation_cancels_without_mutation
  - shortcut_row_slint_snapshot::tests::factory_reset_confirmation_stages_defaults_and_blocks_background_actions
---

# 02: About card hierarchy and subtle Troubleshooting reset action

**What to build:**
1. Reorganize About so Card 3 begins with Support development, Issue Tracker, and GitHub repository. Follow the action row with a full-bleed `CardDivider`, then place the Troubleshooting & Recovery section in the lower part of that same card.
2. Move publisher attribution/link, GPL-3.0 notice, in-process update/privacy disclosure, copyright, and third-party NOTICE pointer to a dedicated Card 4 that is the final About card.
3. Preserve the reset action as a design-system secondary control while moving it: subtle background (`Palette.bg_subtle`), `Palette.stroke_card` border, `Palette.bg_card_hover` on hover, normal secondary weight, and `Palette.signal_error` for text only. Do not introduce an error-filled/red-box background; the current source already supplies the required subtle visual primitives.
4. Preserve all callbacks and the existing confirmation modal contract. The only code path that invokes `factory_reset_defaults` remains confirmation; Escape or Cancel leaves the draft unchanged and the modal blocks background Save, Revert, navigation, pane actions, and external links.

**Blocked by:** None (can start immediately).

**Status:** ready-for-agent

## Acceptance Criteria

- [ ] Card 3 contains the three support/source actions before a full-bleed `CardDivider`, followed by Troubleshooting & Recovery in the same zero-padding card layout.
- [ ] The Reset all settings action retains subtle secondary card styling and error-colored text without an error-filled background.
- [ ] The standalone bottom Card 4 contains all attribution, license, in-process disclosure, copyright, and NOTICE content; its publisher link retains its current accessibility label and callback.
- [ ] No card follows Card 4 in About pane source order.
- [ ] Reset confirmation still opens as an overlay, Escape/Cancel leave the draft unchanged, confirm stages `Config::default()`, and all background actions remain blocked until dismissal.
- [ ] Existing external-link destinations and callback names remain unchanged.
- [ ] Tests named in frontmatter pass cleanly.
