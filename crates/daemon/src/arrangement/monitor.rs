//! Inter-monitor placement planning. Backward-compatible facade re-exporting from `wiradesk_core`.

pub use wiradesk_core::arrangement::monitor::*;

#[cfg(test)]
mod tests {
    use super::super::fixtures::*;
    use super::super::*;
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
        assert_eq!(at, 0);
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
        assert_eq!(r.width(), dst.rect.width() / 2);
        assert_eq!(r.height(), dst.rect.height());
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
        assert_eq!(back, left_half);
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
                assert!(plan.fits_within(&dst.rect));
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
}
