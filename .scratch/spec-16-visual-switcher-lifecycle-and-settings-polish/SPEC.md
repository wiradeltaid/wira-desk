---
spec: SPEC-16
release: "0.4.0"
prd: wira-desk
fr: []
status: closed
---

# SPEC-16 — Visual Switcher Start Menu Suppression, WinUI Helper Exclusion, UI Em-Dash Removal, and Stepper Spacing Polish

## Problem Statement

Following manual testing of SPEC-15 on Windows 11 desktop, the product owner reported four
user-visible defects. Each is restated here against the code it lands in, not against the draft's
own account of that code.

1. **Hold Delay row spacing and stepper vertical centering.**
   The Hold Delay Threshold row (`crates/settings/ui/panes/general_pane.slint:63-186`) is
   `padding-top: 12px; padding-bottom: 16px`. The owner reports the bottom as very slightly too
   tall. Separately, the stepper cluster (`−`, value, `+`) is not vertically centred against the
   row: the sibling text column is a `VerticalLayout` with `alignment: center`
   (`general_pane.slint:71-74`), while the stepper is a bare `HorizontalLayout` with no cross-axis
   container at all (`general_pane.slint:90-92`).

2. **Em-dashes in the visual application.**
   The owner asked for every em-dash (`—`, U+2014) to be removed from the visible application.
   Six such characters sit in user-visible strings; the other twenty-five occurrences in the same
   files are source comments and are **not** in scope (see Implementation Decisions).

3. **Start Menu opens when the visual switcher is committed with `Win`, and switching is
   intermittent.**
   Holding `Win + ~` past the threshold and releasing `Win` to commit opens the Windows Start Menu,
   and the window switch itself succeeds only sometimes. `Escape` cancel and `Alt + ~` are reported
   clean.

   **Root cause.** The hook swallows the main key on both edges (`hook.rs:666-670`, `hook.rs:492`),
   so the shell sees an uninterrupted `Win` down followed by `Win` up and treats it as a lone
   Windows-key tap. `suppress_start_menu()` (`worker.rs:42`) exists to break exactly that pattern by
   injecting one `VK_NONAME` while `Win` is physically down. Before SPEC-15-04 it ran as part of the
   keydown cycle; decoupling moved the cycle to the release edge, so nothing injects during the hold.

   **What the first draft of this spec got wrong, and it changes the fix.**
   `suppress_start_menu()` has **not** been removed from the commit path: `worker.rs:443` still calls
   it inside `Command::SwitcherCommit`. It is nevertheless a no-op, and always has been on this path.
   The commit is enqueued *because* `Win` came up — from the hook's modifier-keyup branch
   (`hook.rs:448-458`) or from the watchdog's `!are_any_modifiers_down()` (`worker.rs:233`) — so by
   the time the Worker drains the command, `suppress_start_menu()`'s own guard
   (`worker.rs:66-70`, `GetAsyncKeyState(VK_LWIN|VK_RWIN) & 0x8000`) is false and it returns
   immediately. **Suppression on the release edge is unreachable by construction.** The injection has
   to happen while the key is still down, which means at arm time or at hold-timer fire, never at
   commit.

4. **Modern Notepad / WinUI 3 helper surfaces appear as ghost cards.**
   SPEC-15-02 added `PopupHost` and `Xaml_WindowedPopupClass` to `HELPER_SURFACE_CLASSES`
   (`cycling/mod.rs:48`). Live WinProbe inspection of Windows 11 `Notepad.exe` shows the actual
   surfaces carry class `Microsoft.UI.Content.PopupWindowSiteBridge` and window text `Pop-upHost`,
   owned by the main Notepad window, with a small extent (e.g. 38x47). The class is not in the list,
   so `is_helper_surface()` (`cycling/mod.rs:130`) returns false and the windows stay eligible.

---

## Solution

1. **SPEC-16-01 — Settings stepper centering, row spacing, and em-dash removal.**
   Balance the Hold Delay row to `padding-top: 12px; padding-bottom: 12px`, and wrap the stepper
   cluster in a `VerticalLayout { alignment: center; }` — the same idiom the sibling text column
   already uses. Remove the six user-visible em-dashes, with each replacement string pinned below.

2. **SPEC-16-02 — Start Menu suppression while the chord is held.**
   Call `suppress_start_menu()` from the Worker at the two points where `Win` is still physically
   down: at the top of `execute_switcher(Command::SwitcherArm | Command::SwitcherArmPrev)`
   (`worker.rs:357`), before the `decide_switcher_hold_delay` gate, and at the top of
   `open_visual_switcher` (`worker.rs:249`), before both of its early returns. One injection anywhere
   inside a `Win` press is sufficient — the shell's lone-Win test asks whether *any* other key was
   pressed during that press — so arming covers commit, cancel, disarm, and the no-eligible-candidate
   path with a single call.

   The dead call at `worker.rs:443` is removed, with a comment recording why the release edge cannot
   work, so that it is not reinstated by a future reader.

3. **SPEC-16-03 — WinUI 3 popup bridge exclusion, by class only.**
   Add `CLASS_POPUP_WINDOW_SITE_BRIDGE = "Microsoft.UI.Content.PopupWindowSiteBridge"` to
   `HELPER_SURFACE_CLASSES` and a matching fixture to `expected_decisions()`. Nothing else changes:
   `is_helper_surface()` is already consulted by both the production policy
   (`eligibility.rs:49-51`) and `ReferencePolicy` (`cycling/mod.rs:480-482`), so parity between
   blind cycling and the switcher holds by construction.

---

## User Stories

1. **As a user configuring Wira Desk**, I want the Hold Delay Threshold stepper vertically centred
   and the bottom spacing balanced, so that the control looks deliberate rather than nudged.
2. **As a user reading the application**, I want no em-dashes in what is drawn on screen.
3. **As a keyboard multitasker using `Win + ~`**, I want releasing `Win` to switch windows without
   the Start Menu opening and without the switch failing.
4. **As a Windows 11 user**, I want modern Notepad's popup bridge surfaces excluded from the
   switcher, so only real document windows appear as cards.

---

## Implementation Decisions

### Em-dash scope is what is drawn, not what is in the file

Thirty-one `—` characters exist under `crates/settings/ui/`. Six are in user-visible strings; the
remaining twenty-five are source comments, several of which are defect post-mortems
(`components/shortcut_row.slint:22-27`, `main_window.slint:203-219`). Rewriting those buys nothing
the owner asked for and degrades the record, so the guard asserts zero U+2014 **inside string
literals** (`text:` and `accessible-label:` values), not zero per file.

The scope is genuinely bounded by `crates/settings/ui/`. The only other U+2014 in a string literal
anywhere under `crates/` is `worker.rs:1003`, inside `debug_log`, which `util.rs:32-34` documents as
a developer diagnostic path distinct from user-facing logging. Nothing in the daemon tray, the
overlay, or the installer carries one.

### The six replacement strings are pinned here, not left to the implementer

A test will assert whatever string ships, so the string is decided in the spec rather than ratified
after the fact.

| Site | Now | Becomes |
|---|---|---|
| `panes/about_pane.slint:382` | `...background service —\nupdate checks run entirely...` | `...background service.\nUpdate checks run entirely...` |
| `components/key_check.slint:99` | `Wira Desk isn't running — only this window is being checked.` | `Wira Desk isn't running. Only this window is being checked.` |
| `components/key_check.slint:142` | `"—"` (empty-state keycap glyph) | `"None yet"` |
| `components/key_check.slint:161` | `...before that app — this shortcut will work.` | `...before that app. This shortcut will work.` |
| `components/key_check.slint:162` | `...taking it first — often GPU software...` | `...taking it first, often GPU software...` |
| `panes/shortcuts_pane.slint:164` | `...Ctrl + Alt + ... — a good family...` | `...Ctrl + Alt + ..., a good family...` |

`key_check.slint:142` is the one that is not punctuation: the em-dash is the empty-state placeholder
inside a 170px keycap-shaped `Rectangle`, drawn in `text_tertiary` on `bg_subtle`. A bare `-` there
reads as the Minus **key**, which is the opposite of "nothing captured yet", so the placeholder
becomes a word.

### The en dash stays

`(100–500 ms)` (`general_pane.slint:83`) is U+2013, not U+2014, and it is asserted verbatim by
`app::tests::visual_hold_delay_description_wraps_with_newline` (`app.rs:3222`) — the layout
SPEC-15-01 delivered and the owner accepted. It is **out of scope**; the guard matches U+2014 only.

### Slint `alignment` is a main-axis property

`HorizontalLayout { alignment: ... }` distributes children horizontally. The stepper cluster is
already `alignment: end` to pin it to the right edge of the row; changing that to `center` would
move it off the right edge and do nothing vertically. Vertical centring inside a horizontal row is
done by wrapping in a `VerticalLayout { alignment: center; }`, which is exactly how the sibling text
column two blocks above it is built.

### Suppression runs on the Worker, never in the hook callback

`worker.rs:31-41` carries this as a stated SAFETY precondition, earned from a live regression:
calling `SendInput` from inside the low-level keyboard hook re-enters input processing, raced the
activation, and stopped cycling from moving focus at all. SPEC-16-02 adds no `SendInput` — and no
call that reaches one — to `hook.rs`.

### Exclusion is by class; title and owner are not touched

`WindowFacts` (`cycling/mod.rs:104-121`) carries `has_title: bool`, not a title string, so
"exclude windows titled `Pop-upHost`" cannot be expressed without widening the `WindowFacts`
contract. DEC-026's closing note already records that widening as the owner's call, not an
implementer's. It is also unnecessary — the class is stable and diagnosed — and it would be a
hazard: a document literally named `Pop-upHost` would vanish from the switcher.

For the same reason SPEC-16-03 does **not** exclude on `is_owned` or on extent. SPEC-15-02
deliberately left those deferred, and three tests assert the deferral on purpose:
`empty_title_window_is_still_eligible`, `zero_extent_window_is_still_eligible`,
`owned_window_is_still_eligible` (`eligibility.rs:395,407,419`). Reversing them is a decision
against DEC-026 clause 3's recorded state, and it needs a new `DEC-`, not a ticket.

---

## Open questions and debt carried

1. **The second half of owner note 3 is not proven to share a cause.** The owner reported the Start
   Menu *and* "glitch tidak stabilnya perpindahan switcher, kadang bisa kadang tidak". The focus
   race with `StartMenuExperienceHost` is a coherent single explanation and is the working
   hypothesis — but it is inferred, not measured. A second mechanism produces the same pair: when
   `open_visual_switcher` takes either early return (`worker.rs:267`, `worker.rs:277`) the overlay
   never opens, `switcher_active` stays false, and the swallowed main-key release past the deadline
   enqueues nothing (`hook.rs:474-482`) — no overlay, no switch, and no injection, so Start opens.
   Suppression at arm time closes the Start Menu half of that path regardless. SPEC-16-02's
   acceptance therefore tests the **switch** as well as the Start Menu; if the intermittency
   survives, it is a separate diagnosis under `wdi-systematic-debugging`, not a reopening of this
   spec.

2. **Escape-cancel is asserted safe without a mechanism.** `VK_ESCAPE` is mapped at `hook.rs:539`
   and its branch returns `KeyHandleResult::Swallow` (`hook.rs:564-567`), exactly as the main key
   does, and `Command::SwitcherCancel` (`worker.rs:445`) never suppresses. By the causal model
   above, Escape-cancel should open Start too. Either the model is incomplete or the owner's test
   did not reach that state. Arm-time placement makes the question moot in practice; it is recorded
   here because it is the one observation in the report that the code does not explain.

3. **`fr: []` is inherited debt, not a claim.** The visual switcher still has no FR and no
   capability in the corpus — the same state SPEC-13, SPEC-14 and SPEC-15 recorded. SPEC-16 adds no
   new promise, and it does not discharge that debt either. Owner's call at G2.
