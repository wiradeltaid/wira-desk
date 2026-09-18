---
id: SPEC-29-01
component: settings
satisfies: [UC-9, FR-26]
blocked_by: []
status: closed
touches:
  - crates/shared/src/config.rs
  - crates/shared/src/constants.rs
  - crates/settings/src/persistence.rs
  - README.md
  - docs/CONFIGURATION.md
  - .control/registry/requirements-wira-desk.yaml
  - .what/window-management/04-usecases/UC-9-snap-window-custom-percentage.md
tests:
  - config::tests::default_snapping_percentages_have_top_at_33_and_others_at_67
  - config::tests::partial_snapping_percentages_preserve_explicit_values_and_default_omitted_fields
  - config::tests::percent_snap_fields_roundtrip_through_toml
  - persistence::tests::default_config_uses_frozen_shortcuts
---

# 01: Feature — Update default snap custom top percentage to 33%

**What to build:** Change the fresh-default percentage for custom-percentage top edge snap (`snap_percent_top` / `percent_top`) from 67% to 33%, while keeping left, right, and bottom edge snaps (`percent_left`, `percent_right`, `percent_bottom`) at 67%.

## Implementation Details

1. **Shared constants (`crates/shared/src/constants.rs`):**
   - Add `pub const DEFAULT_SNAP_PERCENT_TOP: u32 = 33;` for custom top-edge snap.
   - Retain `pub const DEFAULT_SNAP_PERCENT: u32 = 67;` and clarify its documentation comment as the default for left, right, and bottom edges (`percent_left`, `percent_right`, `percent_bottom`).
   - Retain `MIN_SNAP_PERCENT = 1` and `MAX_SNAP_PERCENT = 99`.

2. **Shared configuration (`crates/shared/src/config.rs`):**
   - In `SnappingConfig::default()`:
     - `percent_left: crate::constants::DEFAULT_SNAP_PERCENT,` (67)
     - `percent_right: crate::constants::DEFAULT_SNAP_PERCENT,` (67)
     - `percent_top: crate::constants::DEFAULT_SNAP_PERCENT_TOP,` (33)
     - `percent_bottom: crate::constants::DEFAULT_SNAP_PERCENT,` (67)
   - Update doc comments on `percent_top` (`/// Percentage of work-area height for top-edge snap (default 33).`).
   - Update and rename tests in `crates/shared/src/config.rs`:
     - Rename `default_snapping_percentages_are_67` to `default_snapping_percentages_have_top_at_33_and_others_at_67`, asserting `percent_top == 33` and left/right/bottom `== 67`.
     - In `partial_snapping_percentages_preserve_explicit_values_and_default_omitted_fields()`, add an explicit test case proving `percent_top = 50` deserializes as `50`, while omitted `percent_top` defaults in memory to `33`.
     - In `frozen_snapping_defaults()`, assert `cfg.percent_top == 33`.

3. **Settings persistence & UI (`crates/settings/`):**
   - Verified dependency: `ShortcutField::SnapPercentTop.percent()` already reads `cfg.snapping.percent_top` dynamically, and Restore Defaults copies `Config::default()`. No UI code change is required.
   - In `persistence::tests::default_config_uses_frozen_shortcuts()`, assert `cfg.snapping.percent_top == 33` and other edges `== 67`.

4. **Documentation and Requirement Alignment:**
   - Update `README.md` lines 41 and 51 to reflect that directional custom percentages default to 67%, with the top edge defaulting to 33%.
   - Mirror these claims in localized README files (`docs/README.zh-CN.md`, `docs/README.ja.md`, `docs/README.id.md`) when authored/touched.
   - Update `docs/CONFIGURATION.md` sample TOML to show `percent_top = 33`.
   - Update `FR-26` in `.control/registry/requirements-wira-desk.yaml` to document default 33% for top edge and 67% for left, right, and bottom.
   - Update `.what/window-management/04-usecases/UC-9-snap-window-custom-percentage.md` (trigger/precondition/flow notes).

## Acceptance Criteria

- [x] **Default constants:** `DEFAULT_SNAP_PERCENT_TOP` is 33; `DEFAULT_SNAP_PERCENT` remains 67 (documented for left/right/bottom). Fresh `Config::default().snapping` has `percent_top == 33`, `percent_left == 67`, `percent_right == 67`, and `percent_bottom == 67`.
- [x] **Preservation of explicit values & omitted defaults:** Loading a TOML with explicit `percent_top = 50` preserves 50; omitting `percent_top` in a TOML defaults it to 33 in memory.
- [x] **Roundtrip:** All percentage fields roundtrip cleanly through serialization and deserialization.
- [x] **Corpus & docs:** `README.md`, `docs/CONFIGURATION.md`, `FR-26`, and `UC-9` accurately state top is 33% and left/right/bottom are 67%.
- [x] **Workspace Integrity:** Workspace tests, clippy, and formatting pass cleanly without regressions.
