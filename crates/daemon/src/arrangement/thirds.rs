//! DPI-aware thirds snap planning. Backward-compatible facade re-exporting from `wiradesk_core`.

pub use wiradesk_core::arrangement::thirds::*;

#[cfg(test)]
pub mod tests {
    use super::super::fixtures::*;
    use super::super::*;
    use super::*;

    fn only(plan: &PlacementPlan) -> Rect {
        assert_eq!(plan.placements.len(), 1);
        plan.placements[0].rect
    }

    const W: WindowId = WindowId(1);

    #[test]
    fn thirds_tile_the_work_area_without_gap_or_overlap() {
        let work = primary_work_area();
        let left = only(&plan_snap_third(&work, W, ThirdColumn::Left).unwrap());
        let middle = only(&plan_snap_third(&work, W, ThirdColumn::Middle).unwrap());
        let right = only(&plan_snap_third(&work, W, ThirdColumn::Right).unwrap());

        assert_eq!(left.left, work.rect.left);
        assert_eq!(right.right, work.rect.right);

        assert_eq!(left.right, middle.left);
        assert_eq!(middle.right, right.left);

        assert_eq!(left.top, work.rect.top);
        assert_eq!(left.bottom, work.rect.bottom);
        assert_eq!(middle.top, work.rect.top);
        assert_eq!(middle.bottom, work.rect.bottom);
        assert_eq!(right.top, work.rect.top);
        assert_eq!(right.bottom, work.rect.bottom);

        assert_eq!(
            left.width() + middle.width() + right.width(),
            work.rect.width()
        );
    }

    #[test]
    fn remainder_width_goes_to_the_middle_column() {
        let work = odd_width_work_area();
        assert_eq!(work.rect.width(), 1367);
        let left = only(&plan_snap_third(&work, W, ThirdColumn::Left).unwrap());
        let middle = only(&plan_snap_third(&work, W, ThirdColumn::Middle).unwrap());
        let right = only(&plan_snap_third(&work, W, ThirdColumn::Right).unwrap());

        assert_eq!(left.width(), 455);
        assert_eq!(middle.width(), 457);
        assert_eq!(right.width(), 455);
        assert_eq!(left.width() + middle.width() + right.width(), 1367);

        let work1000 = WorkArea::new(Rect::new(0, 0, 1000, 600).unwrap(), 96).unwrap();
        let l = only(&plan_snap_third(&work1000, W, ThirdColumn::Left).unwrap());
        let m = only(&plan_snap_third(&work1000, W, ThirdColumn::Middle).unwrap());
        let r = only(&plan_snap_third(&work1000, W, ThirdColumn::Right).unwrap());

        assert_eq!(l.width(), 333);
        assert_eq!(m.width(), 334);
        assert_eq!(r.width(), 333);
        assert_eq!(l.width() + m.width() + r.width(), 1000);
    }

    #[test]
    fn narrow_work_area_is_refused() {
        let narrow = WorkArea::new(Rect::new(0, 0, 2, 100).unwrap(), 96).unwrap();
        assert!(plan_snap_third(&narrow, W, ThirdColumn::Left).is_err());
        assert!(plan_snap_third(&narrow, W, ThirdColumn::Middle).is_err());
        assert!(plan_snap_third(&narrow, W, ThirdColumn::Right).is_err());

        let narrow1 = WorkArea::new(Rect::new(0, 0, 1, 100).unwrap(), 96).unwrap();
        assert!(plan_snap_third(&narrow1, W, ThirdColumn::Left).is_err());
    }

    #[test]
    fn negative_origin_work_area_is_supported() {
        let work = negative_origin_work_area();
        let left = only(&plan_snap_third(&work, W, ThirdColumn::Left).unwrap());
        let middle = only(&plan_snap_third(&work, W, ThirdColumn::Middle).unwrap());
        let right = only(&plan_snap_third(&work, W, ThirdColumn::Right).unwrap());

        assert_eq!(left.left, -1920);
        assert_eq!(left.right, -1280);
        assert_eq!(middle.left, -1280);
        assert_eq!(middle.right, -640);
        assert_eq!(right.left, -640);
        assert_eq!(right.right, 0);

        assert_eq!(left.width() + middle.width() + right.width(), 1920);
    }

    #[test]
    fn thirds_stay_inside_the_work_area() {
        for work in [
            primary_work_area(),
            negative_origin_work_area(),
            odd_width_work_area(),
        ] {
            assert!(plan_snap_third(&work, W, ThirdColumn::Left)
                .unwrap()
                .fits_within(&work.rect));
            assert!(plan_snap_third(&work, W, ThirdColumn::Middle)
                .unwrap()
                .fits_within(&work.rect));
            assert!(plan_snap_third(&work, W, ThirdColumn::Right)
                .unwrap()
                .fits_within(&work.rect));
        }
    }
}
