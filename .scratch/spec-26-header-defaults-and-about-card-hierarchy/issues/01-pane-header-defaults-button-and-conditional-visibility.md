---
id: SPEC-26-01
component: settings
satisfies: [UC-4, UC-14]
blocked_by: []
status: ready-for-agent
tests:
  - app::tests::restore_general_defaults_resets_only_general_preferences
  - app::tests::restore_mouse_defaults_resets_only_mouse_preferences
  - app::tests::pane_defaults_differ_from_default_detection
  - app::tests::general_defaults_preserves_check_updates_preference
  - shortcut_row_slint_snapshot::tests::general_pane_defaults_button_visibility_and_action
  - shortcut_row_slint_snapshot::tests::mouse_pane_defaults_button_visibility_and_action
  - shortcut_row_slint_snapshot::tests::shortcuts_pane_defaults_button_visibility_and_action
---

# 01: Conditional header Defaults actions for Shortcuts, General, and Mouse

**What to build:**
1. Replace the Shortcuts header action text `↺ Restore shortcuts` with icon-plus-text `↺ Defaults`, retaining accessible label `Restore shortcuts to defaults`.
2. Add the same visual pattern to General and Mouse with accessible labels `Restore General settings to defaults` and `Restore Mouse settings to defaults`.
3. Bind each button's existence to a pure comparison of the active draft with `Config::default()`, not to dirty state or the saved baseline:
   - General: `auto_start`, `switcher.visual_enabled`, and `switcher.visual_hold_delay_ms` only. `general.check_updates` is intentionally retained because it is controlled in About and no About defaults action is in scope.
   - Mouse: `enabled`, `thumb_back`, `thumb_forward`, `tilt_left`, and `tilt_right` only.
   - Shortcuts: every `ShortcutField` chord and enable flag, all four snap percentages, and `layout.stack_width_percent`.
4. Add model methods for each comparison and for General/Mouse restoration. Restores must update only their ownership set, reset capture/last-capture/feedback state, and leave `saved`, disk, and daemon state untouched.
5. In every restore callback, clear the shared uncommitted percentage buffer before mutating the model, then increment the Slint `revert_generation` after the restore so a stale `TextInput` cannot later overwrite the restored draft. Re-sync the UI immediately.
6. Ensure normal sync after any draft change, Revert, or confirmed full reset recalculates visibility. When a restore makes the pane equal its factory-owned defaults, the button disappears even if the whole model remains dirty because another pane differs from `saved`.

**Blocked by:** None (can start immediately).

**Status:** ready-for-agent

## Acceptance Criteria

- [ ] Each instantiated header action renders `↺ Defaults`, has `accessible-role: button`, and has its exact pane-specific accessible label.
- [ ] No defaults button is instantiated when that pane's owned draft fields equal `Config::default()`; changing any one owned field makes it appear after sync.
- [ ] A custom saved baseline with a factory-default draft does not make a button appear solely because the model is dirty.
- [ ] General restore changes only `auto_start`, `visual_enabled`, and `visual_hold_delay_ms`; it preserves `check_updates`, shortcuts, mouse, and VM preferences.
- [ ] Mouse restore changes only its master switch and four presets; it preserves General, shortcuts, and VM preferences.
- [ ] Shortcuts restore continues to restore all chords, enable flags, four snap percentages, and stack width while preserving non-shortcut preferences.
- [ ] Every restore is draft-only, clears stale capture/pending percentage state, leaves dirty state derived, and Revert restores the prior saved configuration.
- [ ] Unit and Slint snapshot tests named in frontmatter pass cleanly.
