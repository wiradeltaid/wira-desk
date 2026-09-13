# 01: Settings stepper centering and em-dash removal

**What to build:**

## 1. Hold Delay Threshold row spacing

In `crates/settings/ui/panes/general_pane.slint`, the row is the
`if root.visual_switcher_enabled : HorizontalLayout` block at lines 63-186. Change
`padding-bottom: 16px` to `padding-bottom: 12px`; `padding-top` is already `12px` and stays.

`min-height: 48px` is not the binding constraint — the row height is driven by the two-line text
column — so this is a real 4px reduction, which is what the owner asked for ("sangat sedikit saja").

## 2. Vertically centre the stepper cluster

The stepper is the second child of that row: `HorizontalLayout { spacing: 6px; alignment: end; ... }`
at lines 90-92, holding the `−` button, the value display, and the `+` button.

**Do not change `alignment: end` to `alignment: center`.** In Slint, a `HorizontalLayout`'s
`alignment` distributes children along the **main (horizontal)** axis. `end` is what pins the
stepper to the right edge of the row; `center` would move it inward and would not affect vertical
position at all.

Wrap the stepper `HorizontalLayout` in a `VerticalLayout { alignment: center; }`. This is the idiom
already used by the sibling text column in the same row (lines 71-74:
`VerticalLayout { spacing: 2px; alignment: center; horizontal-stretch: 1; }`), so the two halves of
the row are centred the same way and for the same reason.

## 3. Em-dash removal — six user-visible strings only

Scope is **what is drawn**, not what is in the file. Thirty-one `—` (U+2014) characters exist under
`crates/settings/ui/`; twenty-five of them are source comments, several being defect post-mortems
(`components/shortcut_row.slint:22-27`, `main_window.slint:203-219`). Leave every comment alone.

Replace exactly these six, with exactly these strings (pinned in SPEC.md so the test asserts a
decided string rather than an invented one):

| File:line | Replacement |
|---|---|
| `panes/about_pane.slint:382` | `"No telemetry, no account, no separate background service.\nUpdate checks run entirely in-process against GitHub Releases, which you can switch off."` |
| `components/key_check.slint:99` | `"Wira Desk isn't running. Only this window is being checked."` |
| `components/key_check.slint:142` | `"None yet"` (replaces the bare `"—"` placeholder) |
| `components/key_check.slint:161` | `...still receives it before that app. This shortcut will work.` |
| `components/key_check.slint:162` | `...is taking it first, often GPU software, Discord, or a game overlay. Try a different combination.` |
| `panes/shortcuts_pane.slint:164` | `...Wira Desk ships on Ctrl + Alt + ..., a good family for custom actions too.` |

`key_check.slint:142` is `text: root.last_display == "" ? "—" : root.last_display;` — an empty-state
glyph in a 170px keycap-shaped `Rectangle`, not punctuation. A bare `-` there reads as the Minus
**key**, so it becomes a word.

**The en dash is out of scope.** `(100–500 ms)` at `general_pane.slint:83` is U+2013. It must
survive untouched: `app::tests::visual_hold_delay_description_wraps_with_newline` asserts it verbatim
(`app.rs:3222`), and it is the layout SPEC-15-01 delivered and the owner accepted.

## 4. Tests

- Update the expected string inside `app::tests::about_pane_renders_in_process_disclosure_with_newline`
  (`crates/settings/src/app.rs:2999-3020`, literal at `:3011`) to the new About text.
  **Keep the test name.** SPEC-15-01's registry row names this test (`specs.yaml:1447`); renaming it
  leaves that row pointing at a test that no longer exists, and `validate.py` only checks that a
  ticket's `tests:` list is non-empty (`validate.py:477`), so nothing would catch the dangling name.
- Add `app::tests::all_visible_slint_strings_have_zero_em_dashes`: `include_str!` each of the seven
  `.slint` files under `crates/settings/ui/` that currently contain U+2014, and assert zero U+2014
  **inside string literals** (`text:` and `accessible-label:` values). Comments are exempt by design;
  the test must not be a whole-file scan, or it permanently bans the character from this crate's
  comments and forces twenty-five unrelated rewrites.
- Add `app::tests::general_pane_stepper_is_vertically_centered`: follow the existing `include_str!`
  source-assertion idiom already used by `visual_hold_delay_description_wraps_with_newline`
  (`app.rs:3216-3224`) — assert the stepper cluster sits inside a `VerticalLayout` with
  `alignment: center`, and that `alignment: end` is still present on the stepper `HorizontalLayout`.
  A Rust unit test cannot measure Slint layout geometry; assert the structure that produces it, and
  say so in the test's own comment rather than letting the name overclaim.
- `scripts/verify-settings-runtime.ps1` still passes.

**Blocked by:** None

**Status:** ready-for-agent

- [ ] `general_pane.slint` Hold Delay row is `padding-top: 12px; padding-bottom: 12px`.
- [ ] The stepper cluster is wrapped in `VerticalLayout { alignment: center; }`, and its
      `HorizontalLayout` still carries `alignment: end`.
- [ ] The six user-visible strings above match the pinned replacements exactly.
- [ ] `(100–500 ms)` (U+2013) is unchanged, and `visual_hold_delay_description_wraps_with_newline`
      still passes.
- [ ] No `.slint` **comment** was rewritten.
- [ ] `about_pane_renders_in_process_disclosure_with_newline` keeps its name and asserts the new string.
- [ ] `all_visible_slint_strings_have_zero_em_dashes` passes and is scoped to string literals.
- [ ] `scripts/verify-settings-runtime.ps1` passes.
