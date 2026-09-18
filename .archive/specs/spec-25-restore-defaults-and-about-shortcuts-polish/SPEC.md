# SPEC-25: Restore Defaults, Conflict Resolution Banner, and About/Shortcuts UI Polish

## Problem Statement

A legacy on-disk configuration can retain the former Overlapping Stack chord
`Ctrl+Alt+Shift+Down`. That chord is now the default for Snap to bottom edge, so startup
keeps Snap Bottom and leaves Stack unbound under `DEC-009`. The warning is in the log, while
Settings offers no focused recovery path. The existing **Revert** action only restores the
saved configuration over an unsaved draft; it cannot repair a collision already stored on disk.

The Shortcuts pane also carries an obsolete reserved-key tip that crowds the header. In About,
the combined GitHub link does not distinguish source browsing from issue reporting, and there is
no draft-safe way to restore all persisted preferences.

## Solution

1. **Restore Shortcuts:** Add a secondary `[↺ Restore Shortcuts]` action at the right of the
   Shortcuts heading. It restores only shortcut-owned values in the in-memory draft: every
   `ShortcutField::ALL` chord and enable flag, the four custom-edge percentages, and the
   Overlapping Stack width. It must not reset General settings, visual-switcher enablement or
   hold delay, mouse mappings, or VM bypass preferences.
2. **Known legacy-conflict banner:** Show an inline warning only when the enabled Overlapping
   Stack and Snap to bottom edge actions hold the same chord. Its normal title, body, and action
   are `Update a conflicting shortcut`, `“Overlapping window stack” used the same shortcut as
   “Snap to bottom edge.” Move it to its new default shortcut: Ctrl + Alt + Shift + S.`, and
   `[Update shortcut]`. The action is available only when no other enabled action already owns
   `Ctrl+Alt+Shift+S`; otherwise it explains that the user must choose a unique Stack shortcut.
   A one-click repair must never replace one duplicate with another.
3. **Tip removal:** Remove the obsolete Windows-reserved-key tip from the Shortcuts pane.
4. **About action row:** Replace the combined GitHub link and separate support row with one
   horizontally aligned row: `[Support development]`, `[Issue Tracker]`, and an icon-only
   GitHub repository button. The visible GitHub icon button still has the accessible name
   `GitHub repository`.
5. **Restore all preferences:** Add a **Troubleshooting & Recovery** card with
   `[Reset all settings…]`. Its confirmation dialog calls this a restoration of saved
   preferences, not a file-deleting factory reset: confirmation replaces the draft with
   `Config::default()`; it does not delete `config.toml`, re-run onboarding, or alter disk or
   the scheduled task until the user chooses **Save Changes**.

## User Stories

1. As a user upgrading with the known Stack/Snap Bottom collision, I can repair Stack to its new
   default without losing unrelated custom shortcuts.
2. As a user who wants a fresh shortcut setup, I can restore shortcut chords, enable flags, and
   shortcut percentages without changing non-shortcut preferences.
3. As a user reviewing Shortcuts, I see a focused header without obsolete reserved-key advice.
4. As an About-pane user, I can separately open the support landing page, issue tracker, and
   GitHub repository from balanced, accessible buttons.
5. As a user with a misconfigured saved setup, I can stage a restoration of every preference,
   inspect it, then either save it or revert it.

## Draft, Revert, and Conflict Contract

- Every restore or repair action changes only `SettingsModel::draft`; it never calls
  `Config::save`, `save_and_notify`, or daemon IPC. `is_dirty` remains derived from
  `draft != saved`: pressing a restore control on an already identical draft remains clean.
- Before a restore or repair mutates the draft, it cancels capture, clears any stale
  capture-swap state, discards the shared uncommitted percentage buffer, and advances
  `revert_generation`. The focused `TextInput` must therefore show the restored value instead
  of committing its old pending value on a later Save.
- **Revert** restores exactly `saved`, including a legacy collision, and leaves disk unchanged.
  A successful Save promotes the staged values to `saved`; a rejected or failed Save does not.
- The Stack repair is deliberately narrower than generic conflict handling. It requires the exact
  enabled Stack/Snap Bottom pair, clears that pair by assigning Stack its `DEC-011` default, and
  refuses to introduce a collision with any third enabled field.

## Implementation Decisions

- Add `restore_shortcuts_defaults`, `can_fix_stack_conflict`, and `fix_stack_conflict` to
  `SettingsModel`; derive the restore values from `Config::default()` and
  `ShortcutField::ALL`, not duplicated chord literals. Copy individual shortcut-owned fields so
  `SwitcherConfig::visual_enabled` and `visual_hold_delay_ms` stay untouched.
- Add explicit Shortcuts callbacks and derived properties through `MainWindow`,
  `ShortcutsPane`, `bind_callbacks`, and `sync_model_to_ui`. The banner condition is derived in
  Rust from the draft, not maintained as a separate mutable Boolean.
- Add `open_issues_url` alongside the existing publisher, source, and support callbacks. The
  URLs are `https://wiradigital.id/wira-desk`,
  `https://github.com/wiradigitalid/wira-desk/issues`, and
  `https://github.com/wiradigitalid/wira-desk`. They are already within the HTTPS allowlist.
  **Support development** is the established product wording; no repository artifact supports a
  `Donate` label or a claim about third-party donation providers, so neither is introduced.
- Put the confirmation surface in the window-level overlay layer, not inside the scrolling About
  pane. It must block background actions, expose a clear accessible title and destructive-action
  explanation, keep focus inside while open, cancel on Escape/Cancel without mutation, and invoke
  the reset exactly once on Confirm.

## Testing Decisions

- `app::tests` prove shortcut restoration covers every field and enable flag, all five shortcut
  percentages, preserves non-shortcut preferences, and is dirty only when it differs from saved.
  They also prove Revert restores the saved legacy configuration without a write; reset is draft
  only, restores every `Config` section, and Revert restores the pre-reset saved configuration.
- UI tests prove Restore Shortcuts is reachable in the header; an in-progress numeric edit and
  active capture cannot overwrite a restored draft; the exact legacy banner appears and a safe
  repair removes it; disabled actions and a third-party collision do not offer an unsafe repair.
- About tests prove all three action buttons are discoverable by accessible name and invoke their
  independent callbacks. Confirmation tests prove opening/cancelling reset is non-mutating,
  Confirm stages the default draft, background controls cannot fire while it is open, and the
  footer enables Save Changes only when the staged default differs from saved.
- Existing validation and persistence suites remain green, including duplicate-chord rejection on
  Save and the daemon reload contract.

## Out of Scope

- Rewriting configurations during installer execution or daemon startup.
- Direct donation-platform URLs or unverified donation-provider claims in the binary.
- Deleting `config.toml`, re-running onboarding, or creating an undo history beyond the existing
  draft / Save Changes / Revert lifecycle.
