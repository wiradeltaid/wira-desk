# SPEC-26: Header Defaults Buttons, Conditional Visibility, and About Pane Card Hierarchy Polish

## Problem Statement

The Settings panes do not use one consistent, contextual reset pattern. Shortcuts exposes an unconditional restore action even after its draft matches factory defaults, while General and Mouse lack a pane-scoped way to restore their displayed preferences.

About also separates its support actions from recovery and places attribution above the last content card. The existing reset action already uses the design system's subtle secondary treatment; its hierarchy must remain subtle when it moves into the unified support-and-recovery card rather than becoming an error-filled destructive control.

## Solution

1. **Uniform conditional `↺ Defaults` header actions**
   - Use the same icon-plus-text action, `↺ Defaults`, in Shortcuts, General, and Mouse. Each button has its pane-specific accessible label.
   - Derive visibility from the active `draft`, never from dirty state or `saved`: a pane whose draft already equals its factory-owned values shows no button even if saved preferences are custom and Revert remains available.
   - General owns exactly `auto_start`, `switcher.visual_enabled`, and `switcher.visual_hold_delay_ms` for this action. `general.check_updates` remains outside it because its control lives in About and this SPEC does not add an About defaults action.
   - Mouse owns exactly `enabled`, `thumb_back`, `thumb_forward`, `tilt_left`, and `tilt_right`.
   - Shortcuts owns every `ShortcutField` chord and enable flag, the four snap percentages, and `stack_width_percent`.
   - Each restore action changes only those owned values in the in-memory draft. It cancels capture, clears capture-derived conflict/swap state and feedback, clears any pending percentage text held by the view callback, increments the view's `revert_generation`, and does not write disk configuration or signal the daemon. The model derives dirty state normally from `draft != saved`; it must not force it.

2. **About card hierarchy and recovery affordance**
   - Card 3 contains the Support development, Issue Tracker, and GitHub repository action row first, then a full-bleed `CardDivider`, then Troubleshooting & Recovery.
   - Retain the reset button as a subtle secondary action: `Palette.bg_subtle`, `Palette.stroke_card`, hover `Palette.bg_card_hover`, normal secondary typography, and `Palette.signal_error` only for its text. It must not gain an error background.
   - Card 4 is the final About card and contains the publisher attribution/link, GPL-3.0 notice, in-process update/privacy disclosure, copyright, and NOTICE pointer.
   - The confirmation modal remains the only route to `factory_reset_defaults`; Escape/cancel leaves the draft unchanged, background interaction remains blocked while open, and confirmation stages `Config::default()` only until Save Changes.

## User Stories

1. As a user on General with customized displayed preferences, I can restore only General defaults from its header.
2. As a user on General whose displayed preferences already match factory defaults, I do not see a redundant defaults action.
3. As a user on Mouse with customized mappings, I can restore only mouse defaults from its header.
4. As a user on Mouse whose mappings match factory defaults, I do not see a redundant defaults action.
5. As a user on Shortcuts, I see a concise defaults action only while shortcut-owned draft values differ from factory defaults.
6. As a user exploring About, I find support and recovery actions in one coherent card.
7. As a user reviewing application information, I find attribution, licensing, and privacy disclosure in a standalone final card.
8. As a user considering reset, I see a destructive-label action that remains visually secondary until its confirmation dialog.

## Implementation Decisions

- Add pure `SettingsModel` derivations: `general_differs_from_default`, `mouse_differs_from_default`, and `shortcuts_differ_from_default`. They compare only the ownership sets above against a fresh `Config::default()`.
- Add `restore_general_defaults` and `restore_mouse_defaults`; retain `restore_shortcuts_defaults`. All three are staged model operations that cancel capture and clear stale feedback/swap state, but only the UI callback owns clearing pending `TextInput` state and incrementing `revert_generation` after a restore.
- Pass `show_defaults_button` and `restore_defaults_clicked` through GeneralPane, MousePane, and ShortcutsPane. The buttons are not instantiated when false, have `accessible-role: button`, and preserve the exact labels: `Restore General settings to defaults`, `Restore Mouse settings to defaults`, and `Restore shortcuts to defaults`.
- Re-sync conditional visibility after every draft mutation, Revert, and confirmed full factory reset. A stale button that remains visible after its action restores the pane is a contract failure.
- Restructure `about_pane.slint` without changing the external-link callbacks, modal callbacks, or reset semantics. The publisher link remains reachable from the new final card.

## Testing Decisions

- `app::tests` proves each ownership-scoped restore, all three false/true default derivations, dirty-state derivation, Revert restoration, and that `check_updates` is preserved by General defaults.
- Slint snapshot tests prove each header button is absent at the factory-owned draft, visible after a relevant draft mutation, invokes only its pane restore, clears pending percentage/capture state when applicable, and disappears after the restored draft is re-synced.
- About tests prove Card 3 ordering and divider placement, Card 4 is final and retains attribution/link accessibility, the reset control uses the subtle secondary tokens rather than an error fill, and the existing confirmation, Escape, and background-blocking regressions stay green.

## Out of Scope

- A defaults action for VM & Exceptions or About update consent.
- Changing confirmation-dialog wording, factory-reset scope, persistence boundaries, or the configuration schema.
- Altering external-link destinations or update behavior.

## Further Notes

All changes remain staged in the in-memory draft until the user explicitly clicks **Save Changes** in the footer.
