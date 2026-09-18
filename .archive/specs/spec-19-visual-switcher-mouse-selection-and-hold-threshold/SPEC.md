# SPEC-19: Visual Switcher Mouse Selection Synchronization & Default Hold Delay Threshold

## Problem Statement

When using the same-application visual switcher overlay (`Win + ~` or `Alt + ~`), clicking on a window card using the mouse cursor does not reliably activate the clicked window as the active foreground focus. Instead, it frequently activates whatever window was initially selected by the keyboard chord when the overlay opened. In contrast, keyboard navigation (hotkeys or arrow keys) selects and activates windows consistently. Additionally, the default visual hold delay threshold of 150 ms is too aggressive for daily driving, often triggering the visual overlay when the user intended a rapid blind cycle.

## Solution

1. Synchronize visual switcher selection state so that mouse hover and mouse click interactions on the overlay window properly update the controller's candidate selection. When a card is clicked or committed, the controller must commit the window target that matches the overlay's selected card.
2. Update the default visual hold delay threshold from 150 ms to 300 ms across shared configuration, daemon defaults, settings UI defaults, and documentation, providing a more comfortable margin for blind cycling while keeping hold activation prompt and predictable.

## User Stories

1. As a keyboard and mouse user, I want to click any window card displayed in the visual switcher overlay with my mouse cursor, so that the clicked window reliably becomes the active foreground window.
2. As a visual switcher user, I want hovering my mouse over different window cards to update the current selection highlight, so that the visual presentation accurately reflects which window will be selected upon click or commit.
3. As a visual switcher user, I want clicking a card on paginated overlays (pages beyond page 1) to accurately compute the global candidate index, so that the correct target window across pages is activated without index truncation or offset errors.
4. As a user who rapidly switches between same-app windows, I want a default hold delay threshold of 300 ms instead of 150 ms, so that rapid blind cycling keystrokes do not inadvertently trigger the visual overlay.
5. As a settings user, I want the Settings General pane default hold delay value to reflect 300 ms, so that fresh configurations and reset states match the runtime default.
6. As a user with custom configured hold delay thresholds between 100 ms and 500 ms, I want existing saved configuration values to be respected without being overwritten by the new default.

## Implementation Decisions

1. **Selection State Single Source of Truth / Synchronization:**
   - Expose accessor `pub fn selected_index(&self) -> usize` on `SwitcherOverlay`.
   - In `SwitcherController`, delegate `selected_index()` and `candidates_from_selection()` directly to `self.overlay.selected_index()` (or synchronize `self.selected_index` on every read/write), eliminating the duplicate, desynchronized state between the controller and the overlay.
   - When handling `WM_MOUSEMOVE` or `WM_LBUTTONDOWN` in the overlay window procedure, updating `overlay.set_selected_index(candidate_idx)` immediately takes effect for any subsequent commit.
   - In `candidates_from_selection()`, return candidate targets ordered starting from the overlay's current selected index, guaranteeing that fallback activation targets the window the user clicked or hovered.
2. **Default Hold Delay Threshold:**
   - Update `SwitcherConfig::DEFAULT_HOLD_DELAY_MS` in shared configuration from `150` to `300`.
   - Update `visual_hold_delay_ms` initial property default in the Settings UI (Slint `general_pane` and `main_window`) to `300`.
   - Update daemon and settings unit tests to assert the new default of `300` ms while keeping the allowed clamping band (`100..=500` ms) intact.

## Testing Decisions

- Test that clicking a card in the visual switcher overlay updates the controller's candidate selection so that `candidates_from_selection()` begins with the clicked window ID.
- Test that hovering over a card in the visual switcher overlay updates selection so that a subsequent keyboard modifier release commit (`Command::SwitcherCommit`) activates the hovered window.
- Test that clicking cards across multi-page layouts computes the correct candidate index with `page_start` offsets.
- Test that default configuration initialization across shared and settings models yields `300` ms for `visual_hold_delay_ms`.
- Test that clamping logic preserves custom values within `100..=500` ms.

## Out of Scope

- Changing the visual switcher card layout geometry or aspect ratio calculation.
- Changing the minimum (100 ms) or maximum (500 ms) hold delay boundary limits.
- Changing blind backward or forward cycling session state machines.

## Further Notes

- Root cause of mouse click inconsistency: `SwitcherController` maintained a duplicate `selected_index` field that was updated during keyboard navigation (`next`, `prev`, `up`, `down`) but was bypassed when `SwitcherOverlay::wnd_proc` handled `WM_MOUSEMOVE` and `WM_LBUTTONDOWN`. On `SwitcherCommit`, the worker queried `SwitcherController::candidates_from_selection()`, which used the stale controller index.
