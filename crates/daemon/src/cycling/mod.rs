//! Frozen cycling contract — Worker-domain only.
//!
//! Re-exports pure cycling domain vocabulary from `wiradesk_core::cycling`
//! and retains daemon-specific runtime traits (`CandidateSource`, `EligibilityPolicy`,
//! `Activator`, `run_cycle`) and test fixtures.

#![allow(dead_code)]

pub mod activation;
pub mod eligibility;
pub mod selection;
pub mod source;

pub use wiradesk_core::cycling::*;

/// Supplies one fresh top-to-bottom Z-order snapshot.
pub trait CandidateSource {
    fn snapshot(&self) -> Vec<Candidate>;
}

/// Pure eligibility policy.
pub trait EligibilityPolicy {
    fn evaluate(&self, active: &ActiveContext, candidate: &Candidate) -> Eligibility;
}

/// Performs one activation attempt.
pub trait Activator {
    fn activate(&mut self, target: WindowId) -> ActivationOutcome;
}

/// Drive one deterministic cycle pass.
/// Attempts each eligible target **at most once**, in wrap order, and stops at
/// the first activation.
pub fn run_cycle<S, P, A>(
    source: &S,
    policy: &P,
    activator: &mut A,
    active: &ActiveContext,
) -> CycleOutcome
where
    S: CandidateSource + ?Sized,
    P: EligibilityPolicy + ?Sized,
    A: Activator + ?Sized,
{
    let candidates = source.snapshot();

    let eligible: Vec<WindowId> = candidates
        .iter()
        .filter(|c| policy.evaluate(active, c).is_eligible())
        .map(|c| c.facts.window)
        .collect();

    if eligible.is_empty() {
        return CycleOutcome::NoEligibleTarget;
    }

    for target in cycle_order(&candidates, active) {
        if !eligible.contains(&target) {
            continue;
        }
        if activator.activate(target) == ActivationOutcome::Activated {
            return CycleOutcome::Activated(target);
        }
    }

    CycleOutcome::Exhausted
}

/// Frozen fixtures and expected decisions.
#[cfg(test)]
pub mod fixtures {
    use super::*;

    pub const HOST_EXE: &str = "notepad.exe";
    pub const OTHER_EXE: &str = "explorer.exe";

    pub fn identity(exe: &str) -> AppIdentity {
        AppIdentity::Executable(exe.to_ascii_lowercase())
    }

    pub fn active(window: isize) -> ActiveContext {
        ActiveContext {
            foreground: WindowId(window),
            identity: identity(HOST_EXE),
        }
    }

    /// A plain visible top-level window of the active application.
    pub fn normal(window: isize) -> WindowFacts {
        WindowFacts {
            window: WindowId(window),
            visible: true,
            cloaked: false,
            iconic: false,
            tool_window: false,
            class_name: "Notepad".to_string(),
            identity: identity(HOST_EXE),
            has_title: true,
            has_nonzero_extent: true,
            is_owned: false,
        }
    }

    pub fn with_class(window: isize, class_name: &str) -> WindowFacts {
        WindowFacts {
            class_name: class_name.to_string(),
            ..normal(window)
        }
    }

    pub fn ordered(facts: Vec<WindowFacts>) -> Vec<Candidate> {
        facts
            .into_iter()
            .enumerate()
            .map(|(z_index, facts)| Candidate { z_index, facts })
            .collect()
    }

    /// The frozen eligibility vocabulary. Each entry is
    /// `(label, facts, expected decision)`.
    pub fn expected_decisions() -> Vec<(&'static str, WindowFacts, Eligibility)> {
        vec![
            ("visible top-level", normal(1), Eligibility::Eligible),
            (
                "not responding (no probe)",
                normal(2),
                Eligibility::Eligible,
            ),
            (
                "hidden",
                WindowFacts {
                    visible: false,
                    ..normal(3)
                },
                Eligibility::Excluded(ExclusionReason::Hidden),
            ),
            (
                "cloaked but styled visible",
                WindowFacts {
                    visible: true,
                    cloaked: true,
                    ..normal(13)
                },
                Eligibility::Excluded(ExclusionReason::Cloaked),
            ),
            (
                "iconic",
                WindowFacts {
                    iconic: true,
                    ..normal(4)
                },
                Eligibility::Excluded(ExclusionReason::Iconic),
            ),
            (
                "WS_EX_TOOLWINDOW",
                WindowFacts {
                    tool_window: true,
                    ..normal(5)
                },
                Eligibility::Excluded(ExclusionReason::ToolWindow),
            ),
            (
                CLASS_GHOST,
                with_class(6, CLASS_GHOST),
                Eligibility::Excluded(ExclusionReason::GhostWindow),
            ),
            (
                CLASS_SHELL_TRAY,
                with_class(7, CLASS_SHELL_TRAY),
                Eligibility::Excluded(ExclusionReason::ShellSurface),
            ),
            (
                CLASS_SHELL_SECONDARY_TRAY,
                with_class(8, CLASS_SHELL_SECONDARY_TRAY),
                Eligibility::Excluded(ExclusionReason::ShellSurface),
            ),
            (
                CLASS_PROGMAN,
                with_class(9, CLASS_PROGMAN),
                Eligibility::Excluded(ExclusionReason::ShellSurface),
            ),
            (
                CLASS_WORKERW,
                with_class(10, CLASS_WORKERW),
                Eligibility::Excluded(ExclusionReason::ShellSurface),
            ),
            (
                CLASS_POPUP_HOST,
                with_class(14, CLASS_POPUP_HOST),
                Eligibility::Excluded(ExclusionReason::HelperSurface),
            ),
            (
                CLASS_XAML_WINDOWED_POPUP,
                with_class(15, CLASS_XAML_WINDOWED_POPUP),
                Eligibility::Excluded(ExclusionReason::HelperSurface),
            ),
            (
                CLASS_POPUP_WINDOW_SITE_BRIDGE,
                with_class(16, CLASS_POPUP_WINDOW_SITE_BRIDGE),
                Eligibility::Excluded(ExclusionReason::HelperSurface),
            ),
            (
                "different application",
                WindowFacts {
                    identity: identity(OTHER_EXE),
                    ..normal(11)
                },
                Eligibility::Excluded(ExclusionReason::DifferentApplication),
            ),
            (
                "unavailable identity",
                WindowFacts {
                    identity: AppIdentity::Unavailable,
                    ..normal(12)
                },
                Eligibility::Excluded(ExclusionReason::UnavailableIdentity),
            ),
        ]
    }

    /// Reference policy encoding the frozen decisions.
    pub struct ReferencePolicy;

    impl EligibilityPolicy for ReferencePolicy {
        fn evaluate(&self, active: &ActiveContext, candidate: &Candidate) -> Eligibility {
            let f = &candidate.facts;
            if f.is_ghost() {
                return Eligibility::Excluded(ExclusionReason::GhostWindow);
            }
            if f.is_shell_surface() {
                return Eligibility::Excluded(ExclusionReason::ShellSurface);
            }
            if f.is_helper_surface() {
                return Eligibility::Excluded(ExclusionReason::HelperSurface);
            }
            if !f.visible {
                return Eligibility::Excluded(ExclusionReason::Hidden);
            }
            if f.cloaked {
                return Eligibility::Excluded(ExclusionReason::Cloaked);
            }
            if f.iconic {
                return Eligibility::Excluded(ExclusionReason::Iconic);
            }
            if f.tool_window {
                return Eligibility::Excluded(ExclusionReason::ToolWindow);
            }
            if !f.identity.is_available() {
                return Eligibility::Excluded(ExclusionReason::UnavailableIdentity);
            }
            if !f.identity.same_application(&active.identity) {
                return Eligibility::Excluded(ExclusionReason::DifferentApplication);
            }
            Eligibility::Eligible
        }
    }

    /// Injected snapshot source — no Win32, no live desktop.
    pub struct StaticSource(pub Vec<Candidate>);

    impl CandidateSource for StaticSource {
        fn snapshot(&self) -> Vec<Candidate> {
            self.0.clone()
        }
    }

    /// Fake activator replaying scripted outcomes and recording every attempt.
    pub struct ScriptedActivator {
        pub scripted: Vec<(WindowId, ActivationOutcome)>,
        pub default: ActivationOutcome,
        pub attempts: Vec<WindowId>,
    }

    impl ScriptedActivator {
        pub fn always(outcome: ActivationOutcome) -> Self {
            ScriptedActivator {
                scripted: Vec::new(),
                default: outcome,
                attempts: Vec::new(),
            }
        }

        pub fn scripted(scripted: Vec<(WindowId, ActivationOutcome)>) -> Self {
            ScriptedActivator {
                scripted,
                default: ActivationOutcome::Activated,
                attempts: Vec::new(),
            }
        }
    }

    impl Activator for ScriptedActivator {
        fn activate(&mut self, target: WindowId) -> ActivationOutcome {
            self.attempts.push(target);
            self.scripted
                .iter()
                .find(|(w, _)| *w == target)
                .map(|(_, o)| *o)
                .unwrap_or(self.default)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::fixtures::*;
    use super::*;

    // --- executable normalization -----------------------------

    #[test]
    fn normalizes_basename_case_insensitively() {
        assert_eq!(
            normalize_executable(r"C:\Windows\System32\NOTEPAD.EXE"),
            Some("notepad.exe".to_string())
        );
    }

    #[test]
    fn normalizes_forward_slash_paths() {
        assert_eq!(
            normalize_executable("C:/Program Files/App/App.Exe"),
            Some("app.exe".to_string())
        );
    }

    #[test]
    fn bare_basename_is_accepted() {
        assert_eq!(
            normalize_executable("Code.exe"),
            Some("code.exe".to_string())
        );
    }

    #[test]
    fn empty_or_trailing_separator_yields_unavailable() {
        assert_eq!(normalize_executable(""), None);
        assert_eq!(normalize_executable(r"C:\Windows\"), None);
        assert_eq!(
            AppIdentity::from_process_path(None),
            AppIdentity::Unavailable
        );
        assert_eq!(
            AppIdentity::from_process_path(Some("")),
            AppIdentity::Unavailable
        );
    }

    #[test]
    fn same_executable_different_pid_is_same_application() {
        let a = AppIdentity::from_process_path(Some(r"C:\A\notepad.exe"));
        let b = AppIdentity::from_process_path(Some(r"D:\B\NOTEPAD.EXE"));
        assert!(a.same_application(&b));
    }

    #[test]
    fn unavailable_identity_never_matches() {
        let unknown = AppIdentity::Unavailable;
        assert!(!unknown.same_application(&AppIdentity::Unavailable));
        assert!(!unknown.same_application(&identity(HOST_EXE)));
        assert!(!identity(HOST_EXE).same_application(&unknown));
    }

    // --- frozen eligibility vocabulary ------------------------

    #[test]
    fn frozen_fixtures_match_expected_decisions() {
        let policy = ReferencePolicy;
        let active = active(1);
        for (label, facts, expected) in expected_decisions() {
            let candidate = Candidate {
                z_index: 0,
                facts: facts.clone(),
            };
            assert_eq!(
                policy.evaluate(&active, &candidate),
                expected,
                "frozen decision drifted for fixture: {label}"
            );
        }
    }

    #[test]
    fn every_named_shell_class_is_covered() {
        let labels: Vec<&str> = expected_decisions()
            .into_iter()
            .map(|(l, _, _)| l)
            .collect();
        for class in SHELL_SURFACE_CLASSES.iter().chain([&CLASS_GHOST]) {
            assert!(labels.contains(class), "fixture missing for class {class}");
        }
    }

    #[test]
    fn every_helper_surface_class_has_a_fixture() {
        let labels: Vec<&str> = expected_decisions()
            .into_iter()
            .map(|(l, _, _)| l)
            .collect();
        for class in HELPER_SURFACE_CLASSES {
            assert!(labels.contains(class), "fixture missing for class {class}");
        }
    }

    // --- ordering and wrap -----------------------

    #[test]
    fn cycle_order_is_least_recently_used_first() {
        let candidates = ordered(vec![normal(1), normal(2), normal(3)]);
        let order = cycle_order(&candidates, &active(2));
        assert_eq!(order, vec![WindowId(1), WindowId(3)]);
    }

    #[test]
    fn repeated_cycles_reach_every_window() {
        let mut z = vec![1isize, 2, 3];
        let mut visited = std::collections::HashSet::new();
        let mut current = 1isize;
        for _ in 0..6 {
            let candidates = ordered(z.iter().map(|w| normal(*w)).collect());
            let next = cycle_order(&candidates, &active(current))[0];
            visited.insert(next.0);
            current = next.0;
            z.retain(|w| *w != current);
            z.insert(0, current);
        }
        assert_eq!(
            visited.len(),
            3,
            "cycling never reached every window: {visited:?}"
        );
    }

    #[test]
    fn cycle_order_from_topmost_starts_at_the_bottom() {
        let candidates = ordered(vec![normal(1), normal(2), normal(3)]);
        let order = cycle_order(&candidates, &active(1));
        assert_eq!(order, vec![WindowId(3), WindowId(2)]);
    }

    #[test]
    fn cycle_order_when_active_absent_starts_at_the_bottom() {
        let candidates = ordered(vec![normal(1), normal(2)]);
        let order = cycle_order(&candidates, &active(99));
        assert_eq!(order, vec![WindowId(2), WindowId(1)]);
    }

    #[test]
    fn snapshot_order_is_top_to_bottom() {
        let candidates = ordered(vec![normal(7), normal(8)]);
        assert_eq!(candidates[0].z_index, 0);
        assert_eq!(candidates[0].facts.window, WindowId(7));
        assert_eq!(candidates[1].z_index, 1);
    }

    // --- harness over the full pass ---------------------------

    #[test]
    fn activates_next_window_of_same_application() {
        let source = StaticSource(ordered(vec![normal(1), normal(2), normal(3)]));
        let mut activator = ScriptedActivator::always(ActivationOutcome::Activated);
        let outcome = run_cycle(&source, &ReferencePolicy, &mut activator, &active(1));
        assert_eq!(outcome, CycleOutcome::Activated(WindowId(3)));
        assert_eq!(activator.attempts, vec![WindowId(3)]);
    }

    #[test]
    fn excluded_windows_are_skipped() {
        let source = StaticSource(ordered(vec![
            normal(1),
            with_class(2, CLASS_SHELL_TRAY),
            WindowFacts {
                iconic: true,
                ..normal(3)
            },
            normal(4),
        ]));
        let mut activator = ScriptedActivator::always(ActivationOutcome::Activated);
        let outcome = run_cycle(&source, &ReferencePolicy, &mut activator, &active(1));
        assert_eq!(outcome, CycleOutcome::Activated(WindowId(4)));
        assert_eq!(activator.attempts, vec![WindowId(4)]);
    }

    #[test]
    fn invalid_target_continues_to_next_candidate() {
        let source = StaticSource(ordered(vec![normal(1), normal(2), normal(3)]));
        let mut activator = ScriptedActivator::scripted(vec![
            (WindowId(3), ActivationOutcome::InvalidTarget),
            (WindowId(2), ActivationOutcome::Activated),
        ]);
        let outcome = run_cycle(&source, &ReferencePolicy, &mut activator, &active(1));
        assert_eq!(outcome, CycleOutcome::Activated(WindowId(2)));
        assert_eq!(activator.attempts, vec![WindowId(3), WindowId(2)]);
    }

    #[test]
    fn failed_activation_also_continues() {
        let source = StaticSource(ordered(vec![normal(1), normal(2), normal(3)]));
        let mut activator = ScriptedActivator::scripted(vec![
            (WindowId(3), ActivationOutcome::Failed),
            (WindowId(2), ActivationOutcome::Activated),
        ]);
        let outcome = run_cycle(&source, &ReferencePolicy, &mut activator, &active(1));
        assert_eq!(outcome, CycleOutcome::Activated(WindowId(2)));
    }

    #[test]
    fn each_eligible_target_attempted_at_most_once_then_terminates() {
        let source = StaticSource(ordered(vec![normal(1), normal(2), normal(3)]));
        let mut activator = ScriptedActivator::always(ActivationOutcome::InvalidTarget);
        let outcome = run_cycle(&source, &ReferencePolicy, &mut activator, &active(1));
        assert_eq!(outcome, CycleOutcome::Exhausted);
        assert_eq!(activator.attempts, vec![WindowId(3), WindowId(2)]);
    }

    #[test]
    fn no_eligible_target_when_all_excluded() {
        let source = StaticSource(ordered(vec![
            with_class(1, CLASS_PROGMAN),
            with_class(2, CLASS_WORKERW),
        ]));
        let mut activator = ScriptedActivator::always(ActivationOutcome::Activated);
        let outcome = run_cycle(&source, &ReferencePolicy, &mut activator, &active(9));
        assert_eq!(outcome, CycleOutcome::NoEligibleTarget);
        assert!(activator.attempts.is_empty());
    }

    #[test]
    fn single_window_application_has_nothing_to_cycle_to() {
        let source = StaticSource(ordered(vec![normal(1)]));
        let mut activator = ScriptedActivator::always(ActivationOutcome::Activated);
        let outcome = run_cycle(&source, &ReferencePolicy, &mut activator, &active(1));
        assert_eq!(outcome, CycleOutcome::Exhausted);
        assert!(activator.attempts.is_empty());
    }

    #[test]
    fn windows_of_other_applications_are_not_targets() {
        let source = StaticSource(ordered(vec![
            normal(1),
            WindowFacts {
                identity: identity(OTHER_EXE),
                ..normal(2)
            },
            normal(3),
        ]));
        let mut activator = ScriptedActivator::always(ActivationOutcome::Activated);
        let outcome = run_cycle(&source, &ReferencePolicy, &mut activator, &active(1));
        assert_eq!(outcome, CycleOutcome::Activated(WindowId(3)));
    }

    #[test]
    fn multi_process_same_executable_groups_together() {
        let source = StaticSource(ordered(vec![
            normal(1),
            WindowFacts {
                identity: AppIdentity::from_process_path(Some(r"D:\other\NOTEPAD.EXE")),
                ..normal(2)
            },
        ]));
        let mut activator = ScriptedActivator::always(ActivationOutcome::Activated);
        let outcome = run_cycle(&source, &ReferencePolicy, &mut activator, &active(1));
        assert_eq!(outcome, CycleOutcome::Activated(WindowId(2)));
    }

    #[test]
    fn harness_needs_no_hook_or_live_desktop() {
        let source = StaticSource(ordered(vec![normal(1), normal(2)]));
        let mut activator = ScriptedActivator::always(ActivationOutcome::Activated);
        let _ = run_cycle(&source, &ReferencePolicy, &mut activator, &active(1));
    }

    #[test]
    fn backward_cycle_order_is_the_forward_rotation_unreversed() {
        let candidates = ordered(vec![normal(1), normal(2), normal(3), normal(4)]);
        let active = active(2);

        let forward = cycle_order_directed(&candidates, &active, Direction::Forward);
        let backward = cycle_order_directed(&candidates, &active, Direction::Backward);

        let mut forward_reversed = forward.clone();
        forward_reversed.reverse();
        assert_eq!(backward, forward_reversed);
        assert_eq!(backward, vec![WindowId(3), WindowId(4), WindowId(1)]);
        assert_eq!(forward, vec![WindowId(1), WindowId(4), WindowId(3)]);
    }

    #[test]
    fn backward_blind_cycle_traverses_all_eligible_windows() {
        let mut z = vec![1isize, 2, 3, 4];
        let mut visited = Vec::new();
        let mut current = 1isize;
        let mut session = BackwardCycleSession::new();

        for i in 0..6 {
            let candidates = ordered(z.iter().map(|w| normal(*w)).collect());
            let tick = 1000 + i * 200;
            let targets = session.cycle_order(&candidates, &active(current), tick);
            assert!(!targets.is_empty(), "targets should not be empty");
            let next = targets[0];
            session.record_activation(next, tick);
            visited.push(next.0);
            current = next.0;
            z.retain(|w| *w != current);
            z.insert(0, current);
        }

        assert_eq!(
            visited,
            vec![2, 3, 4, 1, 2, 3],
            "backward cycling must traverse all windows in reverse rather than oscillating"
        );
    }

    #[test]
    fn backward_cycle_session_resets_on_timeout() {
        let candidates = ordered(vec![normal(1), normal(2), normal(3)]);
        let mut session = BackwardCycleSession::new();

        let targets1 = session.cycle_order(&candidates, &active(1), 1000);
        assert_eq!(targets1[0], WindowId(2));
        session.record_activation(WindowId(2), 1000);

        let targets2 = session.cycle_order(
            &candidates,
            &active(2),
            1000 + BACKWARD_CYCLE_SESSION_TIMEOUT_MS + 100,
        );
        assert_eq!(targets2[0], WindowId(3));
    }

    #[test]
    fn backward_cycle_session_resets_on_focus_departure() {
        let candidates = ordered(vec![normal(1), normal(2), normal(3)]);
        let mut session = BackwardCycleSession::new();

        let targets1 = session.cycle_order(&candidates, &active(1), 1000);
        assert_eq!(targets1[0], WindowId(2));
        session.record_activation(WindowId(2), 1000);

        let targets2 = session.cycle_order(&candidates, &active(99), 1100);
        assert_eq!(targets2[0], WindowId(1));
    }

    #[test]
    fn backward_cycle_session_prunes_closed_window() {
        let candidates = ordered(vec![normal(1), normal(2), normal(3), normal(4)]);
        let mut session = BackwardCycleSession::new();

        let targets1 = session.cycle_order(&candidates, &active(1), 1000);
        assert_eq!(targets1[0], WindowId(2));
        session.record_activation(WindowId(2), 1000);

        let remaining_candidates = ordered(vec![normal(2), normal(1), normal(4)]);
        let targets2 = session.cycle_order(&remaining_candidates, &active(2), 1200);
        assert_eq!(targets2[0], WindowId(4));
        session.record_activation(WindowId(4), 1200);

        let targets3 = session.cycle_order(&remaining_candidates, &active(4), 1400);
        assert_eq!(targets3[0], WindowId(1));
    }
}
