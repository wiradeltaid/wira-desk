//! Overlapping-stack planning facade forwarding to `wiradesk_core::arrangement::stack`.

use super::{PlanResult, WorkArea};
use crate::cycling::WindowId;
use shared::config::LayoutConfig;

/// Plan an overlapping stack for `candidates` using `LayoutConfig`.
pub fn plan_stack(layout: &LayoutConfig, work: &WorkArea, candidates: &[WindowId]) -> PlanResult {
    wiradesk_core::arrangement::stack::plan_stack_with_percent(
        layout.stack_width_percent,
        work,
        candidates,
    )
}

#[cfg(test)]
mod tests {
    use super::super::fixtures::*;
    use super::super::*;
    use super::*;

    fn enabled(percent: u32) -> LayoutConfig {
        LayoutConfig {
            stack_width_percent: percent,
            ..LayoutConfig::default()
        }
    }

    #[test]
    fn stack_planner_no_longer_reads_a_layout_level_toggle() {
        let layout = enabled(50);
        let plan = plan_stack(&layout, &primary_work_area(), &windows(3)).unwrap();
        assert_eq!(plan.placements.len(), 3);
    }

    #[test]
    fn zero_candidates_is_a_successful_noop() {
        let plan = plan_stack(&enabled(50), &primary_work_area(), &[]).unwrap();
        assert!(plan.is_noop());
    }

    #[test]
    fn live_order_is_preserved() {
        let candidates = vec![WindowId(7), WindowId(3), WindowId(9)];
        let plan = plan_stack(&enabled(50), &primary_work_area(), &candidates).unwrap();
        let got: Vec<WindowId> = plan.placements.iter().map(|p| p.window).collect();
        assert_eq!(got, candidates);
    }

    #[test]
    fn no_more_than_three_windows_are_arranged() {
        let plan = plan_stack(&enabled(50), &primary_work_area(), &windows(7)).unwrap();
        assert_eq!(plan.placements.len(), STACK_MAX_WINDOWS);
        let got: Vec<WindowId> = plan.placements.iter().map(|p| p.window).collect();
        assert_eq!(got, windows(3), "the first three in live order");
    }

    #[test]
    fn one_window_is_centred() {
        let work = primary_work_area();
        let plan = plan_stack(&enabled(50), &work, &windows(1)).unwrap();
        let r = plan.placements[0].rect;
        assert_eq!(r.width(), 960);
        assert_eq!(r.left, 480);
        assert_eq!(r.right, 1440);
        assert_eq!(r.left - work.rect.left, work.rect.right - r.right);
    }

    #[test]
    fn two_windows_anchor_left_and_right() {
        let work = primary_work_area();
        let plan = plan_stack(&enabled(50), &work, &windows(2)).unwrap();
        assert_eq!(plan.placements[0].rect.left, work.rect.left);
        assert_eq!(plan.placements[1].rect.right, work.rect.right);
    }
}
