//! DPI-aware snap planning. Pure geometry — no User32.
//! Coordinates arrive already in physical pixels from a Per-Monitor-V2-aware
//! process, so nothing here scales by DPI. The `dpi` field on [`WorkArea`] is
//! carried for traceability only; applying it would scale coordinates twice.

use super::{PlacementPlan, PlanError, PlanResult, Rect, WorkArea};
use crate::window::WindowId;

/// Minimum allowed percentage for custom-percentage edge snap (1%).
pub const MIN_SNAP_PERCENT: u32 = 1;

/// Maximum allowed percentage for custom-percentage edge snap (99%).
pub const MAX_SNAP_PERCENT: u32 = 99;

/// The x coordinate that divides the work area into halves.
/// An odd width is split deterministically: the left half takes the floor, the
/// right half takes the remainder. Because both halves are computed from this
/// one boundary, `left.right == right.left` always holds — the halves tile the
/// work area with neither a one-pixel gap nor a one-pixel overlap.
fn split_x(work: &Rect) -> Result<i32, PlanError> {
    let width = work
        .checked_width()
        .ok_or(PlanError::UnrepresentableGeometry)?;
    if width <= 0 {
        return Err(PlanError::EmptyOrInvertedWorkArea);
    }
    work.left
        .checked_add(width / 2)
        .ok_or(PlanError::UnrepresentableGeometry)
}

/// The y coordinate that divides the work area into halves.
///
/// The vertical twin of [`split_x`], and deliberately the same shape rather than a second
/// approach: one boundary computed once, with both halves derived from it, so
/// `top.bottom == bottom.top` always holds and the halves tile the work area with neither
/// a one-pixel gap nor a one-pixel overlap. An odd height gives the floor to the top half
/// and the remainder to the bottom, matching how [`split_x`] favours the left.
fn split_y(work: &Rect) -> Result<i32, PlanError> {
    let height = work
        .checked_height()
        .ok_or(PlanError::UnrepresentableGeometry)?;
    if height <= 0 {
        return Err(PlanError::EmptyOrInvertedWorkArea);
    }
    work.top
        .checked_add(height / 2)
        .ok_or(PlanError::UnrepresentableGeometry)
}

/// Validate before planning. Uses [`Rect::validate`] rather than `is_valid`
/// so an unrepresentable work area is reported as such instead of being
/// misfiled as merely empty.
fn ensure_usable(work: &WorkArea) -> Result<(), PlanError> {
    work.rect.validate()
}

/// Left half of the usable work area.
pub fn plan_snap_left(work: &WorkArea, window: WindowId) -> PlanResult {
    ensure_usable(work)?;
    let mid = split_x(&work.rect)?;
    let rect = Rect::new(work.rect.left, work.rect.top, mid, work.rect.bottom)?;
    Ok(PlacementPlan::single(window, rect))
}

/// Right half — the exact complement of [`plan_snap_left`].
pub fn plan_snap_right(work: &WorkArea, window: WindowId) -> PlanResult {
    ensure_usable(work)?;
    let mid = split_x(&work.rect)?;
    let rect = Rect::new(mid, work.rect.top, work.rect.right, work.rect.bottom)?;
    Ok(PlacementPlan::single(window, rect))
}

/// Top half of the usable work area.
pub fn plan_snap_top(work: &WorkArea, window: WindowId) -> PlanResult {
    ensure_usable(work)?;
    let mid = split_y(&work.rect)?;
    let rect = Rect::new(work.rect.left, work.rect.top, work.rect.right, mid)?;
    Ok(PlacementPlan::single(window, rect))
}

/// Bottom half — the exact complement of [`plan_snap_top`].
pub fn plan_snap_bottom(work: &WorkArea, window: WindowId) -> PlanResult {
    ensure_usable(work)?;
    let mid = split_y(&work.rect)?;
    let rect = Rect::new(work.rect.left, mid, work.rect.right, work.rect.bottom)?;
    Ok(PlacementPlan::single(window, rect))
}

/// The complete usable work area.
/// Note this is the *work area*, never full monitor bounds: the taskbar and any
/// reserved appbar are already excluded by whoever supplied the [`WorkArea`],
/// and re-expanding to monitor bounds would cover them.
pub fn plan_snap_maximize(work: &WorkArea, window: WindowId) -> PlanResult {
    ensure_usable(work)?;
    let rect = Rect::new(
        work.rect.left,
        work.rect.top,
        work.rect.right,
        work.rect.bottom,
    )?;
    Ok(PlacementPlan::single(window, rect))
}

/// Which edge a custom-percentage snap is anchored to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapEdge {
    Left,
    Right,
    Top,
    Bottom,
}

fn calculate_extent(dimension: i32, percent: u32) -> Result<i32, PlanError> {
    let extent = (dimension as i64 * percent as i64 / 100) as i32;
    if extent <= 0 {
        return Err(PlanError::EmptyOrInvertedWorkArea);
    }
    Ok(extent)
}

/// Snap against the named edge at `percent` percent of the work area.
pub fn plan_snap_percent(
    work: &WorkArea,
    window: WindowId,
    edge: SnapEdge,
    percent: u32,
) -> PlanResult {
    ensure_usable(work)?;

    if !(MIN_SNAP_PERCENT..=MAX_SNAP_PERCENT).contains(&percent) {
        return Err(PlanError::InvalidWidthPercent(percent));
    }

    let rect = match edge {
        SnapEdge::Left => {
            let width = work
                .rect
                .checked_width()
                .ok_or(PlanError::UnrepresentableGeometry)?;
            let extent = calculate_extent(width, percent)?;
            let right = work
                .rect
                .left
                .checked_add(extent)
                .ok_or(PlanError::UnrepresentableGeometry)?;
            Rect::new(work.rect.left, work.rect.top, right, work.rect.bottom)?
        }
        SnapEdge::Right => {
            let width = work
                .rect
                .checked_width()
                .ok_or(PlanError::UnrepresentableGeometry)?;
            let extent = calculate_extent(width, percent)?;
            let left = work
                .rect
                .right
                .checked_sub(extent)
                .ok_or(PlanError::UnrepresentableGeometry)?;
            Rect::new(left, work.rect.top, work.rect.right, work.rect.bottom)?
        }
        SnapEdge::Top => {
            let height = work
                .rect
                .checked_height()
                .ok_or(PlanError::UnrepresentableGeometry)?;
            let extent = calculate_extent(height, percent)?;
            let bottom = work
                .rect
                .top
                .checked_add(extent)
                .ok_or(PlanError::UnrepresentableGeometry)?;
            Rect::new(work.rect.left, work.rect.top, work.rect.right, bottom)?
        }
        SnapEdge::Bottom => {
            let height = work
                .rect
                .checked_height()
                .ok_or(PlanError::UnrepresentableGeometry)?;
            let extent = calculate_extent(height, percent)?;
            let top = work
                .rect
                .bottom
                .checked_sub(extent)
                .ok_or(PlanError::UnrepresentableGeometry)?;
            Rect::new(work.rect.left, top, work.rect.right, work.rect.bottom)?
        }
    };

    Ok(PlacementPlan::single(window, rect))
}

#[cfg(test)]
pub mod tests {
    use super::super::fixtures::*;
    use super::*;

    fn only(plan: &PlacementPlan) -> Rect {
        assert_eq!(plan.placements.len(), 1);
        plan.placements[0].rect
    }

    const W: WindowId = WindowId(1);

    #[test]
    pub fn test_half_left_and_right() {
        let work = WorkArea::new(Rect::new(0, 0, 1920, 1080).unwrap(), 96).unwrap();
        let left_plan = plan_snap_left(&work, WindowId(1)).unwrap();
        let right_plan = plan_snap_right(&work, WindowId(2)).unwrap();
        assert_eq!(
            left_plan.placements[0].rect,
            Rect::new(0, 0, 960, 1080).unwrap()
        );
        assert_eq!(
            right_plan.placements[0].rect,
            Rect::new(960, 0, 1920, 1080).unwrap()
        );
    }

    #[test]
    fn snap_left_returns_left_half() {
        let work = primary_work_area();
        let r = only(&plan_snap_left(&work, W).unwrap());
        assert_eq!(r, Rect::new(0, 0, 960, 1040).unwrap());
    }

    #[test]
    fn snap_right_returns_complementary_half() {
        let work = primary_work_area();
        let r = only(&plan_snap_right(&work, W).unwrap());
        assert_eq!(r, Rect::new(960, 0, 1920, 1040).unwrap());
    }

    #[test]
    fn halves_tile_the_work_area_without_gap_or_overlap() {
        for work in [
            primary_work_area(),
            negative_origin_work_area(),
            odd_width_work_area(),
        ] {
            let left = only(&plan_snap_left(&work, W).unwrap());
            let right = only(&plan_snap_right(&work, W).unwrap());
            assert_eq!(left.right, right.left, "gap or overlap at the seam");
            assert_eq!(left.left, work.rect.left);
            assert_eq!(right.right, work.rect.right);
            assert_eq!(
                left.width() + right.width(),
                work.rect.width(),
                "halves must exactly cover the work area"
            );
        }
    }

    #[test]
    fn odd_width_is_split_deterministically() {
        let work = odd_width_work_area(); // width 1367
        let left = only(&plan_snap_left(&work, W).unwrap());
        let right = only(&plan_snap_right(&work, W).unwrap());
        assert_eq!(left.width(), 683);
        assert_eq!(right.width(), 684);
        for _ in 0..4 {
            assert_eq!(only(&plan_snap_left(&work, W).unwrap()), left);
            assert_eq!(only(&plan_snap_right(&work, W).unwrap()), right);
        }
    }

    #[test]
    fn halves_stay_inside_the_work_area() {
        for work in [
            primary_work_area(),
            negative_origin_work_area(),
            odd_width_work_area(),
        ] {
            assert!(plan_snap_left(&work, W).unwrap().fits_within(&work.rect));
            assert!(plan_snap_right(&work, W).unwrap().fits_within(&work.rect));
        }
    }

    #[test]
    fn halves_work_at_negative_origin() {
        let work = negative_origin_work_area(); // left -1920, width 1920
        let left = only(&plan_snap_left(&work, W).unwrap());
        let right = only(&plan_snap_right(&work, W).unwrap());
        assert_eq!(left, Rect::new(-1920, -200, -960, 880).unwrap());
        assert_eq!(right, Rect::new(-960, -200, 0, 880).unwrap());
    }

    #[test]
    fn snap_top_returns_top_half() {
        let work = primary_work_area();
        let r = only(&plan_snap_top(&work, W).unwrap());
        assert_eq!(r, Rect::new(0, 0, 1920, 520).unwrap());
    }

    #[test]
    fn snap_bottom_returns_complementary_half() {
        let work = primary_work_area();
        let r = only(&plan_snap_bottom(&work, W).unwrap());
        assert_eq!(r, Rect::new(0, 520, 1920, 1040).unwrap());
    }

    #[test]
    fn snap_maximize_returns_work_area() {
        let work = primary_work_area();
        let r = only(&plan_snap_maximize(&work, W).unwrap());
        assert_eq!(r, work.rect);
    }

    #[test]
    fn snap_percent_left_and_right() {
        let work = primary_work_area(); // 1920x1040
        let left = only(&plan_snap_percent(&work, W, SnapEdge::Left, 33).unwrap());
        let right = only(&plan_snap_percent(&work, W, SnapEdge::Right, 67).unwrap());
        assert_eq!(left.width(), 633);
        assert_eq!(right.width(), 1286);
        assert_eq!(left.left, 0);
        assert_eq!(right.right, 1920);
    }
}
