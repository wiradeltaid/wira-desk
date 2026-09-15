---
id: SPEC-28-01
component: settings
satisfies: [UC-4, UC-14]
blocked_by: []
status: open
tests:
  - shortcut_row_slint_snapshot::tests::modal_reset_dialog_has_compact_content_derived_height
  - shortcut_row_slint_snapshot::tests::modal_reset_dialog_is_vertically_centered_in_window
---

# 01: Compact modal dialog height and content-derived container geometry

**What to build:**
1. In `crates/settings/ui/main_window.slint`:
   - Within `modal_focus := FocusScope`, update the confirmation dialog container `Rectangle`:
     - Bind height directly to inner layout preferred height: `height: dialog_layout.preferred-height;`.
     - Name the inner `VerticalLayout` as `dialog_layout := VerticalLayout`.
     - Give the card container accessible role and label (`accessible-role: group; accessible-label: "Factory reset confirmation card";`) so UI test harnesses can locate and measure its actual rendered geometry.
     - Ensure positioning uses:
       ```slint
       width: 440px;
       height: dialog_layout.preferred-height;
       x: (parent.width - self.width) / 2;
       y: (parent.height - self.height) / 2;
       ```
   - Keep 12px border radius, background, drop shadow, 24px layout padding, and 14px layout spacing.
2. In `crates/settings/src/shortcut_row_slint_snapshot.rs`:
   - Add unit/snapshot tests asserting that when `factory_reset_dialog_open = true`:
     - Dialog card container geometry is directly measured at runtime: height is strictly content-proportional (`< 260px`, `>= 180px`), not expanding to window height (560px).
     - Dialog card is centered vertically on screen (`y > 100px` and `y + height < 460px`).
     - Buttons and text remain accessible and interactive.

**Blocked by:** None.

**Status:** open

## Acceptance Criteria

- [ ] The Factory Reset modal confirmation dialog card container uses `height: dialog_layout.preferred-height;`.
- [ ] The dialog card exposes a test-addressable identifier (`accessible-role: group; accessible-label: "Factory reset confirmation card";`).
- [ ] The dialog card renders compactly (~200px-230px height) rather than stretching to the full window height.
- [ ] The dialog card is centered vertically and horizontally within the Settings window.
- [ ] Keyboard navigation (Tab, Arrow keys, Return, Space, Escape) and mouse click handlers continue to operate smoothly without regressions.
- [ ] Tests named in frontmatter measure rendered runtime geometry and pass cleanly.
