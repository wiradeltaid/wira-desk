---
id: SPEC-24-01
component: settings
satisfies: [UC-9, FR-26]
blocked_by: []
status: closed
tests:
  - config::tests::default_snapping_percentages_are_67
  - config::tests::partial_snapping_percentages_preserve_explicit_values_and_default_omitted_fields
  - config::tests::percent_snap_fields_roundtrip_through_toml
  - persistence::tests::default_config_uses_frozen_shortcuts
  - persistence::tests::an_out_of_range_percentage_is_rejected_before_save
---

# 01: Feature — Update default snap to edge percentage to 67%

**What to build:** Change the fresh-default percentage for all four custom-percentage edge snaps (`snap_percent_left`, `snap_percent_right`, `snap_percent_top`, and `snap_percent_bottom`) from 50% to 67%. This changes neither a user-selected stored percentage nor the allowed 1..=99 range.

## Implementation

1. **Shared default (`crates/shared/src/constants.rs`, `crates/shared/src/config.rs`):**
   - Change `DEFAULT_SNAP_PERCENT` from `50` to `67` and its comment to 67%.
   - Keep `MIN_SNAP_PERCENT = 1` and `MAX_SNAP_PERCENT = 99` unchanged.
   - Keep all four `SnappingConfig::default()` fields sourced from `DEFAULT_SNAP_PERCENT`; update their stale 50% field comments.

2. **Configuration compatibility:**
   - `SnappingConfig` already has container-level `#[serde(default)]`. Loading an existing `config.toml` preserves every explicit percentage on disk; an omitted percentage receives the current `SnappingConfig::default()` value in memory.
   - Loading does **not** rewrite `config.toml`. The defaulted omitted fields are serialized only when Settings later saves the configuration through the normal explicit save path. No installer, daemon load, or upgrade migration may overwrite a stored percentage merely because this default changes.

3. **Existing Settings and daemon paths:**
   - No direct `crates/settings/src/main.rs` change is required: each custom row reads its corresponding `ShortcutField::percent()` value, and its fallback already reads `DEFAULT_SNAP_PERCENT`.
   - Preserve the existing Settings pre-save validation and daemon reload/planner validation for all four 1..=99 values.

4. **Documentation alignment:**
   - Update `.what/window-management/04-usecases/UC-9-snap-window-custom-percentage.md` and `.how/settings/05-model/data-model.md` to state 67 as the fresh default.
   - Update authoritative requirement `FR-26` in `.control/registry/requirements-wira-desk.yaml` to state the default as 67% (instead of 50%).
   - Regenerate rendered documents via `uv run .constitution/method/scripts/validate.py --generate`. Do not edit `.what-rendered/` or `.how-rendered/` by hand.

**Blocked by:** none

**Status:** ready-for-agent

## Acceptance Criteria

- [ ] **Default constant and all edges:** `DEFAULT_SNAP_PERCENT` is `67`; a fresh `Config::default().snapping` has `percent_left`, `percent_right`, `percent_top`, and `percent_bottom` all equal to 67.
- [ ] **Mixed old/new TOML:** A `[snapping]` table that explicitly sets `percent_left = 50` loads with `percent_left == 50` while omitted right, top, and bottom fields each equal 67. Serializing that loaded configuration writes these in-memory values without changing the explicit left value.
- [ ] **Explicit user values:** `percent_snap_fields_roundtrip_through_toml` continues to prove all four independently selected stored percentages survive serialization/deserialization unchanged.
- [ ] **Bounds and UI:** The existing 1..=99 Settings pre-save validation and daemon reload/planner validation remain unchanged; an out-of-range custom percentage is still refused before save.
- [ ] **Documentation:** `FR-26` in `.control/registry/requirements-wira-desk.yaml`, `UC-9`, and the settings data-model source reflect 67 as the fresh default; generated rendered documents are regenerated rather than hand-edited.
- [ ] **Workspace Integrity:** `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `$env:WIRADESK_SKIP_MANIFEST = '1'; cargo test --workspace` pass cleanly.
