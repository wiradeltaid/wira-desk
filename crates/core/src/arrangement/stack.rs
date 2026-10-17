//! Overlapping-stack planning. Pure geometry — no User32.
//! Up to three same-application windows are laid out at a configured width,
//! spread evenly across the leftover horizontal travel so each keeps a visible
//! clickable edge. Disabled or empty input is a **successful no-op**, never an
//! error.

use super::{Placement, PlacementPlan, PlanError, PlanResult, Rect, WorkArea, STACK_MAX_WINDOWS};
use crate::window::WindowId;

/// Stack configuration options for pure planning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StackOptions {
    pub stack_width_percent: u32,
}

impl StackOptions {
    pub const fn new(stack_width_percent: u32) -> Self {
        Self {
            stack_width_percent,
        }
    }
}

/// Plan an overlapping stack using explicit options.
pub fn plan_stack(options: &StackOptions, work: &WorkArea, candidates: &[WindowId]) -> PlanResult {
    plan_stack_with_percent(options.stack_width_percent, work, candidates)
}

/// Plan an overlapping stack for `candidates` at `percent` width.
pub fn plan_stack_with_percent(
    percent: u32,
    work: &WorkArea,
    candidates: &[WindowId],
) -> PlanResult {
    work.rect.validate()?;

    if percent == 0 || percent > 100 {
        return Err(PlanError::InvalidWidthPercent(percent));
    }

    if candidates.is_empty() {
        return Ok(PlacementPlan::empty());
    }

    let work_width = work.rect.width();
    let window_width = (work_width as i64 * percent as i64 / 100) as i32;
    if window_width <= 0 {
        return Err(PlanError::InvalidWidthPercent(percent));
    }

    // Leftover horizontal room the anchors are distributed across.
    let travel = work_width - window_width;

    // Live order is preserved; only the first three are arranged.
    let chosen: Vec<WindowId> = candidates.iter().copied().take(STACK_MAX_WINDOWS).collect();

    let placements = chosen
        .iter()
        .enumerate()
        .map(|(i, window)| {
            let offset = anchor_offset(i, chosen.len(), travel);
            let left = work
                .rect
                .left
                .checked_add(offset)
                .ok_or(PlanError::UnrepresentableGeometry)?;
            let right = left
                .checked_add(window_width)
                .ok_or(PlanError::UnrepresentableGeometry)?;
            let rect = Rect::new(left, work.rect.top, right, work.rect.bottom)?;
            Ok(Placement {
                window: *window,
                rect,
            })
        })
        .collect::<Result<Vec<Placement>, PlanError>>()?;

    Ok(PlacementPlan { placements })
}

/// Horizontal anchor for window `index` of `count`, across `travel` pixels.
/// One window is centred; two are pinned to the ends; three take start, middle,
/// and end. Distributing across the *travel* range rather than the full width
/// is what keeps every rectangle inside the work area by construction.
fn anchor_offset(index: usize, count: usize, travel: i32) -> i32 {
    match count {
        1 => travel / 2,
        _ => {
            let denom = (count - 1) as i64;
            ((index as i64 * travel as i64) / denom) as i32
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::super::fixtures::*;
    use super::*;

    #[test]
    pub fn test_stack_offset() {
        let work = WorkArea::new(Rect::new(0, 0, 1920, 1080).unwrap(), 96).unwrap();
        let windows = vec![WindowId(1), WindowId(2), WindowId(3)];
        let plan = plan_stack_with_percent(50, &work, &windows).unwrap();
        assert_eq!(plan.placements.len(), 3);
        assert_eq!(plan.placements[0].rect.left, 0);
        assert_eq!(plan.placements[1].rect.left, 480);
        assert_eq!(plan.placements[2].rect.left, 960);
    }

    #[test]
    fn zero_candidates_is_a_successful_noop() {
        let plan = plan_stack_with_percent(50, &primary_work_area(), &[]).unwrap();
        assert!(plan.is_noop());
    }

    #[test]
    fn live_order_is_preserved() {
        let candidates = vec![WindowId(7), WindowId(3), WindowId(9)];
        let plan = plan_stack_with_percent(50, &primary_work_area(), &candidates).unwrap();
        let got: Vec<WindowId> = plan.placements.iter().map(|p| p.window).collect();
        assert_eq!(got, candidates);
    }

    #[test]
    fn no_more_than_three_windows_are_arranged() {
        let plan = plan_stack_with_percent(50, &primary_work_area(), &windows(7)).unwrap();
        assert_eq!(plan.placements.len(), STACK_MAX_WINDOWS);
        let got: Vec<WindowId> = plan.placements.iter().map(|p| p.window).collect();
        assert_eq!(got, windows(3));
    }

    #[test]
    fn one_window_is_centred() {
        let work = primary_work_area(); // width 1920
        let plan = plan_stack_with_percent(50, &work, &windows(1)).unwrap();
        let r = plan.placements[0].rect;
        assert_eq!(r.width(), 960);
        assert_eq!(r.left, 480);
        assert_eq!(r.right, 1440);
        assert_eq!(r.left - work.rect.left, work.rect.right - r.right);
    }

    #[test]
    fn two_windows_anchor_left_and_right() {
        let work = primary_work_area();
        let plan = plan_stack_with_percent(50, &work, &windows(2)).unwrap();
        assert_eq!(plan.placements[0].rect.left, work.rect.left);
        assert_eq!(plan.placements[1].rect.right, work.rect.right);
    }
}
