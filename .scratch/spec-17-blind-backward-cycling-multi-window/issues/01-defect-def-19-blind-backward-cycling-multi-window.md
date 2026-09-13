# 01: Defect DEF-19 — Blind backward cycling oscillates between two windows

**What to build:** Fix `DEF-19` (`.control/registry/defects.yaml`). During blind cycling (rapid tap of the cycle chord), holding Shift with `Alt + ~` or `Win + ~` currently causes focus to ping-pong repeatedly between the top two windows (`A <-> B`) instead of cycling backward through all eligible same-application windows (`A -> D -> C -> B -> A`).

**Blocked by:** None (can start immediately)

**Status:** done

## Second-opinion review notes (2026-09-13)

- **Root cause is a provable invariant, not a probabilistic bug.** Windows' Z-order
  mutation on activation is remove-and-prepend: the activated window moves to the
  front, everyone else keeps relative order. That rule guarantees the just-deactivated
  foreground always lands at exactly `Z=1` after *any* activation — unconditionally,
  for any window count. Since `Direction::Backward` always picks `Z=1`, it is
  mathematically certain to re-select the prior foreground on the very next tap once
  `active` sits at `Z=0` (the normal post-activation state). This is invisible at n=2
  only because `Z=1` is the only other window there anyway.
- **This is not a two-way design choice — session tracking is required.** A
  fully stateless (position-only) reformulation that both (a) makes the first
  backward tap select the immediate neighbor and (b) guarantees full traversal
  on repeated taps does not exist: after each activation, only `Z=0` and `Z=1` are
  deterministic outcomes of the mutation; positions `Z≥2` preserve prior relative
  order but there is no fixed offset into that region that reproduces a coherent
  full backward sequence (hand-traced on 4 windows — the correct sequence needs
  index 1, then index 2 of the *mutated* order, then index 3: a moving target, not
  a fixed rule). The fix needs a cycling session (a tracked visitation
  history/cursor, analogous to the visual switcher's frozen candidate list +
  cursor) — not a smarter formula over the live snapshot alone.
- **This reverses DEC-026 §B-4, an owner-accepted and already-shipped decision.**
  `.control/decisions/DEC-026-cross-monitor-switcher-shift-cycle-and-window-eligibility.md`
  §B-4 explicitly prescribes *"the backward order is the same rotation without the
  final `reverse()`"* — exactly the code this ticket now proposes to change, accepted
  by the owner in session and already implemented under SPEC-16. Live testing has
  shown that accepted premise void. Per this repo's decision policy, that is the
  "planning assumption turns out to be void" case `wdi-decision` exists for — this
  should go through that skill (to record the void assumption and supersede/amend
  DEC-026 §B-4) rather than being silently patched as an ordinary bugfix. Not
  actioned here — flagged for the owner to route.
- **The proposed tests need to simulate a *mutating* Z-order, not a single static
  call.** `backward_cycle_order_is_the_forward_rotation_unreversed` in `mod.rs` and
  a single-call assertion on `cycle_order_directed` would pass against the current
  broken code too — the bug only appears across repeated live activations. The new
  tests must drive consecutive activations against a Z-order that reorders itself
  after each pick, the way `repeated_cycles_reach_every_window` already does for
  forward.

- [x] **Confirm root cause in dynamic Z-order activation**:
      In `crates/daemon/src/cycling/mod.rs:255`, `cycle_order_directed` for `Direction::Backward` omits `rotated.reverse()`, selecting the window immediately after the foreground in current Z-order (`Z=1`). When window `Z=1` is activated on the live desktop, Windows immediately raises it to the top of the Z-order (`Z=0`), pushing the previous foreground window to `Z=1`. Consequently, the next backward blind cycle command sees the original window at `Z=1` and activates it, locking navigation into a two-window oscillation.
- [x] Implement backward traversal for blind cycling via a cycling session (tracked
      visitation history/cursor while consecutive backward taps continue), not a
      stateless reformulation of `cycle_order_directed` — see review notes above for
      why a stateless fix cannot satisfy both first-tap-is-neighbor and full-traversal
      at once. Define the session's lifecycle explicitly: when it starts (first
      backward tap after a direction change or a fresh, non-cycling foreground),
      and when it resets (app switch outside cycling, a session window closing,
      a forward tap interleaved, or a timeout).
- [x] Add unit test `cycling::tests::backward_blind_cycle_traverses_all_eligible_windows` verifying that repeated backward cycles on a **dynamic, self-mutating** Z-order (candidates reordered after each pick, per the remove-and-prepend rule) with 3+ windows visit all windows in reverse sequence — a single static `cycle_order_directed` call is not sufficient to catch this regression.
- [x] Add worker test `worker::tests::repeated_backward_blind_cycles_visit_every_window_in_reverse` ensuring simulated consecutive `CyclePrev` commands against a dynamic Z-order do not oscillate between two windows.
- [x] Verify visual switcher backward entry (`Direction::Backward` / `Command::SwitcherArmPrev` / `Command::SwitcherPrev`) remains intact and aligned with card selection.
- [x] Full test suite green once across workspace (`WIRADESK_SKIP_MANIFEST=1 cargo test --workspace`).
