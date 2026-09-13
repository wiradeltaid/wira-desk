# Smoke Test Execution — Mandate DEC-030

**Date:** 2026-09-13
**Mandate:** DEC-030
**Scope:** SPEC-16 (Visual Switcher Start Menu Suppression, WinUI Helper Exclusion, UI Em-Dash Removal, and Stepper Spacing Polish)
**Executor:** Agent (automated test suite & production runtime verification)

---

### Specifications Delivered

#### 1. SPEC-16-01: Settings stepper centering and em-dash removal
- **Verdict:** PASS
- **Evidence:**
  - `app::tests::general_pane_stepper_is_vertically_centered`
  - `app::tests::all_visible_slint_strings_have_zero_em_dashes`
  - `app::tests::about_pane_renders_in_process_disclosure_with_newline`
  - `app::tests::visual_hold_delay_description_wraps_with_newline`
  - `scripts/verify-settings-runtime.ps1` (10/10 PASS: automation tree, light/dark themes, declared stops, named elements)

#### 2. SPEC-16-02: Start Menu suppression while the visual switcher chord is held
- **Verdict:** PASS
- **Evidence:**
  - `worker::tests::switcher_arm_injects_start_menu_suppression`
  - `worker::tests::open_visual_switcher_suppresses_before_the_no_candidate_return`
  - `worker::tests::switcher_commit_no_longer_suppresses`
  - `hook::tests::releasing_win_key_commits_switcher_without_enqueueing_anything_new`

#### 3. SPEC-16-03: Modern WinUI 3 popup bridge exclusion
- **Verdict:** PASS
- **Evidence:**
  - `cycling::eligibility::tests::popup_window_site_bridge_is_excluded`
  - `cycling::eligibility::tests::winui_notepad_popup_bridge_profile_is_excluded`
  - `cycling::eligibility::tests::real_notepad_window_remains_eligible`
  - `cycling::tests::every_helper_surface_class_has_a_fixture`
  - `cycling::eligibility::tests::agrees_with_reference_policy_on_every_fixture`
  - Three deferral tests pass unmodified: `empty_title_window_is_still_eligible`, `zero_extent_window_is_still_eligible`, `owned_window_is_still_eligible`

---

### Verification Summary
- **Workspace Test Suite:** 439 daemon unit tests + 203 settings unit tests passed; 2 ignored (interactive desktop SendInput tests excluded from routine test runs per owner instruction); full suite GREEN.
- **Compiler Hygiene:** `cargo fmt --all -- --check` clean; `cargo clippy --workspace --all-targets -- -D warnings` clean.
- **Method Validation:** `validate.py --generate` passes GREEN with zero findings.
