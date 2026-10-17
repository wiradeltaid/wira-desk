//! Pure eligibility policy facade.
//! Re-exports from `wiradesk_core::cycling::eligibility`.

pub use wiradesk_core::cycling::eligibility::evaluate_facts;

use super::{ActiveContext, Candidate, Eligibility, EligibilityPolicy};

pub struct WindowEligibility;

impl EligibilityPolicy for WindowEligibility {
    fn evaluate(&self, active: &ActiveContext, candidate: &Candidate) -> Eligibility {
        evaluate_facts(&active.identity, &candidate.facts)
    }
}

#[cfg(test)]
mod tests {
    use super::super::fixtures::*;
    use super::super::*;
    use super::*;

    #[test]
    fn reproduces_every_frozen_fixture() {
        let active = active(1);
        for (label, facts, expected) in expected_decisions() {
            let candidate = Candidate {
                z_index: 0,
                facts: facts.clone(),
            };
            assert_eq!(
                WindowEligibility.evaluate(&active, &candidate),
                expected,
                "production policy drifted from frozen fixture: {label}"
            );
        }
    }

    #[test]
    fn agrees_with_reference_policy_on_every_fixture() {
        let active = active(1);
        for (label, facts, _) in expected_decisions() {
            let candidate = Candidate {
                z_index: 0,
                facts: facts.clone(),
            };
            assert_eq!(
                WindowEligibility.evaluate(&active, &candidate),
                ReferencePolicy.evaluate(&active, &candidate),
                "production and reference policy disagree on: {label}"
            );
        }
    }

    // --- base decisions ---------------------------------------

    #[test]
    fn visible_non_iconic_top_level_window_is_eligible() {
        assert_eq!(
            evaluate_facts(&identity(HOST_EXE), &normal(1)),
            Eligibility::Eligible
        );
    }

    #[test]
    fn hidden_is_excluded() {
        let facts = WindowFacts {
            visible: false,
            ..normal(1)
        };
        assert_eq!(
            evaluate_facts(&identity(HOST_EXE), &facts),
            Eligibility::Excluded(ExclusionReason::Hidden)
        );
    }

    #[test]
    fn cloaked_is_excluded_even_though_it_reports_visible() {
        let facts = WindowFacts {
            visible: true,
            cloaked: true,
            class_name: "ApplicationFrameWindow".to_string(),
            ..normal(1)
        };
        assert_eq!(
            evaluate_facts(&identity(HOST_EXE), &facts),
            Eligibility::Excluded(ExclusionReason::Cloaked)
        );
    }

    #[test]
    fn cloaked_outranks_iconic_and_tool_window() {
        let facts = WindowFacts {
            cloaked: true,
            iconic: true,
            tool_window: true,
            ..normal(1)
        };
        assert_eq!(
            evaluate_facts(&identity(HOST_EXE), &facts),
            Eligibility::Excluded(ExclusionReason::Cloaked)
        );
    }

    #[test]
    fn hidden_outranks_cloaked() {
        let facts = WindowFacts {
            visible: false,
            cloaked: true,
            ..normal(1)
        };
        assert_eq!(
            evaluate_facts(&identity(HOST_EXE), &facts),
            Eligibility::Excluded(ExclusionReason::Hidden)
        );
    }

    #[test]
    fn minimized_is_excluded() {
        let facts = WindowFacts {
            iconic: true,
            ..normal(1)
        };
        assert_eq!(
            evaluate_facts(&identity(HOST_EXE), &facts),
            Eligibility::Excluded(ExclusionReason::Iconic)
        );
    }

    #[test]
    fn tool_window_is_excluded() {
        let facts = WindowFacts {
            tool_window: true,
            ..normal(1)
        };
        assert_eq!(
            evaluate_facts(&identity(HOST_EXE), &facts),
            Eligibility::Excluded(ExclusionReason::ToolWindow)
        );
    }

    #[test]
    fn every_shell_overlay_class_is_excluded() {
        for class in SHELL_SURFACE_CLASSES {
            assert_eq!(
                evaluate_facts(&identity(HOST_EXE), &with_class(1, class)),
                Eligibility::Excluded(ExclusionReason::ShellSurface),
                "shell class not excluded: {class}"
            );
        }
    }

    #[test]
    fn shell_and_ghost_class_matching_is_case_insensitive() {
        assert_eq!(
            evaluate_facts(&identity(HOST_EXE), &with_class(1, "shell_traywnd")),
            Eligibility::Excluded(ExclusionReason::ShellSurface)
        );
        assert_eq!(
            evaluate_facts(&identity(HOST_EXE), &with_class(2, "GHOST")),
            Eligibility::Excluded(ExclusionReason::GhostWindow)
        );
    }

    // --- UX honesty for hung windows --------------------------

    #[test]
    fn synthetic_hung_application_window_remains_eligible() {
        let hung_real_window = WindowFacts {
            class_name: "Notepad".to_string(),
            ..normal(42)
        };
        assert_eq!(
            evaluate_facts(&identity(HOST_EXE), &hung_real_window),
            Eligibility::Eligible
        );

        let ghost_surrogate = with_class(43, CLASS_GHOST);
        assert_eq!(
            evaluate_facts(&identity(HOST_EXE), &ghost_surrogate),
            Eligibility::Excluded(ExclusionReason::GhostWindow)
        );
    }

    #[test]
    fn contract_exposes_no_responsiveness_input() {
        let responsive = normal(1);
        let hung = normal(1);
        assert_eq!(responsive, hung);
        assert_eq!(
            evaluate_facts(&identity(HOST_EXE), &responsive),
            evaluate_facts(&identity(HOST_EXE), &hung)
        );
    }

    // --- exclusion-fact combinations --------------------------

    #[test]
    fn ghost_outranks_every_other_exclusion() {
        let facts = WindowFacts {
            visible: false,
            iconic: true,
            tool_window: true,
            identity: AppIdentity::Unavailable,
            ..with_class(1, CLASS_GHOST)
        };
        assert_eq!(
            evaluate_facts(&identity(HOST_EXE), &facts),
            Eligibility::Excluded(ExclusionReason::GhostWindow)
        );
    }

    #[test]
    fn shell_surface_outranks_hidden_and_iconic() {
        let facts = WindowFacts {
            visible: false,
            iconic: true,
            ..with_class(1, CLASS_PROGMAN)
        };
        assert_eq!(
            evaluate_facts(&identity(HOST_EXE), &facts),
            Eligibility::Excluded(ExclusionReason::ShellSurface)
        );
    }

    #[test]
    fn hidden_outranks_iconic_and_tool_window() {
        let facts = WindowFacts {
            visible: false,
            iconic: true,
            tool_window: true,
            ..normal(1)
        };
        assert_eq!(
            evaluate_facts(&identity(HOST_EXE), &facts),
            Eligibility::Excluded(ExclusionReason::Hidden)
        );
    }

    #[test]
    fn iconic_outranks_tool_window() {
        let facts = WindowFacts {
            iconic: true,
            tool_window: true,
            ..normal(1)
        };
        assert_eq!(
            evaluate_facts(&identity(HOST_EXE), &facts),
            Eligibility::Excluded(ExclusionReason::Iconic)
        );
    }

    #[test]
    fn unavailable_identity_outranks_different_application() {
        let facts = WindowFacts {
            identity: AppIdentity::Unavailable,
            ..normal(1)
        };
        assert_eq!(
            evaluate_facts(&identity(HOST_EXE), &facts),
            Eligibility::Excluded(ExclusionReason::UnavailableIdentity)
        );
    }

    #[test]
    fn decisions_are_deterministic_across_repeated_evaluation() {
        let active = identity(HOST_EXE);
        for (label, facts, expected) in expected_decisions() {
            for _ in 0..8 {
                assert_eq!(
                    evaluate_facts(&active, &facts),
                    expected,
                    "non-deterministic decision for: {label}"
                );
            }
        }
    }

    // --- identity scoping -------------------------------------

    #[test]
    fn different_application_is_excluded_even_when_otherwise_valid() {
        let facts = WindowFacts {
            identity: identity(OTHER_EXE),
            ..normal(1)
        };
        assert_eq!(
            evaluate_facts(&identity(HOST_EXE), &facts),
            Eligibility::Excluded(ExclusionReason::DifferentApplication)
        );
    }

    #[test]
    fn same_executable_across_processes_stays_eligible() {
        let facts = WindowFacts {
            identity: AppIdentity::from_process_path(Some(r"D:\elsewhere\NOTEPAD.EXE")),
            ..normal(1)
        };
        assert_eq!(
            evaluate_facts(&identity(HOST_EXE), &facts),
            Eligibility::Eligible
        );
    }

    #[test]
    fn popup_host_surface_is_excluded() {
        assert_eq!(
            evaluate_facts(&identity(HOST_EXE), &with_class(1, CLASS_POPUP_HOST)),
            Eligibility::Excluded(ExclusionReason::HelperSurface)
        );
    }

    #[test]
    fn xaml_windowed_popup_class_is_excluded() {
        assert_eq!(
            evaluate_facts(
                &identity(HOST_EXE),
                &with_class(1, CLASS_XAML_WINDOWED_POPUP)
            ),
            Eligibility::Excluded(ExclusionReason::HelperSurface)
        );
    }

    #[test]
    fn popup_window_site_bridge_is_excluded() {
        assert_eq!(
            evaluate_facts(
                &identity(HOST_EXE),
                &with_class(1, CLASS_POPUP_WINDOW_SITE_BRIDGE)
            ),
            Eligibility::Excluded(ExclusionReason::HelperSurface)
        );
    }

    #[test]
    fn winui_notepad_popup_bridge_profile_is_excluded() {
        let notepad_identity = AppIdentity::Executable("notepad.exe".to_string());
        let bridge_facts = WindowFacts {
            class_name: CLASS_POPUP_WINDOW_SITE_BRIDGE.to_string(),
            is_owned: true,
            has_nonzero_extent: true,
            has_title: true,
            identity: notepad_identity.clone(),
            ..normal(1)
        };
        assert_eq!(
            evaluate_facts(&notepad_identity, &bridge_facts),
            Eligibility::Excluded(ExclusionReason::HelperSurface)
        );
    }

    #[test]
    fn real_notepad_window_remains_eligible() {
        let notepad_identity = AppIdentity::Executable("notepad.exe".to_string());
        let real_notepad_facts = WindowFacts {
            class_name: "Notepad".to_string(),
            is_owned: false,
            has_nonzero_extent: true,
            has_title: true,
            identity: notepad_identity.clone(),
            ..normal(1)
        };
        assert_eq!(
            evaluate_facts(&notepad_identity, &real_notepad_facts),
            Eligibility::Eligible
        );
    }

    #[test]
    fn empty_title_window_is_still_eligible() {
        let facts = WindowFacts {
            has_title: false,
            ..normal(1)
        };
        assert_eq!(
            evaluate_facts(&identity(HOST_EXE), &facts),
            Eligibility::Eligible
        );
    }

    #[test]
    fn zero_extent_window_is_still_eligible() {
        let facts = WindowFacts {
            has_nonzero_extent: false,
            ..normal(1)
        };
        assert_eq!(
            evaluate_facts(&identity(HOST_EXE), &facts),
            Eligibility::Eligible
        );
    }

    #[test]
    fn owned_window_is_still_eligible() {
        let facts = WindowFacts {
            is_owned: true,
            ..normal(1)
        };
        assert_eq!(
            evaluate_facts(&identity(HOST_EXE), &facts),
            Eligibility::Eligible
        );
    }

    #[test]
    fn unavailable_active_identity_yields_no_eligible_window() {
        let unknown_active = AppIdentity::Unavailable;
        assert_eq!(
            evaluate_facts(&unknown_active, &normal(1)),
            Eligibility::Excluded(ExclusionReason::DifferentApplication)
        );
    }
}
