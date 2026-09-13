---
spec: SPEC-15
release: "0.4.0"
prd: wira-desk
fr: []
status: closed
---

# SPEC-15 — Visual Switcher Active Window Parity, Helper Sanitization, Single-Window Hold, Timing Decoupling, and Settings Polish

## Problem Statement

Following manual testing of SPEC-14 on Windows 11 desktop, four user-visible behavioral defects, timing questions, and visual polish requests were reported by the product owner:

1. **Settings UI Layout & Text Polish:**
   - **General Pane:** The hold delay description caption currently keeps `(100–500 ms)` inline, and the container row sits uncomfortably close against the bottom card border without adequate breathing room.
   - **About Pane:** The disclosure caption `update checks run entirely in-process against GitHub Releases, which you can switch off.` in Card 3 currently hangs awkwardly without an intentional newline break.

2. **Visual Switcher Omits the Primary Active Window:**
   - When 4 Notepad windows are open across desktops/layers, invoking the visual switcher displays only 3 window preview cards. The current active/foreground window is omitted.
   - **Root Cause (Verified):** In `crates/daemon/src/switcher/mod.rs:175-180`, `card_order_for_candidates` calls `crate::cycling::cycle_order`, which rotates the candidate list starting from `pos + 1` and explicitly omits `active.foreground`. While omitting the active window is correct for *blind cycling* (which only activates the *next* window), a visual switcher overlay must present *all* eligible same-application windows as interactive preview cards, positioning the initial selection highlight on the next window (or last window for backward entry).

3. **Stray Helper and PopupHost Windows Polluting Overlay:**
   - Windows 11 WinUI/XAML apps (such as modern `Notepad.exe` and `WindowsTerminal.exe`) spawn top-level helper surfaces with window class names such as `PopupHost` or `Xaml_WindowedPopupClass`, or zero-sized/empty-titled helper surfaces. Because these surfaces share the process binary path of the application, `WindowEligibility::evaluate_facts` currently marks them eligible, polluting the switcher with ghost cards.
   - DEC-026 Clause 3 **authorises** this sanitisation but, by its own *Owner decisions still open* note, does **not** identify what is being sanitised: `WindowFacts` (`crates/daemon/src/cycling/mod.rs:99-113`) carries neither a rect nor a title, so all three named filters are changes to the **captured-facts contract**, not filter rules. SPEC-14-03 part 2 was severed and gated on `wdi-systematic-debugging`. Only `PopupHost` / `Xaml_WindowedPopupClass` now has a reproduction (owner's SPEC-14 test pass: *"masih muncul pop-uphost banyak sekali"*); empty-title, zero-extent, and owned-window remain **undiagnosed** — see Sequencing.

4. **Visual Switcher Fails to Trigger on Single-Window Desktops:**
   - When only 1 Notepad window exists on the active virtual desktop/monitor while 3 exist on another layer/desktop, pressing and holding the cycle hotkey fails to trigger the visual switcher overlay.
   - **Root Cause (Verified):** In `crates/daemon/src/worker.rs:519-539`, `decide_switcher_hold_delay` only returns `Some(delay)` if `outcome` is `CycleOutcome::Activated(_)`. On a desktop with only a single window, blind cycle finds no *other* candidate to switch to, returning `CycleOutcome::NoEligibleTarget` (or exhausted). Consequently, `TIMER_SWITCHER_HOLD` is never armed.

5. **Cycle Activation Colliding with Hold Delay (Timing Decoupling):**
   - Pressing and holding the cycle chord currently triggers an immediate blind cycle switch on the initial keydown event *before* the hold delay threshold expires. The user questions whether blind cycle activation should wait for rapid key release below the hold threshold so that the two actions do not collide.
   - **Root Cause (Verified):** `crates/daemon/src/hook.rs:625-644` enqueues `Command::Cycle` on keydown and arms `rt.switcher_deadline_ms` in the same breath. Both actions are therefore already in flight before the threshold is anywhere near expiry.

---

## Solution

1. **Settings Pane Polish (SPEC-15-01):**
   - In `general_pane.slint`, place `(100–500 ms)` on a dedicated newline and increase the row's vertical padding / height (`padding-bottom: 16px`, min-height affordance) so it does not feel cramped against the bottom border.
   - In `about_pane.slint`, format the update disclosure text with an explicit newline break before `update checks run entirely in-process...`.

2. **Window Eligibility Sanitization (SPEC-15-02):**
   - Extend `WindowFacts` and `WindowEligibility` to filter out non-application top-level surfaces. **The capture and the exclusion are separated by evidence** — see Implementation Decisions:
     - **Excluded now (reproduced):** known XAML/WinUI popup host surfaces (`PopupHost`, `Xaml_WindowedPopupClass`).
     - **Captured now, excluded only once diagnosed:** empty titles (`GetWindowTextLengthW == 0`), zero width or zero height, windows owned by another window (`GetWindow(hwnd, GW_OWNER) != 0`).

3. **Active Window Inclusion & Single-Window Switcher Triggering (SPEC-15-03):**
   - Update `card_order_for_candidates` to preserve the active foreground window in the returned card list, at index 0, followed by the others in LRU order.
   - Update `open_visual_switcher`'s `initial_index` so forward entry selects index `1` (or `0` when `len == 1`) and backward entry selects `len - 1`.
   - Update `decide_switcher_hold_delay` so that holding chord modifiers arms the visual switcher hold timer even when blind cycle did not activate a new window (e.g. single window present), allowing the visual switcher overlay to open and display available windows.

4. **Blind Cycle Timing & Hold Threshold Decoupling (SPEC-15-04):**
   - Decouple blind cycle activation from raw keydown:
     - If the chord is released rapidly before the hold delay threshold expires (tap release): execute blind cycle activation.
     - If the chord is held past the hold delay threshold: open the visual switcher overlay without executing a premature blind cycle prior to overlay opening.
   - Formalize the architectural timing change via a dedicated decision record (`DEC-029`).

---

## User Stories

1. **As a user configuring Wira Desk**, I want the General pane hold delay description and About pane disclosure formatted cleanly across lines with comfortable breathing room, so that the settings window looks visually balanced and professional.
2. **As a keyboard multitasker**, I want the visual switcher overlay to include my currently active window alongside background windows, so that I have a complete view of all open windows of the application.
3. **As a Windows 11 user**, I want helper surfaces like `PopupHost` and empty dialog tools excluded from the visual switcher, so that only real, navigable application windows appear as cards.
4. **As a user on a desktop with only one window of an app**, I want to be able to trigger the visual switcher overlay by holding the shortcut, so that I can see the window or access multi-monitor instances without getting blocked.
5. **As a power user holding the cycle chord**, I want Wira Desk to wait for the hold delay before acting rather than prematurely switching my window before the overlay opens, so that quick taps cycle and deliberate holds open the switcher cleanly without two colliding actions.

---

## Implementation Decisions

- **The timing change is sequenced FIRST, not last.** `SPEC-15-04` MUST land before `SPEC-15-03`. Reason, verified in code: today
  `Command::Cycle` fires at keydown, so by the time `open_visual_switcher` runs (`crates/daemon/src/worker.rs:248`) its
  `capture_active_context()` already returns the window the blind cycle *jumped to*. Adding the active window to the card list while that
  is still true seats the just-jumped-to window at index 0 and the forward highlight at index 1 — two windows away from where the user
  started. The dependency edge in the drafts (`SPEC-15-04` blocked by `SPEC-15-03`) is therefore **backwards** and is reversed here.

- **Option A is selected for the timing contract, with three guard rails.** Blind cycle activation moves from chord keydown to the
  chord's *committing edge* (main-key release below `visual_hold_delay_ms`). Guard rails, all of which `DEC-029` MUST carry:
  1. **Option A is only armed when `visual_enabled` is true.** With the overlay disabled there is no second action to collide with, so
     the keydown path stays exactly as it is and the current latency profile is untouched for those users.
  2. **SM-1's measurement point changes and MUST be restated.** The PRD (`.what/_prd/wira-desk/prd.md:298`) promises focus transfer
     *"under 1 ms following keypress"*. Under Option A the transfer is under 1 ms following the *release*. `DEC-029` MUST restate SM-1's
     measurement point as the chord's committing edge, or the promise MUST be amended at G2. Shipping Option A while SM-1 reads
     "keypress" leaves the PRD asserting something the code no longer does.
  3. **Auto-repeat MUST be suppressed on the chord main key.** The hook has no repeat guard (`crates/daemon/src/hook.rs` carries no
     `LLKHF_` repeat test and no already-down latch), and the switcher navigation path (`crates/daemon/src/hook.rs:514-540`) enqueues
     without passing the `ANTI_MACRO_THROTTLE_MS` gate at line 613. Holding the chord past the Windows repeat delay therefore emits
     repeated `WM_KEYDOWN` of `switcher_main_vk`, which the open overlay reads as `SwitcherNext` at the repeat rate. Option A makes
     holding the normal path, so this MUST be closed in the same ticket rather than discovered after.

- **Two timers govern one threshold, and they MUST NOT drift apart.** `rt.switcher_deadline_ms` in the hook
  (`crates/daemon/src/hook.rs:641`) and `SetTimer(TIMER_SWITCHER_HOLD, delay)` in the worker (`crates/daemon/src/worker.rs:593`) each
  derive the same value with their own clamp and their own `150` fallback. `SPEC-15-04` MUST make the release-vs-hold verdict read from
  **one** of them; duplicating the decision is how the two actions collided in the first place.

- **Active window indexing lives in two files, not one.** `card_order_for_candidates` returns the ordering; the selection index is
  computed in `open_visual_switcher` (`crates/daemon/src/worker.rs:322-326`), which today hardcodes `0` for forward. `SPEC-15-03` MUST
  change both sites — a `Vec<WindowId>` cannot express a highlight on its own.

- **Helper filtering is split by evidence.** `PopupHost` / `Xaml_WindowedPopupClass` has a reproduction and ships as an exclusion in
  `SPEC-15-02`. Empty-title, zero-extent, and `GW_OWNER` do not, and each carries a real false-positive cost (a window enumerated
  mid-creation, a legitimately untitled canvas, an owned-but-real tool window). They land in `SPEC-15-02` as **facts captured and
  recorded**, with the exclusion held behind the `wdi-systematic-debugging` diagnosis DEC-026 already demanded.

- **Eligibility has a reference twin and a fixture completeness guard.** `evaluate_facts` is asserted equal to `ReferencePolicy`
  (`crates/daemon/src/cycling/eligibility.rs:105`) over `expected_decisions()`, and `crates/daemon/src/cycling/mod.rs:607` asserts every
  class in `SHELL_SURFACE_CLASSES` has a fixture. Any new `ExclusionReason` MUST be added to the frozen precedence list in the
  `eligibility.rs` doc comment, to `ReferencePolicy`, and to `expected_decisions()`, or the suite goes red on arrival.

---

## Sequencing

| Ticket | Blocked by | Why |
|---|---|---|
| `SPEC-15-01` | — | Settings-only; touches no daemon code. |
| `SPEC-15-02` | — | Facts contract and the `PopupHost` exclusion. |
| `SPEC-15-04` | — | **Reversed from the draft.** The timing contract must be true before card identity means anything. |
| `SPEC-15-03` | `SPEC-15-02`, `SPEC-15-04` | Needs a clean candidate set and a truthful `active.foreground`. |

## Collateral — tests that go red on arrival

These already exist and MUST be updated by the ticket that breaks them. They are named here so they are not mistaken for regressions:

| Test | Broken by | Why |
|---|---|---|
| `settings::app::tests::about_pane_renders_in_process_disclosure` (`crates/settings/src/app.rs:2999`) | `SPEC-15-01` | Matches the disclosure string exactly via `find_about_element`; inserting a line break fails the lookup. |
| `daemon::switcher::tests::card_order_equals_cycle_order` (`crates/daemon/src/switcher/mod.rs:253`) | `SPEC-15-03` | Asserts the identity `SPEC-15-03` exists to break. It MUST be replaced by a test naming the new divergence, not deleted. |
| `daemon::switcher::tests::switcher_candidate_set_diverges_from_blind_cycle_on_monitor_boundary` (`crates/daemon/src/switcher/mod.rs:192`) | `SPEC-15-03` | Compares switcher and blind-cycle sets; the expected switcher set gains the active window. |
| `daemon::worker::tests` hold-delay cases (`crates/daemon/src/worker.rs:1394-1430`) | `SPEC-15-03` | `decide_switcher_hold_delay` gains non-`Activated` arming paths; the `_ => None` arm is the thing being changed. |
| `scripts/verify-settings-runtime.ps1` | `SPEC-15-01` | Walks expected stops; a re-laid-out hold-delay row may move one. |

## Out of scope

- **Cross-virtual-desktop candidates.** DEC-026 Clause 1 fixes switcher scope at `AnyMonitorOnCurrentDesktop`, and the owner's
  1-window/3-window report is a *layer* (virtual desktop) split. `SPEC-15-03` makes the single-window desktop **open its overlay**; it
  does **not** bridge desktops. Widening that boundary is a new `DEC-`, not a ticket.
- **An FR or capability for the visual switcher.** `fr: []` and every empty `satisfies:` here inherit SPEC-14's state, which DEC-026
  records as owner authoring work at G2. This spec MUST NOT invent one.
