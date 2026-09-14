---
id: SPEC-25-02
component: settings
satisfies: []
blocked_by: []
status: ready-for-agent
tests:
  - app::tests::factory_reset_restores_every_config_section_in_the_draft
  - app::tests::factory_reset_is_draft_only_and_revert_restores_saved_preferences
  - app::tests::about_pane_renders_three_independent_action_buttons
  - shortcut_row_slint_snapshot::tests::factory_reset_confirmation_cancels_without_mutation
  - shortcut_row_slint_snapshot::tests::factory_reset_confirmation_stages_defaults_and_blocks_background_actions
---

# 02: About pane — three-button link row and restore-all-preferences card

**What to build:**
1. Replace the combined GitHub text link and separate support row in
   `crates/settings/ui/panes/about_pane.slint` with one horizontal action row:
   - `[Support development]` opens `https://wiradigital.id/wira-desk`.
   - `[Issue Tracker]` opens `https://github.com/wiradigitalid/wira-desk/issues`.
   - An icon-only GitHub repository button opens `https://github.com/wiradigitalid/wira-desk` and
     has accessible label `GitHub repository`.
   Add and bind an independent `open_issues_url` callback; do not overload the repository action.
   Keep **Support development** as the established label. Do not introduce a `Donate` label,
   direct donation-platform URL, or unverified claim about a donation provider.
2. Add a **Troubleshooting & Recovery** card titled **Restore all preferences to defaults** with
   `[Reset all settings…]`. Its copy states that confirmation stages clean-install defaults for all
   saved preferences and that no file, scheduled task, or running daemon changes until **Save
   Changes** is chosen.
3. Place the confirmation dialog in the window-level overlay. It must describe that General,
   visual-switcher, Shortcuts, Mouse, and VM-bypass preferences will be restored; it must keep
   keyboard focus inside, block background interaction, cancel on Escape/Cancel without a model
   mutation, and invoke reset once on Confirm.
4. Confirming calls `SettingsModel::factory_reset_defaults`, sets `draft = Config::default()`,
   cancels capture, clears pending percentage and capture-swap state, advances `revert_generation`,
   and syncs the UI. It does not delete `config.toml`, re-run onboarding, persist, or signal the
   daemon. Revert restores `saved`; Save Changes is the sole persistence boundary.

## Acceptance Criteria

- [ ] About exposes the three horizontal actions by the accessible names `Support development`,
  `Issue Tracker`, and `GitHub repository`; each invokes its own allowed HTTPS URL callback.
- [ ] The action row fits the 760 px normal window width without horizontal scrolling or clipped
  controls.
- [ ] The restore card and accessible confirmation dialog explain the staged-only consequence and
  name the preferences being reset.
- [ ] Opening, cancelling, or pressing Escape in the dialog leaves the draft, saved config, disk,
  and daemon signal unchanged. Background controls cannot trigger while it is open.
- [ ] Confirming resets every `Config` section in memory. Save Changes persists and signals as
  usual; Revert instead restores the exact pre-reset saved configuration.
- [ ] All tests named in frontmatter pass cleanly.
