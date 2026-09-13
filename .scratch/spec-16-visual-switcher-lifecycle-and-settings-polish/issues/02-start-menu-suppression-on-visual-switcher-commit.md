# 02: Start Menu suppression while the visual switcher chord is held

**What to build:**
Stop the Windows Start Menu from opening when the visual switcher is committed by releasing `Win`,
and remove the focus race that follows it.

## 1. What the code actually does today

- The hook swallows the main key on keydown (`hook.rs:666-670`) and on keyup (`hook.rs:492`), so the
  shell sees `Win` down followed by `Win` up with nothing between, and reads it as a lone Windows-key
  tap. That is the whole defect.
- `suppress_start_menu()` (`worker.rs:42`) breaks that pattern by injecting one `VK_NONAME` while
  `Win` is physically down, making the press a combination.
- Before SPEC-15-04 this ran as part of the keydown cycle. Decoupling moved the cycle to the release
  edge, so nothing injects during the hold.

**Correction to the first draft of this ticket:** `suppress_start_menu()` was **not** removed from
the commit path. `worker.rs:443` still calls it inside `Command::SwitcherCommit`. It is a no-op there,
and always has been: the commit is enqueued *because* `Win` came up — from the hook's modifier-keyup
branch (`hook.rs:448-458`) or from the watchdog's `!are_any_modifiers_down()` (`worker.rs:233`) — so
by the time the Worker drains it, the function's own guard (`worker.rs:66-70`) sees `Win` up and
returns immediately.

**Suppression on the release edge cannot work.** Anything phrased as "when the modifier release
commits the switcher, ensure Start Menu cannot trigger" is asking for an injection after the only
moment it could have had an effect.

## 2. Where the call goes

Two sites, both on the Worker, both at points where `Win` is still physically down:

- **`execute_switcher`, `Command::SwitcherArm | Command::SwitcherArmPrev` (`worker.rs:357`)** — call
  `suppress_start_menu()` as the **first** statement of the arm, *before* the
  `decide_switcher_hold_delay` gate and before the `if let Some(delay)`. Arming happens on the main
  key's keydown, so `Win` is held.
- **`open_visual_switcher` (`worker.rs:249`)** — call it as the **first** statement of the function,
  *before* the `eligible.is_empty()` return at `worker.rs:267` and the `eligible_ordered.is_empty()`
  return at `worker.rs:277`. Placed after either one, the no-candidate path still opens Start: the
  overlay never opens, `switcher_active` stays false, and the swallowed main-key release past the
  deadline enqueues nothing (`hook.rs:474-482`), so the user gets no overlay, no switch, and a Start
  Menu. That is precisely the owner's "pas tidak bisa itulah dia muncul start menu".

One injection anywhere inside a single `Win` press is enough — the shell's lone-Win test asks only
whether *any* other key was pressed during that press. Arming therefore covers commit, cancel
(`Command::SwitcherCancel`, `worker.rs:445`), disarm (`Command::SwitcherDisarm`, `worker.rs:385`,
which suppresses nothing today), and the no-eligible-candidate path with a single call.

## 3. Remove the dead call

Delete `suppress_start_menu()` at `worker.rs:443` and leave a comment in its place recording that the
release edge is too late, so a future reader does not reinstate it as a "fix".

## 4. Do not touch `hook.rs`

`worker.rs:31-41` states this as a SAFETY precondition earned from a live regression: calling
`SendInput` from inside the low-level keyboard hook re-enters input processing, races the activation,
and stopped cycling from moving focus at all. **This ticket adds no `SendInput`, and no call that
reaches one, to `crates/daemon/src/hook.rs`.** The only change permitted there is a test.

## 5. Tests

- `worker::tests::switcher_arm_injects_start_menu_suppression` — arming enqueues/performs the
  suppression before the hold-delay gate is consulted.
- `worker::tests::open_visual_switcher_suppresses_before_the_no_candidate_return` — suppression
  happens even when `collect_eligible_candidates` yields nothing. Pure-logic coverage: extract the
  ordering into a testable seam rather than asserting against `SendInput`.
- `worker::tests::switcher_commit_no_longer_suppresses` — the dead call is gone and the commit path
  is unchanged otherwise.
- `hook::tests::releasing_win_key_commits_switcher_without_enqueueing_anything_new` — the hook's
  modifier-keyup commit path is byte-for-byte what it was; this ticket did not change it.

## 6. Manual verification is part of this ticket, and covers both symptoms

The owner reported two things together: the Start Menu opening, **and** the switch itself failing
intermittently ("kadang bisa kadang tidak"). The focus race with `StartMenuExperienceHost` is the
working hypothesis for both, but it is inferred, not measured. Verify them separately:

1. Hold `Win + ~` past the threshold, release `Win`. Start Menu must not open.
2. Repeat ten times. The highlighted window must receive focus every time.

If (1) is clean but (2) still fails, the intermittency has a second cause and goes to
`wdi-systematic-debugging` as its own diagnosis — not back into this ticket.

**Blocked by:** None

**Status:** ready-for-agent

- [ ] `suppress_start_menu()` is the first statement of the `SwitcherArm`/`SwitcherArmPrev` arm,
      before the hold-delay gate.
- [ ] `suppress_start_menu()` is the first statement of `open_visual_switcher`, before both early
      returns.
- [ ] The dead call at `worker.rs:443` is removed and replaced by a comment saying why.
- [ ] `crates/daemon/src/hook.rs` gained no `SendInput` and no call reaching one.
- [ ] Manual: releasing `Win` to commit does not open the Start Menu.
- [ ] Manual: ten consecutive commits all move focus to the highlighted window.
- [ ] Manual: `Escape` cancel and `Alt + ~` are unchanged.
- [ ] Unit tests above pass; the SPEC-15-04 hook tests still pass unchanged.
