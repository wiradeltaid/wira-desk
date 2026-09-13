# 03: Active window card inclusion and single-window switcher triggering

**What to build:**
Make the visual switcher show every same-app window on the current desktop — including the active one — and open reliably when that desktop holds only one.

1. **Active Window Card Inclusion (`crates/daemon/src/switcher/mod.rs:175-180`):**
   - `card_order_for_candidates` stops delegating to `cycling::cycle_order` and returns `active.foreground` at index 0, followed by the remaining eligible windows in the same LRU order `cycle_order` produces.
   - The delegation is the defect: `cycle_order` rotates from `pos + 1` and drops the active window, which is right for blind cycling (it activates only the *next* window) and wrong for an overlay that is meant to be a picker.

2. **Initial Selection (`crates/daemon/src/worker.rs:322-326`) — a second site:**
   - `initial_index` is computed in `open_visual_switcher`, not in `card_order_for_candidates`. Today it hardcodes `0` for forward and `len - 1` for backward.
   - Forward entry MUST select index `1`, or `0` when `len == 1`. Backward entry keeps `len - 1`, which is now the last *non-active* card — re-assert it against DEC-026 §B-6 ("backward entry opens on the last card, and the blind hop before it must agree").

3. **Single-Window Switcher Triggering (`crates/daemon/src/worker.rs:519-539`):**
   - `decide_switcher_hold_delay` returns `None` for every outcome except `CycleOutcome::Activated(_)`. On a one-window desktop the blind cycle returns `NoEligibleTarget` or `Exhausted`, so `TIMER_SWITCHER_HOLD` is never armed.
   - Arm the timer for `NoEligibleTarget` and `Exhausted` too, provided `visual_enabled` is true and `mods.any()`. `visual_enabled == false` MUST still return `None` for every outcome.
   - `open_visual_switcher` already returns early on `eligible.is_empty()`. With the active window now in the list, a single-window desktop yields exactly one card and MUST open rather than early-exit.

4. **Virtual-desktop scope — assert, do not widen:**
   - DEC-026 Clause 1 fixes switcher scope at `SpatialScope::AnyMonitorOnCurrentDesktop`. The owner's 1-window/3-window report is a *virtual desktop* split, and bridging desktops is out of scope for this spec (a new `DEC-` owns that).
   - Add a test asserting a window on another virtual desktop stays out of the card set, so the boundary is proven rather than assumed.

**Collateral (MUST be handled in this ticket):**
- `switcher::tests::card_order_equals_cycle_order` (`switcher/mod.rs:253`) asserts exactly the identity this ticket breaks. Replace it with a test that names the new divergence — do not delete it.
- `switcher::tests::switcher_candidate_set_diverges_from_blind_cycle_on_monitor_boundary` (`switcher/mod.rs:192`): the expected switcher set gains the active window.
- `worker::tests` hold-delay cases (`worker.rs:1394-1430`) cover the `_ => None` arm being changed.

**Blocked by:** [SPEC-15-02, SPEC-15-04]

> `SPEC-15-04` first, and the draft's edge was the other way round. Until `SPEC-15-04` lands, `Command::Cycle` has already fired by the time `open_visual_switcher` calls `capture_active_context()`, so "the active window" is the window the blind cycle *jumped to*. Seating that at index 0 and highlighting index 1 lands the user two windows from where they started — a worse defect than the one being fixed.

**Status:** blocked

- [ ] When 4 same-app windows exist on the current desktop, `card_order_for_candidates` returns 4 window IDs, with `active.foreground` at index 0.
- [ ] Forward entry with `len > 1` highlights index 1; with `len == 1` it highlights index 0.
- [ ] Backward entry highlights `len - 1`, and that card is not the active window when `len > 1`.
- [ ] On a desktop with only 1 window of the active app, holding the cycle chord arms `TIMER_SWITCHER_HOLD` and opens the overlay with one card.
- [ ] `decide_switcher_hold_delay` still returns `None` for every outcome when `visual_enabled` is false, and when no modifier is down.
- [ ] A window on another virtual desktop is absent from the card set.
- [ ] **Escape** cancels and leaves the original active window focused. Releasing the modifiers **commits** the highlighted card — with forward entry and no navigation that is index 1, matching the blind cycle, not the origin. (The draft's criterion claiming release leaves the origin focused contradicts the commit-on-modifier-release path at `hook.rs:445-455` and is corrected here.)
- [ ] The two collateral `switcher::tests` above are updated and green.
