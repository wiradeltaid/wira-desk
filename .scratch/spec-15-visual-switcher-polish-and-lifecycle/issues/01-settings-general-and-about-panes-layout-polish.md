# 01: Settings General and About panes layout polish

**What to build:**
UI aesthetics and layout polish across `general_pane.slint` and `about_pane.slint`:

1. **General Pane Hold Delay Threshold:**
   - In `crates/settings/ui/panes/general_pane.slint:82`, update the description for Hold Delay Threshold so that `(100–500 ms)` is placed on its own line beneath `Milliseconds to hold the chord before the visual overlay appears`.
   - Increase the vertical padding and breathing room of the Hold Delay Threshold container row (e.g. `padding-top: 12px`, `padding-bottom: 16px`, `min-height: 48px`) so that the row is not cramped against the bottom border of Card 2.
   - Keep `wrap: word-wrap;` on the caption — `app::tests::every_caption_in_the_general_pane_sets_a_wrap_mode` scans the source for it on every caption.

2. **About Pane Disclosure Text Break:**
   - In `crates/settings/ui/panes/about_pane.slint:382`, format the update disclosure text so that `update checks run entirely in-process against GitHub Releases, which you can switch off.` starts on a new line instead of wrapping awkwardly mid-phrase.

**Collateral (MUST be updated in this ticket, not reported as a regression):**
- `crates/settings/src/app.rs:2999` `about_pane_renders_in_process_disclosure` passes the whole disclosure string to `find_about_element` as an exact match. Inserting a line break makes that lookup fail. Update the expected string to the new form; do not delete the test.
- `scripts/verify-settings-runtime.ps1` walks expected focus stops. Re-run it after the layout change and update expected stops only if the change genuinely moved one.

**Blocked by:** None

**Status:** ready-for-agent

- [ ] In `general_pane.slint`, the text `(100–500 ms)` renders on a new line.
- [ ] In `general_pane.slint`, the Hold Delay Threshold row height/padding is increased with ample clearance from the card bottom border.
- [ ] In `about_pane.slint`, `update checks run entirely in-process...` starts cleanly on a new line.
- [ ] `about_pane_renders_in_process_disclosure` is updated to the new string and passes.
- [ ] `every_caption_in_the_general_pane_sets_a_wrap_mode` still passes.
- [ ] No horizontal scrollbar is introduced in either pane at default/minimum window width.
- [ ] `scripts/verify-settings-runtime.ps1` passes.
