//! Overlapping stack planning adapter.

#[allow(unused_imports)]
pub use wiradesk_core::arrangement::stack::*;

/// Compatibility facade accepting &LayoutConfig for existing Windows daemon callers.
pub fn plan_stack(
    layout: &shared::config::LayoutConfig,
    work: &wiradesk_core::arrangement::WorkArea,
    candidates: &[wiradesk_core::WindowId],
) -> wiradesk_core::arrangement::PlanResult {
    wiradesk_core::arrangement::stack::plan_stack(layout.stack_width_percent, work, candidates)
}

#[cfg(test)]
mod tests {
    use super::super::fixtures::*;
    use super::*;
    use shared::config::LayoutConfig;

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
}
