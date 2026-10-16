//! Stateless next-target selection — sans-IO.

use super::{cycle_order, ActiveContext, Candidate, SelectionResult};
use crate::window::WindowId;

/// Build the full attempt order: every eligible window except the active one,
/// starting immediately after the active window and wrapping at most once.
pub fn attempt_order(
    candidates: &[Candidate],
    eligible: &[WindowId],
    active: &ActiveContext,
) -> Vec<WindowId> {
    cycle_order(candidates, active)
        .into_iter()
        .filter(|w| *w != active.foreground && eligible.contains(w))
        .collect()
}

/// Choose the next target, or [`SelectionResult::NoCandidate`] for a no-op.
pub fn select_next(
    candidates: &[Candidate],
    eligible: &[WindowId],
    active: &ActiveContext,
) -> SelectionResult {
    match attempt_order(candidates, eligible, active).first() {
        Some(target) => SelectionResult::Target(*target),
        None => SelectionResult::NoCandidate,
    }
}

/// Advance past targets already tried in this pass.
pub fn select_after(
    candidates: &[Candidate],
    eligible: &[WindowId],
    active: &ActiveContext,
    tried: &[WindowId],
) -> SelectionResult {
    match attempt_order(candidates, eligible, active)
        .into_iter()
        .find(|w| !tried.contains(w))
    {
        Some(target) => SelectionResult::Target(target),
        None => SelectionResult::NoCandidate,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cycling::{AppIdentity, WindowFacts};

    fn make_candidate(id: isize, z: usize) -> Candidate {
        Candidate {
            z_index: z,
            facts: WindowFacts {
                window: WindowId(id),
                visible: true,
                cloaked: false,
                iconic: false,
                tool_window: false,
                class_name: "TestClass".to_string(),
                identity: AppIdentity::Executable("test.exe".to_string()),
                has_title: true,
                has_nonzero_extent: true,
                is_owned: false,
            },
        }
    }

    #[test]
    fn selects_the_least_recently_used_window() {
        let candidates = vec![
            make_candidate(1, 0),
            make_candidate(2, 1),
            make_candidate(3, 2),
        ];
        let eligible = vec![WindowId(1), WindowId(2), WindowId(3)];
        let active = ActiveContext {
            foreground: WindowId(1),
            identity: AppIdentity::Executable("test.exe".to_string()),
        };

        let result = select_next(&candidates, &eligible, &active);
        assert_eq!(result, SelectionResult::Target(WindowId(3)));
    }

    #[test]
    fn select_after_skips_tried_windows() {
        let candidates = vec![
            make_candidate(1, 0),
            make_candidate(2, 1),
            make_candidate(3, 2),
        ];
        let eligible = vec![WindowId(1), WindowId(2), WindowId(3)];
        let active = ActiveContext {
            foreground: WindowId(1),
            identity: AppIdentity::Executable("test.exe".to_string()),
        };

        let tried = vec![WindowId(3)];
        let result = select_after(&candidates, &eligible, &active, &tried);
        assert_eq!(result, SelectionResult::Target(WindowId(2)));
    }
}
