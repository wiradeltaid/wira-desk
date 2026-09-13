# 04: Cycle activation timing and hold threshold decoupling

**What to build:**
Resolve the collision between blind cycle activation on chord keydown and the visual switcher hold threshold.

**The collision, verified:** `crates/daemon/src/hook.rs:625-644` enqueues `Command::Cycle` *and* arms `rt.switcher_deadline_ms` in the same keydown block. `crates/daemon/src/worker.rs:575-601` then arms `TIMER_SWITCHER_HOLD` from the cycle's own outcome. So a hold performs the jump at t=0 and opens the overlay at t=threshold — two actions from one gesture, which is exactly what the owner asked about.

## 1. Timing contract — Option A, with three guard rails

**Option A (selected):** blind cycle activation moves from the chord's keydown to its *committing edge* — the main key released below `visual_hold_delay_ms`. Held past the threshold, the overlay opens and no blind cycle has run.

Option B (immediate activation plus overlay-tracked reversion) is rejected: it keeps the visible double-jump the owner reported and only hides it behind a revert, which is a second focus change and a second chance to fight the shell.

`DEC-029` MUST carry all three guard rails:

1. **Armed only when `visual_enabled` is true.** With the overlay off there is no second action to collide with, so the keydown path stays exactly as it is and that user's latency profile is untouched. This also keeps `disabled_visual_switcher_never_arms_the_hold_timer` meaningful.
2. **SM-1's measurement point changes and MUST be restated.** `.what/_prd/wira-desk/prd.md:298` promises focus transfer *"under 1 ms following keypress"*. Under Option A the transfer is under 1 ms following the *release*. `DEC-029` MUST restate SM-1's measurement point as the chord's committing edge, or the promise is amended at G2 first. Shipping while SM-1 reads "keypress" leaves the PRD asserting something the code no longer does. This is the one mandatory-recording case: it contradicts a shipped promise.
3. **Auto-repeat MUST be suppressed on the chord main key.** The hook has no repeat guard — no `LLKHF_` repeat test, no already-down latch — and the switcher navigation path (`hook.rs:514-540`) enqueues *before* the `ANTI_MACRO_THROTTLE_MS` gate at `hook.rs:613`, so it is unthrottled. Holding the chord past the Windows repeat delay emits repeated `WM_KEYDOWN` of `switcher_main_vk`, which an open overlay reads as `SwitcherNext` at the repeat rate (~31/s at default settings). Option A makes holding the normal path, so this must be closed here. Latch the main key down on first keydown and ignore further keydowns of that VK until its keyup.

## 2. One threshold, one owner

`rt.switcher_deadline_ms` (`hook.rs:641`) and `SetTimer(TIMER_SWITCHER_HOLD, delay)` (`worker.rs:593`) each derive the same threshold with their own clamp and their own `150` fallback. The release-vs-hold verdict MUST be read from one of them. Duplicating the decision across two clocks is how the two actions came to overlap in the first place; leaving both in place under Option A makes a tap near the boundary resolvable as *both* a cycle and an overlay open.

## 3. Implementation notes

- The keyup branch at `hook.rs:471-484` already handles `rt.switcher_armed && vk == rt.switcher_main_vk && now < rt.switcher_deadline_ms` by emitting `SwitcherDisarm`. That is the exact seam where `Command::Cycle` now belongs.
- `set_last_cycle_mods(rt.mods)` (`hook.rs:626`) is read by the worker to arm the hold timer. Under Option A the modifier state at release is what matters; confirm it is still captured before the disarm path clears it.
- `Command::Cycle` / `Command::CyclePrev` direction still travels as its own command byte (DEC-026 §B-4). Deferral MUST NOT move direction into the modifier side-channel.
- The bypass-latch path (`hook.rs:430-442`) passes a latched chord through untouched. Confirm deferral does not swallow a main-key release that the latch is meant to pass.

**Blocked by:** None

> The draft placed this after `SPEC-15-03`. That edge is reversed: `SPEC-15-03` depends on **this** ticket, because the active window's identity at overlay-open time is only truthful once the premature cycle is gone.

**Status:** ready-for-agent

- [ ] `DEC-029` is authored and accepted, naming the timing contract, the `visual_enabled` gate, the SM-1 restatement, and the auto-repeat rule.
- [ ] SM-1 in `.what/_prd/wira-desk/prd.md` reads consistently with the shipped behaviour (restated, or the decision records why it stands).
- [ ] With `visual_enabled == false`, `Command::Cycle` still fires on keydown and no behaviour changes.
- [ ] With `visual_enabled == true`, a chord release below the threshold fires exactly one `Command::Cycle`, and none was enqueued at keydown.
- [ ] Holding past the threshold opens the overlay with **zero** `Command::Cycle` enqueued beforehand; the pre-hold foreground window is still foreground when the overlay appears.
- [ ] Repeated taps below the threshold each produce one cycle, subject only to `ANTI_MACRO_THROTTLE_MS`, with no added lag beyond the user's own key release.
- [ ] Holding the chord with the overlay open does **not** advance the selection: a repeated `WM_KEYDOWN` of `switcher_main_vk` while that key is already down is ignored. This test MUST be seen failing before the latch is added.
- [ ] The release-vs-hold verdict is derived in one place; a test asserts the hook and worker thresholds cannot disagree.
- [ ] `cargo test --workspace` is green with `--no-fail-fast`; hook timing, disarm, and modifier-release tests all pass.
