# Smoke Test Execution — Mandate DEC-028

**Date:** 2026-09-13
**Mandate:** DEC-028
**Scope:** SPEC-14 (Visual Switcher Refinements, Cross-Monitor Candidates, Shift-Cycle Parity, and Settings Polish)
**Executor:** Agent (automated test suite & production binary verification)

---

### Specifications Delivered

#### 1. SPEC-14-01: About pane Card 3 inline publisher link and full-bleed dividers
- **Verdict:** PASS
- **Evidence:** `app::tests::about_pane_renders_inline_publisher_link_with_open_icon`, `app::tests::about_pane_clicking_inline_publisher_opens_url`, `app::tests::about_pane_standalone_publisher_row_is_gone`, `app::tests::about_pane_card_dividers_span_the_card`, `app::tests::about_pane_attribution_line_wraps_without_horizontal_overflow`, `app::tests::about_pane_focus_order_survives_the_removed_publisher_row`.

#### 2. SPEC-14-02: General pane hold-delay caption wraps
- **Verdict:** PASS
- **Evidence:** `app::tests::visual_hold_delay_description_wraps_without_horizontal_scroll`, `app::tests::every_caption_in_the_general_pane_sets_a_wrap_mode`.

#### 3. SPEC-14-03: Visual switcher cross-monitor candidates
- **Verdict:** PASS (Part 1 delivered; Part 2 helper window diagnosis severable to future spec)
- **Evidence:** `context::tests::same_monitor_scope_reproduces_every_existing_spatial_decision`, `context::tests::any_monitor_scope_excludes_nothing_when_the_origin_monitor_is_unavailable`, `worker::tests::visual_switcher_collects_candidates_across_all_physical_monitors`, `worker::tests::visual_switcher_still_excludes_other_virtual_desktops`, `worker::tests::blind_cycle_stays_locked_to_the_active_monitor`, `worker::tests::a_cross_monitor_commit_activates_in_place_and_moves_no_window`, `worker::tests::the_blind_cycle_after_a_cross_monitor_commit_locks_to_the_new_monitor`, `switcher::tests::switcher_candidate_set_diverges_from_blind_cycle_on_monitor_boundary`, `switcher::thumbnail::tests::every_registration_is_unregistered_on_dismissal`, `switcher::thumbnail::tests::a_failed_registration_degrades_one_card_not_the_overlay`.

#### 4. SPEC-14-04: Visual switcher Shift-held backward cycling
- **Verdict:** PASS
- **Evidence:** `hook::tests::the_exact_pass_resolves_before_any_shift_relaxed_pass`, `hook::tests::a_shift_relaxed_cycle_never_shadows_the_default_stack_chord`, `hook::tests::a_shift_variant_never_shadows_another_configured_chord`, `hook::tests::cycle_chord_with_shift_held_enters_the_switcher_backward`, `hook::tests::shift_is_cleared_from_the_switcher_chord_modifiers`, `hook::tests::releasing_one_chord_modifier_while_shift_is_held_commits_the_overlay`, `worker::tests::releasing_shift_during_the_hold_window_still_opens_the_overlay`, `worker::tests::a_backward_entry_opens_on_the_last_card`, `worker::tests::queued_cycle_and_cycle_prev_each_keep_their_own_direction`, `cycling::tests::backward_cycle_order_is_the_forward_rotation_unreversed`, `commands::tests::cycle_prev_is_throttled_exactly_as_cycle_is`, `persistence::tests::a_cycle_chord_containing_shift_is_refused_at_save`, `persistence::tests::a_cycle_chord_whose_shift_variant_is_reserved_is_refused`, `config::tests::a_legacy_shift_bearing_cycle_chord_reloads_without_rejecting_the_config`, `config::tests::a_cycle_chord_whose_shift_variant_is_reserved_is_refused_on_reload`.

#### 5. SPEC-14-05: Worker honours visual_enabled and visual_hold_delay_ms
- **Verdict:** PASS
- **Evidence:** `config::tests::worker_snapshot_carries_the_visual_switcher_settings`, `worker::tests::a_cold_start_snapshot_carries_the_on_disk_switcher_settings`, `worker::tests::disabled_visual_switcher_never_arms_the_hold_timer`, `worker::tests::enabled_visual_switcher_arms_with_the_configured_hold_delay`, `worker::tests::an_out_of_range_hold_delay_is_clamped_before_it_reaches_settimer`, `worker::tests::a_reload_between_arming_and_firing_does_not_open_the_overlay`, `worker::tests::changing_hold_delay_while_timer_armed_keeps_in_flight_delay`.

#### 6. SPEC-14-06: Aspect-ratio tiles and height-bounded row packing
- **Verdict:** PASS
- **Evidence:** `switcher::layout::tests::tile_width_follows_the_source_window_aspect_ratio`, `switcher::layout::tests::every_card_on_a_page_shares_the_scaled_card_height`, `switcher::layout::tests::an_extreme_aspect_ratio_is_clamped_into_band`, `switcher::layout::tests::an_unreadable_or_zero_height_rect_falls_back_to_sixteen_by_nine`, `switcher::layout::tests::no_packed_row_ever_exceeds_the_content_width`, `switcher::layout::tests::rows_derive_from_work_area_height_on_short_displays`, `switcher::layout::tests::row_count_is_never_zero`, `switcher::layout::tests::total_pages_comes_from_packing_and_no_page_is_empty`, `switcher::layout::tests::geometry_scales_with_monitor_dpi`, `switcher::layout::tests::candidates_beyond_one_page_paginate_rather_than_shrink`, `switcher::selection::tests::up_and_down_pick_the_greatest_horizontal_overlap_across_ragged_rows`.

---

### Verification Summary
- **Workspace Test Suite:** 419 daemon tests + 200 settings tests + 71 shared tests (690 tests total) all GREEN.
- **Compiler Hygiene:** `cargo fmt --all -- --check` clean; `cargo clippy --workspace --all-targets -- -D warnings` clean.
- **Production Binaries:** `./build.ps1 -Mode prod` succeeded (`wiradesk.exe` + `wiradesk-settings.exe`).
- **Public Export Hygiene:** `scripts/verify-public-export.ps1` passes 10/10 checks.
- **Method Validation:** `validate.py --generate` passes GREEN with zero findings.
