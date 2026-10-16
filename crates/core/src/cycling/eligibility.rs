//! Pure eligibility policy — sans-IO.

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
mod tests {
    use super::*;
    use crate::window::WindowId;

    fn sample_facts() -> WindowFacts {
        WindowFacts {
            window: WindowId(1),
            visible: true,
            cloaked: false,
            iconic: false,
            tool_window: false,
            class_name: "Notepad".to_string(),
            identity: AppIdentity::Executable("notepad.exe".to_string()),
            has_title: true,
            has_nonzero_extent: true,
            is_owned: false,
        }
    }

    #[test]
    fn eligible_when_normal_and_same_app() {
        let active = AppIdentity::Executable("notepad.exe".to_string());
        let facts = sample_facts();
        assert_eq!(evaluate_facts(&active, &facts), Eligibility::Eligible);
    }

    #[test]
    fn excluded_when_ghost() {
        let active = AppIdentity::Executable("notepad.exe".to_string());
        let mut facts = sample_facts();
        facts.class_name = "Ghost".to_string();
        assert_eq!(
            evaluate_facts(&active, &facts),
            Eligibility::Excluded(ExclusionReason::GhostWindow)
        );
    }

    #[test]
    fn excluded_when_hidden_or_cloaked() {
        let active = AppIdentity::Executable("notepad.exe".to_string());
        let mut facts = sample_facts();
        facts.visible = false;
        assert_eq!(
            evaluate_facts(&active, &facts),
            Eligibility::Excluded(ExclusionReason::Hidden)
        );

        let mut facts2 = sample_facts();
        facts2.cloaked = true;
        assert_eq!(
            evaluate_facts(&active, &facts2),
            Eligibility::Excluded(ExclusionReason::Cloaked)
        );
    }

    #[test]
    fn excluded_when_different_application() {
        let active = AppIdentity::Executable("calc.exe".to_string());
        let facts = sample_facts();
        assert_eq!(
            evaluate_facts(&active, &facts),
            Eligibility::Excluded(ExclusionReason::DifferentApplication)
        );
    }
}
