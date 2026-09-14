# 01: Defect DEF-21 — Visual switcher mouse click selection synchronization & default hold delay threshold 300ms

**What to build:** Fix `DEF-21` (`.control/registry/defects.yaml`).
1. In the visual switcher overlay, clicking a window card with the mouse cursor does not reliably activate the clicked window as the active foreground focus. While keyboard navigation (`next`, `prev`, `up`, `down`) updates both `SwitcherController.selected_index` and `SwitcherOverlay.selected_index`, mouse hover and click events (`WM_MOUSEMOVE` and `WM_LBUTTONDOWN`) in `SwitcherOverlay`'s window procedure only updated `overlay.selected_index`. When `Command::SwitcherCommit` is processed, `sw.candidates_from_selection()` queries `SwitcherController.selected_index`, which remained stuck on the initial keyboard selection, ignoring the clicked card. Synchronize selection state so `candidates_from_selection()` reflects the overlay's selected card.
2. Update the default visual switcher hold delay threshold (`visual_hold_delay_ms`) from 150 ms to 300 ms across shared config (`crates/shared/src/config.rs`), Settings UI (`crates/settings/ui/panes/general_pane.slint`, `crates/settings/ui/main_window.slint`), and companion tests.

**Blocked by:** None (can start immediately)

**Status:** ready-for-agent

## Acceptance Criteria

- [ ] **Mouse Click Selection Synchronization**:
      When a card is hovered or clicked in `crates/daemon/src/switcher/overlay.rs`, the selected candidate index is reflected in `SwitcherController` and `candidates_from_selection()`. On `Command::SwitcherCommit`, the first candidate returned by `candidates_from_selection()` MUST be the window corresponding to the clicked card.
- [ ] **Mouse Hover Followed by Keyboard Modifier Commit**:
      When a card is hovered by the mouse cursor, `SwitcherOverlay` updates its selection highlight, and a subsequent modifier release without mouse click (`Command::SwitcherCommit`) commits the hovered card window rather than reverting to the initial keyboard selection.
- [ ] **Multi-Page Mouse Selection Support**:
      Clicking a card on page > 0 correctly incorporates `page_start` and activates the intended target window without out-of-bounds indexing or page-wrap desync.
- [ ] **Default Hold Delay Raised to 300 ms**:
      Update `shared::config::SwitcherConfig::DEFAULT_HOLD_DELAY_MS` to `300`. Update the initial value of `visual_hold_delay_ms` in `crates/settings/ui/panes/general_pane.slint` and `crates/settings/ui/main_window.slint` to `300`. Existing validation clamping band (`100..=500` ms) remains unchanged.
- [ ] **Unit and Integration Test Coverage**:
      Add unit tests in `crates/daemon/src/switcher/mod.rs` verifying:
      1. Mouse click selection updates candidate ordering in `SwitcherController::candidates_from_selection()`.
      2. Mouse hover updates selection for subsequent modifier release commits.
      3. Default hold delay threshold is 300 ms across shared config, daemon, and settings test assertions.
- [ ] **Workspace Test Suite**:
      All workspace tests pass green (`WIRADESK_SKIP_MANIFEST=1 cargo test --workspace`).
