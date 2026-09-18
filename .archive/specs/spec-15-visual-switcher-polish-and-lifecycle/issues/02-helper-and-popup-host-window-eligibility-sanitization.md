# 02: Helper and PopupHost window eligibility sanitization

**What to build:**
Sanitize window eligibility discovery so modern WinUI/XAML popup hosts stop appearing as switcher cards, and capture the facts the remaining DEC-026 Clause 3 filters will need once they are diagnosed.

DEC-026 Clause 3 *authorises* this sanitisation; its own **Owner decisions still open** section says it does **not** identify the subject, and gates SPEC-14-03 part 2 on `wdi-systematic-debugging`. Only the popup host has a reproduction. That split governs this ticket.

1. **Facts Collection (`crates/daemon/src/cycling/source.rs`, `WindowFacts` in `cycling/mod.rs:99-113`):**
   - Add `has_title: bool` (from `GetWindowTextLengthW`), `has_nonzero_extent: bool` (from the window rect), and `is_owned: bool` (from `GetWindow(hwnd, GW_OWNER)`).
   - `class_name` is already captured; no new capture is needed for the popup-host check.

2. **Eligibility — exclude now (`crates/daemon/src/cycling/eligibility.rs`):**
   - Add `HELPER_SURFACE_CLASSES = ["PopupHost", "Xaml_WindowedPopupClass"]` alongside `SHELL_SURFACE_CLASSES`, and `WindowFacts::is_helper_surface()` mirroring `is_shell_surface()`.
   - Add `ExclusionReason::HelperSurface` and exclude on it.
   - Place it in the frozen precedence list **immediately after `shell surface`** — both are class-name checks and neither depends on any other fact. Update the numbered precedence doc comment at the top of `eligibility.rs`; the order is contract, not commentary.

3. **Eligibility — capture only, do NOT exclude yet:**
   - `has_title`, `has_nonzero_extent`, and `is_owned` are recorded on `WindowFacts` and surfaced to fixtures, but `evaluate_facts` MUST NOT branch on them in this ticket. Each has a real false-positive cost: a window enumerated mid-creation has no title yet, a legitimately untitled canvas window is a real target, and an owned-but-real tool window is a target the user expects. Excluding them without a reproduction trades a visible ghost card for an invisible missing card.
   - Open the exclusion as a question via `wdi-question` (class `assumptions.md`) naming what evidence would close it.

**Collateral (MUST be handled in this ticket):**
- `crates/daemon/src/cycling/eligibility.rs:105` asserts `evaluate_facts` agrees with `ReferencePolicy` (`cycling/mod.rs:441`) on every fixture. `ReferencePolicy` MUST gain the same helper-surface rule in the same precedence position.
- `crates/daemon/src/cycling/mod.rs:607` asserts every class in `SHELL_SURFACE_CLASSES` (plus `CLASS_GHOST`) has a fixture in `expected_decisions()`. Extend that guard to cover `HELPER_SURFACE_CLASSES` too, and add the fixtures.

**Blocked by:** None

**Status:** closed

- [x] Top-level windows with class `PopupHost` or `Xaml_WindowedPopupClass` are excluded with `ExclusionReason::HelperSurface`.
- [x] The frozen exclusion-precedence doc comment in `eligibility.rs` names the new rule at its exact position.
- [x] `ReferencePolicy` carries the same rule; `agrees_with_reference_policy_on_every_fixture` passes.
- [x] The fixture-completeness guard covers `HELPER_SURFACE_CLASSES`, and a fixture exists for each class.
- [x] `WindowFacts` carries `has_title`, `has_nonzero_extent`, and `is_owned`, populated by `source.rs`.
- [x] `evaluate_facts` does **not** branch on those three fields; a test asserts a titleless / zero-extent / owned window is still `Eligible`, so the deferral is a guard rather than an omission.
- [x] Real application windows (Notepad, Terminal, browsers, and the synthetic hung-window fixture) remain fully eligible.
- [x] The exclusion parity between blind cycling and the visual switcher is preserved — both read the same `WindowEligibility`.
