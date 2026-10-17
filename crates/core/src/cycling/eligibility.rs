//! Pure eligibility policy. Owns no Win32 calls.

use super::{AppIdentity, Eligibility, ExclusionReason, WindowFacts};

/// Decide eligibility from captured facts alone.
pub fn evaluate_facts(active_identity: &AppIdentity, facts: &WindowFacts) -> Eligibility {
    if facts.is_ghost() {
        return Eligibility::Excluded(ExclusionReason::GhostWindow);
    }
    if facts.is_shell_surface() {
        return Eligibility::Excluded(ExclusionReason::ShellSurface);
    }
    if facts.is_helper_surface() {
        return Eligibility::Excluded(ExclusionReason::HelperSurface);
    }
    if !facts.visible {
        return Eligibility::Excluded(ExclusionReason::Hidden);
    }
    if facts.cloaked {
        return Eligibility::Excluded(ExclusionReason::Cloaked);
    }
    if facts.iconic {
        return Eligibility::Excluded(ExclusionReason::Iconic);
    }
    if facts.tool_window {
        return Eligibility::Excluded(ExclusionReason::ToolWindow);
    }
    if !facts.identity.is_available() {
        return Eligibility::Excluded(ExclusionReason::UnavailableIdentity);
    }
    if !facts.identity.same_application(active_identity) {
        return Eligibility::Excluded(ExclusionReason::DifferentApplication);
    }
    Eligibility::Eligible
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::window::WindowId;

    fn test_facts(window: WindowId, class: &str, app: &str) -> WindowFacts {
        WindowFacts {
            window,
            visible: true,
            cloaked: false,
            iconic: false,
            tool_window: false,
            class_name: class.to_owned(),
            identity: AppIdentity::Executable(app.to_owned()),
            has_title: true,
            has_nonzero_extent: true,
            is_owned: false,
        }
    }

    #[test]
    pub fn test_evaluate_facts() {
        let app = AppIdentity::Executable("test.exe".to_owned());
        let facts = test_facts(WindowId(1), "RegularClass", "test.exe");
        assert_eq!(evaluate_facts(&app, &facts), Eligibility::Eligible);

        let mut ghost_facts = facts.clone();
        ghost_facts.class_name = "Ghost".to_owned();
        assert_eq!(
            evaluate_facts(&app, &ghost_facts),
            Eligibility::Excluded(ExclusionReason::GhostWindow)
        );

        let diff_app = AppIdentity::Executable("other.exe".to_owned());
        assert_eq!(
            evaluate_facts(&diff_app, &facts),
            Eligibility::Excluded(ExclusionReason::DifferentApplication)
        );
    }
}
