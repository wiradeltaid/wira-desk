---
id: SPEC-25-01
component: settings
satisfies: [UC-4]
blocked_by: []
status: done
tests:
  - app::tests::restore_shortcuts_defaults_every_field_and_preserves_non_shortcut_preferences
  - app::tests::restore_shortcuts_is_dirty_only_when_draft_differs_from_saved
  - app::tests::revert_after_shortcut_restore_returns_to_the_saved_legacy_configuration
  - app::tests::fix_stack_conflict_requires_the_exact_pair_and_never_creates_a_third_conflict
  - shortcut_row_slint_snapshot::tests::header_restore_shortcuts_button_updates_the_draft
  - shortcut_row_slint_snapshot::tests::restore_shortcuts_discards_pending_percentage_and_cancels_capture
  - shortcut_row_slint_snapshot::tests::legacy_stack_conflict_banner_repairs_only_when_safe
---

# 01: Shortcuts pane — restore defaults button, conflict resolution banner, and tip removal

**What to build:**
1. Remove the obsolete reserved-key tip from `crates/settings/ui/panes/shortcuts_pane.slint`.
2. Add `[↺ Restore Shortcuts]` at the right of the Shortcuts header. It stages current defaults for
   all `ShortcutField::ALL` chords and enable flags, the four custom-edge percentages, and Stack
   width. It preserves General, visual-switcher, mouse, and VM-bypass values; it does not write
   configuration or signal the daemon.
3. Add an inline legacy-conflict panel only when enabled Overlapping Stack and Snap to bottom edge
   have equal chords. Show the title `Update a conflicting shortcut`, the body
   `“Overlapping window stack” used the same shortcut as “Snap to bottom edge.” Move it to its new
   default shortcut: Ctrl + Alt + Shift + S.`, and `[Update shortcut]` when Stack's default is not
   held by another enabled action. If another action owns that chord, retain an explanatory warning
   but disable the one-click repair; do not create a new duplicate.
4. Implement the model actions from `Config::default()` and `ShortcutField::ALL`, not copied chord
   literals. Each restore/repair callback must cancel capture, clear capture-swap state, discard
   pending numeric text, advance `revert_generation`, sync the UI, and leave persistence until
   Save Changes.

## Acceptance Criteria

- [ ] The obsolete tip paragraph is absent from `shortcuts_pane.slint`.
- [ ] Restore Shortcuts is reachable by keyboard/accessibility APIs at the header and stages every
  shortcut chord, enable flag, four edge percentages, and Stack width from current defaults.
- [ ] Restore Shortcuts preserves non-shortcut preferences, including visual-switcher enablement
  and hold delay; it marks the model dirty only when the restored draft differs from saved.
- [ ] An in-progress percentage edit or active capture cannot commit stale data after Restore
  Shortcuts; Revert returns exactly to the saved configuration without disk I/O.
- [ ] The banner appears only for the enabled Stack/Snap Bottom legacy pair. A safe Update shortcut
  assigns Stack `Ctrl+Alt+Shift+S` and removes that conflict; an occupied target does not mutate
  the draft and explains why manual choice is required.
- [ ] Unit and Slint snapshot tests named in frontmatter pass cleanly.
