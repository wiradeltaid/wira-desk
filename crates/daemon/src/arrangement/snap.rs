//! DPI-aware snap planning. Backward-compatible facade re-exporting from `wiradesk_core`.

pub use wiradesk_core::arrangement::snap::*;

#[cfg(test)]
mod tests {
    use super::super::fixtures::*;
    use super::super::*;
    use super::*;

    fn only(plan: &PlacementPlan) -> Rect {
        assert_eq!(plan.placements.len(), 1);
        plan.placements[0].rect
    }

    const W: WindowId = WindowId(1);

    // --- halves ------------------------------------------

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
        // Repeat: the split must not drift.
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

    // --- vertical halves --------------------------------------

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
