# SPEC-29: Default Custom-Percentage Top Edge Snap to 33%

## Problem Statement

Currently, the custom-percentage edge snap feature (`UC-9`, satisfying `FR-26`) defaults all four screen edges (`percent_left`, `percent_right`, `percent_top`, and `percent_bottom`) uniformly to 67% (`DEFAULT_SNAP_PERCENT = 67`).

In vertical window layouts and multi-monitor setups, users frequently prefer the top edge snap (`Ctrl + Alt + Shift + Up`) to occupy the top third (33%) of the screen height by default (ideal for reference panes, terminal consoles, or status dashboards), while horizontal left/right splits and bottom split remain at the dominant 67% ratio.

The user has explicitly instructed:
> "agar snap custom top default 33%, selebihnya tetap."

## Proposed Solution

1. **Shared Constants (`crates/shared/src/constants.rs`):**
   - Introduce `pub const DEFAULT_SNAP_PERCENT_TOP: u32 = 33;` for the custom top-edge snap default.
   - Retain `pub const DEFAULT_SNAP_PERCENT: u32 = 67;` and clarify its documentation comment as the default for left, right, and bottom edges (`percent_left`, `percent_right`, and `percent_bottom`).
   - Retain `MIN_SNAP_PERCENT = 1` and `MAX_SNAP_PERCENT = 99`.

2. **Configuration Defaults & Deserialization (`crates/shared/src/config.rs`):**
   - In `SnappingConfig::default()`, set `percent_top: crate::constants::DEFAULT_SNAP_PERCENT_TOP` (33).
   - Keep `percent_left`, `percent_right`, and `percent_bottom` at `DEFAULT_SNAP_PERCENT` (67).
   - Update documentation comment for `percent_top` to indicate `(default 33)`.
   - `SnappingConfig` carries `#[serde(default)]`. Loading an existing configuration preserves any explicit `percent_top = N` on disk; an omitted `percent_top` defaults in memory to `33`.
   - Add/update tests:
     - Rename `default_snapping_percentages_are_67` to `default_snapping_percentages_have_top_at_33_and_others_at_67` asserting `percent_top == 33` and `percent_left/right/bottom == 67`.
     - Update `partial_snapping_percentages_preserve_explicit_values_and_default_omitted_fields` to test explicit `percent_top = 50` loads as `50` alongside omitted top loading as `33`.
     - Verify `percent_snap_fields_roundtrip_through_toml` and `frozen_snapping_defaults` assert `percent_top == 33`.

3. **Settings Model and UI Invariants (`crates/settings/`):**
   - Verified dependency: `ShortcutField::SnapPercentTop.percent()` already reads `cfg.snapping.percent_top` dynamically, and Settings Restore Defaults copies `Config::default()`. No UI code change is required.
   - Update `persistence::tests::default_config_uses_frozen_shortcuts` to assert `cfg.snapping.percent_top == 33` and other edges `== 67`.

4. **Public Documentation and Corpus Alignment:**
   - Update `README.md` lines 41 and 51 to state custom percentage defaults to 67% with top edge defaulting to 33%.
   - Mirror these updates in localized README files (`docs/README.zh-CN.md`, `docs/README.ja.md`, `docs/README.id.md`) when authored/touched.
   - Update `docs/CONFIGURATION.md` to show `percent_top = 33` in sample configuration.
   - Update `FR-26` in `.control/registry/requirements-wira-desk.yaml` and `UC-9` in `.what/window-management/04-usecases/UC-9-snap-window-custom-percentage.md`.


## User Stories & Use Cases

- **UC-9 (Satisfying FR-26):** When a user launches Wira Desk with a default configuration and presses `Ctrl + Alt + Shift + Up`, the active window snaps flush to the top edge of the current monitor's working area with a height equal to 33% of the work area height.
- Left (`Ctrl + Alt + Shift + Left`), Right (`Ctrl + Alt + Shift + Right`), and Bottom (`Ctrl + Alt + Shift + Down`) remain at 67% of the work area width/height.
