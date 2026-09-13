# 03: Modern WinUI 3 popup bridge exclusion

**What to build:**
Exclude the Windows App SDK / WinUI 3 popup host surfaces that appear as ghost cards in the visual
switcher, most visibly under modern Windows 11 `Notepad.exe`.

## 1. Diagnosis (live WinProbe)

Every modern Notepad instance creates top-level helper surfaces with:

- `class_name: "Microsoft.UI.Content.PopupWindowSiteBridge"`
- `window_text: "Pop-upHost"`
- `is_owned: true` (owner is the main Notepad window)
- small extent, e.g. 38x47

SPEC-15-02 put `PopupHost` and `Xaml_WindowedPopupClass` into `HELPER_SURFACE_CLASSES`
(`cycling/mod.rs:48`), but not this class, so `is_helper_surface()` (`cycling/mod.rs:130`) returns
false and the surfaces stay eligible.

## 2. The change — one class, nothing else

In `crates/daemon/src/cycling/mod.rs`:

- Add `pub const CLASS_POPUP_WINDOW_SITE_BRIDGE: &str = "Microsoft.UI.Content.PopupWindowSiteBridge";`
  beside the existing class constants (`cycling/mod.rs:36-37`).
- Add it to `HELPER_SURFACE_CLASSES` (`cycling/mod.rs:48`).
- Add the matching fixture row to `expected_decisions()` — the existing rows for `CLASS_POPUP_HOST`
  and `CLASS_XAML_WINDOWED_POPUP` at `cycling/mod.rs:439-446` are the pattern; use
  `with_class(16, CLASS_POPUP_WINDOW_SITE_BRIDGE)` with
  `Eligibility::Excluded(ExclusionReason::HelperSurface)`.

**No other production code changes.** `is_helper_surface()` is already consulted by the production
policy (`eligibility.rs:49-51`) and by `ReferencePolicy` (`cycling/mod.rs:480-482`), so parity
between blind cycling and the visual switcher holds by construction rather than by a second edit.

## 3. Two clauses from the first draft are deliberately dropped

**"Exclude windows titled `Pop-upHost`" is not implementable and is not wanted.** `WindowFacts`
(`cycling/mod.rs:104-121`) carries `has_title: bool`, not a title string — there is nothing to
compare against. Expressing it means widening the `WindowFacts` contract, which DEC-026's closing
note explicitly reserves as the owner's call rather than an implementer's. It is also redundant (the
class is diagnosed and stable) and hazardous: a user's document literally named `Pop-upHost` would
disappear from the switcher.

**"Exclude owned helper surfaces where `is_owned` is true and dimensions indicate a popup" is a
reversal of a recorded deferral.** SPEC-15-02 captured `is_owned`, `has_title` and
`has_nonzero_extent` as facts and deliberately left them non-excluding, because DEC-026 clause 3 has
no diagnosed subject for them. Three tests assert that deferral on purpose:

- `cycling::eligibility::tests::empty_title_window_is_still_eligible` (`eligibility.rs:395`)
- `cycling::eligibility::tests::zero_extent_window_is_still_eligible` (`eligibility.rs:407`)
- `cycling::eligibility::tests::owned_window_is_still_eligible` (`eligibility.rs:419`)

Changing that is a decision against DEC-026's recorded state and needs a new `DEC-`, not a ticket.
All three tests stay green and unmodified. The class match fixes the reported defect completely
without touching them.

## 4. Tests

- `cycling::eligibility::tests::popup_window_site_bridge_is_excluded` — a window with class
  `Microsoft.UI.Content.PopupWindowSiteBridge` is `Excluded(HelperSurface)`.
- `cycling::eligibility::tests::winui_notepad_popup_bridge_profile_is_excluded` — the exact live
  profile (that class, `is_owned: true`, `has_nonzero_extent: true`, same application identity as the
  foreground) is `Excluded(HelperSurface)`. This is the regression guard: without the class in the
  list, the other three facts are individually eligible, which is exactly why the surface got through.
- `cycling::eligibility::tests::real_notepad_window_remains_eligible` — an ordinary Notepad document
  window is still `Eligible`.
- `cycling::tests::every_helper_surface_class_has_a_fixture` — already exists (`cycling/mod.rs:643`);
  it must go red before the fixture is added and green after. Break it once and watch it fail before
  claiming it.
- `cycling::eligibility::tests::agrees_with_reference_policy_on_every_fixture` — already exists and
  must stay green, proving blind-cycle / switcher parity over the new fixture.
- The three deferral tests above must still pass **unmodified**.

**Blocked by:** None

**Status:** ready-for-agent

- [ ] `CLASS_POPUP_WINDOW_SITE_BRIDGE` exists and is in `HELPER_SURFACE_CLASSES`.
- [ ] A window with that class is `Excluded(ExclusionReason::HelperSurface)` in both the production
      policy and `ReferencePolicy`.
- [ ] The live Notepad popup-bridge profile is excluded; a real Notepad document window is not.
- [ ] `every_helper_surface_class_has_a_fixture` was seen red before the fixture landed, and is green
      after.
- [ ] `agrees_with_reference_policy_on_every_fixture` is green.
- [ ] `empty_title_window_is_still_eligible`, `zero_extent_window_is_still_eligible` and
      `owned_window_is_still_eligible` pass unmodified.
- [ ] No title-based and no owner-based exclusion was added.
- [ ] Manual: holding the cycle chord over modern Notepad shows document windows only.
