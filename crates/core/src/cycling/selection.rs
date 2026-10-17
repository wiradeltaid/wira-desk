//! Stateless next-target selection. Owns no Win32 calls.

use super::{cycle_order, ActiveContext, Candidate, SelectionResult, WindowId};

/// Build the full attempt order: every eligible window except the active one.
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
pub mod tests {
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
                class_name: "Class".to_owned(),
                identity: AppIdentity::Executable("app.exe".to_owned()),
                has_title: true,
                has_nonzero_extent: true,
                is_owned: false,
            },
        }
    }

    #[test]
    pub fn test_select_next() {
        let candidates = vec![
            make_candidate(1, 0),
            make_candidate(2, 1),
            make_candidate(3, 2),
        ];
        let eligible = vec![WindowId(1), WindowId(2), WindowId(3)];
        let active = ActiveContext {
            foreground: WindowId(1),
            identity: AppIdentity::Executable("app.exe".to_owned()),
        };

        // cycle_order reverses rotated list: 1 -> [3, 2]
        let next = select_next(&candidates, &eligible, &active);
        assert_eq!(next, SelectionResult::Target(WindowId(3)));
    }
}
