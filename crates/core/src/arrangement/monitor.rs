//! Inter-monitor placement planning. Pure geometry — no User32.
//!
//! Deliberately not part of `snap.rs`: every function there takes exactly **one**
//! [`WorkArea`] and divides it, and that single-work-area invariant is what makes the file
//! easy to reason about. This operation needs two work areas and an ordered list of them,
//! so folding it in would have made `snap.rs`'s one clear property untrue (`DEC-007`).
//!
//! Nothing here holds a monitor handle. Caching one is forbidden — it is a handle rather
//! than an identity and does not survive an unplug — and a planner that never receives one
//! cannot cache one. The list arrives as work areas, per invocation, from whoever enumerated
//! it.

use super::{PlacementPlan, PlanError, PlanResult, Rect, WorkArea};
use crate::window::WindowId;

/// The monitor after `from` in enumeration order, wrapping from the last back to the first.
pub fn next_monitor_index(count: usize, from: usize) -> Option<usize> {
    if count < 2 || from >= count {
        return None;
    }
    Some((from + 1) % count)
}

/// Map one coordinate from a source span onto a destination span, by proportion.
fn map_span(value: i32, src_start: i32, src_extent: i32, dst_start: i32, dst_extent: i32) -> i32 {
    debug_assert!(
        src_extent > 0,
        "callers validate the source work area first"
    );
    let offset = i64::from(value) - i64::from(src_start);
    let scaled =
        (offset * i64::from(dst_extent) + i64::from(src_extent) / 2) / i64::from(src_extent);
    let mapped = i64::from(dst_start).saturating_add(scaled);
    mapped.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32
}

/// Clamp `rect` into `bounds`, or `None` when the intersection is empty.
fn intersect(rect: Rect, bounds: Rect) -> Option<Rect> {
    let clamped = Rect {
        left: rect.left.max(bounds.left),
        top: rect.top.max(bounds.top),
        right: rect.right.min(bounds.right),
        bottom: rect.bottom.min(bounds.bottom),
    };
    clamped.validate().ok().map(|()| clamped)
}

/// Place `window` on `destination`, holding the same share of the work area it held on
/// `source`.
pub fn plan_move_to_monitor(
    source: &WorkArea,
    destination: &WorkArea,
    window: WindowId,
    window_rect: Rect,
) -> PlanResult {
    source.rect.validate()?;
    destination.rect.validate()?;

    let clamped = intersect(window_rect, source.rect).ok_or(PlanError::EmptyOrInvertedWorkArea)?;

    let (sw, sh) = (source.rect.width(), source.rect.height());
    let (dw, dh) = (destination.rect.width(), destination.rect.height());

    let left = map_span(
        clamped.left,
        source.rect.left,
        sw,
        destination.rect.left,
        dw,
    );
    let right = map_span(
        clamped.right,
        source.rect.left,
        sw,
        destination.rect.left,
        dw,
    );
    let top = map_span(clamped.top, source.rect.top, sh, destination.rect.top, dh);
    let bottom = map_span(
        clamped.bottom,
        source.rect.top,
        sh,
        destination.rect.top,
        dh,
    );

    let rect = Rect::new(left, top, right, bottom)?;
    Ok(PlacementPlan::single(window, rect))
}

#[cfg(test)]
mod tests {
    use super::super::fixtures::*;
    use super::*;

    const W: WindowId = WindowId(1);

    fn only(plan: &PlacementPlan) -> Rect {
        assert_eq!(plan.placements.len(), 1);
        plan.placements[0].rect
    }

    fn wide_secondary() -> WorkArea {
        WorkArea::new(Rect::new(1920, 0, 5760, 2120).unwrap(), 144).unwrap()
    }

    #[test]
    fn a_single_monitor_has_no_next() {
        assert_eq!(next_monitor_index(1, 0), None);
    }

    #[test]
    fn no_monitor_at_all_has_no_next() {
        assert_eq!(next_monitor_index(0, 0), None);
    }

    #[test]
    fn next_monitor_wraps_from_last_to_first() {
        assert_eq!(next_monitor_index(3, 0), Some(1));
        assert_eq!(next_monitor_index(3, 1), Some(2));
        assert_eq!(next_monitor_index(3, 2), Some(0));
    }

    #[test]
    fn pressing_once_per_monitor_returns_to_the_start() {
        let count = 4;
        let mut at = 0;
        for _ in 0..count {
            at = next_monitor_index(count, at).expect("more than one monitor");
        }
        assert_eq!(at, 0, "the cycle must close");
    }

    #[test]
    fn an_out_of_range_source_has_no_next() {
        assert_eq!(next_monitor_index(2, 2), None);
        assert_eq!(next_monitor_index(2, 99), None);
    }

    #[test]
    fn a_left_half_stays_a_left_half_across_monitors() {
        let src = primary_work_area();
        let dst = wide_secondary();
        let left_half = Rect::new(0, 0, 960, 1040).unwrap();
        let r = only(&plan_move_to_monitor(&src, &dst, W, left_half).unwrap());
        assert_eq!(r, Rect::new(1920, 0, 3840, 2120).unwrap());
        assert_eq!(r.width(), dst.rect.width() / 2, "still half the width");
        assert_eq!(r.height(), dst.rect.height(), "still full height");
    }

    #[test]
    fn a_top_half_stays_a_top_half_across_monitors() {
        let src = primary_work_area();
        let dst = wide_secondary();
        let top_half = Rect::new(0, 0, 1920, 520).unwrap();
        let r = only(&plan_move_to_monitor(&src, &dst, W, top_half).unwrap());
        assert_eq!(r, Rect::new(1920, 0, 5760, 1060).unwrap());
        assert_eq!(r.height(), dst.rect.height() / 2);
    }

    #[test]
    fn a_maximized_window_fills_the_destination_work_area() {
        let src = primary_work_area();
        let dst = wide_secondary();
        let r = only(&plan_move_to_monitor(&src, &dst, W, src.rect).unwrap());
        assert_eq!(r, dst.rect);
    }

    #[test]
    fn absolute_size_is_not_preserved_and_that_is_the_point() {
        let src = primary_work_area();
        let dst = wide_secondary();
        let left_half = Rect::new(0, 0, 960, 1040).unwrap();
        let r = only(&plan_move_to_monitor(&src, &dst, W, left_half).unwrap());
        assert_ne!(r.width(), left_half.width());
        assert_ne!(r.height(), left_half.height());
    }

    #[test]
    fn the_move_is_reversible_for_a_half_screen_window() {
        let a = primary_work_area();
        let b = wide_secondary();
        let left_half = Rect::new(0, 0, 960, 1040).unwrap();
        let there = only(&plan_move_to_monitor(&a, &b, W, left_half).unwrap());
        let back = only(&plan_move_to_monitor(&b, &a, W, there).unwrap());
        assert_eq!(back, left_half, "a round trip must land where it started");
    }

    #[test]
    fn the_placement_always_lands_inside_the_destination_work_area() {
        let src = primary_work_area();
        for dst in [wide_secondary(), negative_origin_work_area(), src] {
            for rect in [
                Rect::new(0, 0, 960, 1040).unwrap(),
                Rect::new(960, 520, 1920, 1040).unwrap(),
                Rect::new(100, 100, 300, 300).unwrap(),
                src.rect,
            ] {
                let plan = plan_move_to_monitor(&src, &dst, W, rect).unwrap();
                assert!(
                    plan.fits_within(&dst.rect),
                    "escaped the destination work area: {:?} -> {:?}",
                    rect,
                    plan.placements[0].rect
                );
            }
        }
    }

    #[test]
    fn a_window_extending_past_the_work_area_is_clamped_before_mapping() {
        let src = primary_work_area();
        let dst = wide_secondary();
        let overhang = Rect::new(-8, -8, 1928, 1080).unwrap();
        let plan = plan_move_to_monitor(&src, &dst, W, overhang).unwrap();
        assert!(plan.fits_within(&dst.rect));
        assert_eq!(only(&plan), dst.rect);
    }

    #[test]
    fn a_window_entirely_off_the_source_work_area_is_refused() {
        let src = primary_work_area();
        let dst = wide_secondary();
        let elsewhere = Rect::new(5000, 5000, 5200, 5200).unwrap();
        assert_eq!(
            plan_move_to_monitor(&src, &dst, W, elsewhere),
            Err(PlanError::EmptyOrInvertedWorkArea)
        );
    }

    #[test]
    fn negative_destination_coordinates_are_supported() {
        let src = primary_work_area();
        let dst = negative_origin_work_area();
        let left_half = Rect::new(0, 0, 960, 1040).unwrap();
        let r = only(&plan_move_to_monitor(&src, &dst, W, left_half).unwrap());
        assert_eq!(r.left, dst.rect.left);
        assert_eq!(r.width(), dst.rect.width() / 2);
    }

    #[test]
    fn an_empty_source_or_destination_is_refused_without_a_placement() {
        let good = primary_work_area();
        let empty = WorkArea {
            rect: Rect {
                left: 0,
                top: 0,
                right: 0,
                bottom: 1040,
            },
            dpi: 96,
        };
        assert_eq!(
            plan_move_to_monitor(&empty, &good, W, good.rect),
            Err(PlanError::EmptyOrInvertedWorkArea)
        );
        assert_eq!(
            plan_move_to_monitor(&good, &empty, W, good.rect),
            Err(PlanError::EmptyOrInvertedWorkArea)
        );
    }

    #[test]
    fn a_share_that_rounds_to_nothing_is_refused_rather_than_invented() {
        let src = primary_work_area();
        let tiny = WorkArea::new(Rect::new(0, 0, 4, 4).unwrap(), 96).unwrap();
        let sliver = Rect::new(0, 0, 2, 1040).unwrap();
        assert_eq!(
            plan_move_to_monitor(&src, &tiny, W, sliver),
            Err(PlanError::EmptyOrInvertedWorkArea)
        );
    }

    #[test]
    fn dpi_never_scales_the_result() {
        let src = primary_work_area();
        let rect = Rect::new(0, 0, 960, 1040).unwrap();
        let mut seen: Option<Rect> = None;
        for dpi in [96u32, 120, 144, 192] {
            let dst = WorkArea::new(Rect::new(1920, 0, 3840, 1040).unwrap(), dpi).unwrap();
            let r = only(&plan_move_to_monitor(&src, &dst, W, rect).unwrap());
            match seen {
                None => seen = Some(r),
                Some(prev) => assert_eq!(prev, r, "DPI {dpi} changed the plan"),
            }
        }
    }
}
