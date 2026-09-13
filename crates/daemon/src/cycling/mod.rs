//! Frozen cycling contract — Worker-domain only.
//!
//! This module is the single deterministic vocabulary shared by the parallel
//! lanes. It is intentionally free of Win32 calls so the harness runs
//! without a global hook or a live desktop.
//!
//! Module ownership:
//! - `cycling/source.rs` — live Z-order and identity discovery
//! - `cycling/eligibility.rs` — pure eligibility policy
//! - `cycling/selection.rs` — target selection
//! - `cycling/activation.rs` — foreground activation
//! - `cycling/mod.rs`, `worker.rs` — composition at convergence
//!
//! These types stay internal to the daemon Worker domain and deliberately do
//! **not** expand the cross-crate `shared` API.

// The contract is published ahead of its consumers; lanes 2.3–2.5 fill it in.
// Same precedent as the `log::warn` capability-only seam.
#![allow(dead_code)]

pub mod activation;
pub mod eligibility;
pub mod selection;
pub mod source;

/// Opaque window handle. Kept as a plain integer so fixtures need no Win32.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowId(pub isize);

/// Window classes that carry a frozen eligibility decision.
pub const CLASS_GHOST: &str = "Ghost";
pub const CLASS_SHELL_TRAY: &str = "Shell_TrayWnd";
pub const CLASS_SHELL_SECONDARY_TRAY: &str = "Shell_SecondaryTrayWnd";
pub const CLASS_PROGMAN: &str = "Progman";
pub const CLASS_WORKERW: &str = "WorkerW";
pub const CLASS_POPUP_HOST: &str = "PopupHost";
pub const CLASS_XAML_WINDOWED_POPUP: &str = "Xaml_WindowedPopupClass";
pub const CLASS_POPUP_WINDOW_SITE_BRIDGE: &str = "Microsoft.UI.Content.PopupWindowSiteBridge";

/// Shell surfaces excluded regardless of style bits.
pub const SHELL_SURFACE_CLASSES: &[&str] = &[
    CLASS_SHELL_TRAY,
    CLASS_SHELL_SECONDARY_TRAY,
    CLASS_PROGMAN,
    CLASS_WORKERW,
];

/// Helper surfaces (WinUI/XAML popup hosts) excluded regardless of style bits.
pub const HELPER_SURFACE_CLASSES: &[&str] = &[
    CLASS_POPUP_HOST,
    CLASS_XAML_WINDOWED_POPUP,
    CLASS_POPUP_WINDOW_SITE_BRIDGE,
];

/// Application identity.
/// Identity is the case-insensitive executable basename. A PID may be used to
/// *query* the process but never as the primary same-application key, so it is
/// absent from this type by construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppIdentity {
    /// Normalized lowercase executable basename, e.g. `notepad.exe`.
    Executable(String),
    /// The process vanished or denied access. Never matches anything.
    Unavailable,
}

impl AppIdentity {
    /// Build an identity from a full process image path.
    /// Returns [`AppIdentity::Unavailable`] when no usable basename exists, so
    /// a vanished or inaccessible process degrades instead of crashing.
    pub fn from_process_path(path: Option<&str>) -> Self {
        match path.and_then(normalize_executable) {
            Some(name) => AppIdentity::Executable(name),
            None => AppIdentity::Unavailable,
        }
    }

    /// Same-application test.
    /// [`AppIdentity::Unavailable`] never matches — not even another
    /// `Unavailable` — because two unknown processes are not known to be the
    /// same application. Do not substitute `==` for this.
    pub fn same_application(&self, other: &AppIdentity) -> bool {
        match (self, other) {
            (AppIdentity::Executable(a), AppIdentity::Executable(b)) => a == b,
            _ => false,
        }
    }

    pub fn is_available(&self) -> bool {
        matches!(self, AppIdentity::Executable(_))
    }
}

/// Normalize a process image path to a case-insensitive executable basename.
/// Accepts both separators because `QueryFullProcessImageNameW` may return
/// either depending on the path format requested.
pub fn normalize_executable(path: &str) -> Option<String> {
    let basename = path.rsplit(['\\', '/']).next().unwrap_or(path).trim();
    if basename.is_empty() {
        return None;
    }
    Some(basename.to_ascii_lowercase())
}

/// Immutable facts captured for one window during one snapshot.
/// Responsiveness is deliberately absent: a real application window marked Not
/// Responding stays eligible, and the contract must not invite a probe
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowFacts {
    pub window: WindowId,
    pub visible: bool,
    /// Hidden by the Desktop Window Manager rather than by its style bits.
    /// This is a **separate** fact from `visible`, because `IsWindowVisible`
    /// answers `true` for a cloaked window: the `WS_VISIBLE` bit really is set,
    /// the compositor simply never draws it. Suspended UWP surfaces and windows
    /// belonging to another virtual desktop are cloaked this way, so without
    /// this fact they look like ordinary windows to the policy.
    pub cloaked: bool,
    pub iconic: bool,
    pub tool_window: bool,
    pub class_name: String,
    pub identity: AppIdentity,
    pub has_title: bool,
    pub has_nonzero_extent: bool,
    pub is_owned: bool,
}

impl WindowFacts {
    pub fn is_shell_surface(&self) -> bool {
        SHELL_SURFACE_CLASSES
            .iter()
            .any(|c| c.eq_ignore_ascii_case(&self.class_name))
    }

    pub fn is_helper_surface(&self) -> bool {
        HELPER_SURFACE_CLASSES
            .iter()
            .any(|c| c.eq_ignore_ascii_case(&self.class_name))
    }

    pub fn is_ghost(&self) -> bool {
        CLASS_GHOST.eq_ignore_ascii_case(&self.class_name)
    }
}

/// One ordered candidate from a single fresh Z-order snapshot.
/// `z_index` is `0` for the topmost window and increases downward.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    pub z_index: usize,
    pub facts: WindowFacts,
}

/// Active-window context, sampled exactly once per accepted `Command::Cycle`.
/// The Worker must call `GetForegroundWindow` one time at the start of the
/// command and carry this value through the whole pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveContext {
    pub foreground: WindowId,
    pub identity: AppIdentity,
}

/// Why a candidate was excluded from the cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExclusionReason {
    Hidden,
    Cloaked,
    Iconic,
    ToolWindow,
    GhostWindow,
    ShellSurface,
    HelperSurface,
    DifferentApplication,
    UnavailableIdentity,
}

/// Eligibility decision for one candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Eligibility {
    Eligible,
    Excluded(ExclusionReason),
}

impl Eligibility {
    pub fn is_eligible(&self) -> bool {
        matches!(self, Eligibility::Eligible)
    }
}

/// Result of choosing the next target from an ordered eligible set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionResult {
    Target(WindowId),
    NoCandidate,
}

/// Result of one activation attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivationOutcome {
    /// The target is now foreground.
    Activated,
    /// The target vanished or is no longer valid; continue to the next target.
    InvalidTarget,
    /// The attempt failed for another reason; continue to the next target.
    Failed,
}

/// Terminal result of one cycle pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CycleOutcome {
    Activated(WindowId),
    /// Every eligible target was attempted once and none activated.
    Exhausted,
    /// No candidate survived eligibility.
    NoEligibleTarget,
}

/// Supplies one fresh top-to-bottom Z-order snapshot. Implemented by.
pub trait CandidateSource {
    fn snapshot(&self) -> Vec<Candidate>;
}

/// Pure eligibility policy. Implemented by.
pub trait EligibilityPolicy {
    fn evaluate(&self, active: &ActiveContext, candidate: &Candidate) -> Eligibility;
}

/// Performs one activation attempt. Implemented by.
pub trait Activator {
    fn activate(&mut self, target: WindowId) -> ActivationOutcome;
}

/// Order the candidates **least-recently-used first**.
/// The snapshot arrives in Z-order, topmost first, so the *last* entry is the
/// window the user touched longest ago. Walking from that end is what makes
/// repeated presses rotate through every window.
/// Taking the *first* entry after the active window — the obvious reading of
/// "next in Z-order" — does not work, because activating a window raises it to
/// the top. With three windows A, B, C that produces:
/// Direction of window cycling (SPEC-14-04).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Backward,
}

/// ```text
/// [A B C] active A -> pick B (B raised)
/// [B A C] active B -> pick A (A raised)
/// [A B C] active A -> pick B ... forever
/// ```
/// C is unreachable. Reversing gives `A -> C -> B -> A`, which visits all three.
/// For two windows the two orders are identical, which is why the bug only
/// appears from three windows onward.
pub fn cycle_order_directed(
    candidates: &[Candidate],
    active: &ActiveContext,
    direction: Direction,
) -> Vec<WindowId> {
    let ordered: Vec<WindowId> = candidates.iter().map(|c| c.facts.window).collect();
    let mut rotated: Vec<WindowId> = match ordered.iter().position(|w| *w == active.foreground) {
        Some(pos) => ordered[pos + 1..]
            .iter()
            .chain(ordered[..pos].iter())
            .copied()
            .collect(),
        None => ordered,
    };
    if direction == Direction::Forward {
        rotated.reverse();
    }
    rotated
}

pub fn cycle_order(candidates: &[Candidate], active: &ActiveContext) -> Vec<WindowId> {
    cycle_order_directed(candidates, active, Direction::Forward)
}

/// Blind backward cycling session timeout in milliseconds.
/// If consecutive backward cycle inputs arrive within this window without
/// intervening focus changes to other apps, navigation continues traversing
/// the same sequence instead of resetting to the top neighbor.
pub const BACKWARD_CYCLE_SESSION_TIMEOUT_MS: u64 = 2000;

/// Tracks the visitation sequence for blind backward cycling across consecutive
/// activations (DEF-19, SPEC-17).
///
/// In live Win32 activation, activating a target window raises it to the top of the
/// Z-order (remove-and-prepend), pushing the prior foreground to Z=1. A stateless
/// backward pick always selects Z=1, locking repeated backward cycling into a
/// two-window oscillation (A <-> B) instead of traversing all eligible windows.
///
/// `BackwardCycleSession` establishes a stable circular window traversal order upon
/// the first backward tap, and advances along that sequence while consecutive
/// backward cycle commands arrive within [`BACKWARD_CYCLE_SESSION_TIMEOUT_MS`] without
/// focus departure or interleaved forward cycling.
#[derive(Debug, Clone, Default)]
pub struct BackwardCycleSession {
    /// Ordered list of windows in the established traversal cycle.
    pub(crate) windows: Vec<WindowId>,
    /// Index in `windows` of the window activated on the last step.
    pub(crate) cursor: usize,
    /// Window activated on the last step, used to verify foreground continuity.
    pub(crate) last_activated: Option<WindowId>,
    /// Timestamp (tick_ms) of the last cycle action.
    pub(crate) last_tick_ms: u64,
}

impl BackwardCycleSession {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.windows.clear();
        self.cursor = 0;
        self.last_activated = None;
        self.last_tick_ms = 0;
    }

    /// Check whether this session is currently active and valid for the given active window and time.
    pub fn is_valid(&self, active_window: WindowId, now_ms: u64) -> bool {
        if self.windows.is_empty() {
            return false;
        }
        if self.last_activated != Some(active_window) {
            return false;
        }
        if self.last_tick_ms > 0
            && now_ms > 0
            && now_ms.saturating_sub(self.last_tick_ms) > BACKWARD_CYCLE_SESSION_TIMEOUT_MS
        {
            return false;
        }
        true
    }

    /// Produce the target attempt order for a backward cycle step.
    ///
    /// If the session is valid, it synchronizes `self.windows` against current `candidates`
    /// and advances the cursor to the next target.
    /// If invalid, it initializes a new sequence from `candidates` where active is the origin,
    /// ordering subsequent windows in Z-order succession.
    pub fn cycle_order(
        &mut self,
        candidates: &[Candidate],
        active: &ActiveContext,
        now_ms: u64,
    ) -> Vec<WindowId> {
        let candidate_ids: Vec<WindowId> = candidates.iter().map(|c| c.facts.window).collect();
        if candidate_ids.is_empty() {
            self.reset();
            return Vec::new();
        }

        if self.is_valid(active.foreground, now_ms) {
            // Retain only windows that still exist as candidates
            self.windows.retain(|w| candidate_ids.contains(w));
            // Append any newly appeared candidates not already in the sequence
            for &cid in &candidate_ids {
                if !self.windows.contains(&cid) {
                    self.windows.push(cid);
                }
            }

            if self.windows.is_empty() {
                self.reset();
                return Vec::new();
            }

            // Sync cursor to current active window
            let current_pos = self
                .windows
                .iter()
                .position(|w| *w == active.foreground)
                .unwrap_or(self.cursor);
            self.cursor = (current_pos + 1) % self.windows.len();
        } else {
            // Fresh backward cycle session:
            // Order candidates starting immediately after active foreground, then wrap to active.
            let mut ordered = candidate_ids.clone();
            if !ordered.contains(&active.foreground) {
                ordered.insert(0, active.foreground);
            }
            let active_pos = ordered
                .iter()
                .position(|w| *w == active.foreground)
                .unwrap_or(0);
            self.windows = ordered;
            self.cursor = (active_pos + 1) % self.windows.len();
        }

        // Return candidate attempt order starting at self.cursor
        let len = self.windows.len();
        let mut targets = Vec::with_capacity(len);
        for i in 0..len {
            let idx = (self.cursor + i) % len;
            let target = self.windows[idx];
            if target != active.foreground {
                targets.push(target);
            }
        }
        targets
    }

    /// Record a successful window activation.
    pub fn record_activation(&mut self, target: WindowId, now_ms: u64) {
        self.last_activated = Some(target);
        self.last_tick_ms = now_ms;
        if let Some(pos) = self.windows.iter().position(|w| *w == target) {
            self.cursor = pos;
        }
    }
}

/// Drive one deterministic cycle pass.
/// Attempts each eligible target **at most once**, in wrap order, and stops at
/// the first activation. An [`ActivationOutcome::InvalidTarget`] or
/// [`ActivationOutcome::Failed`] continues to the next target; running out of
/// targets yields [`CycleOutcome::Exhausted`].
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
/// 's real policy must reproduce [`fixtures::EXPECTED_DECISIONS`]
/// exactly; the reference policy here exists only to drive the harness.
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
                // The cloaked window reports `visible: true` — that is the whole
                // point of this fixture. It pins the case that produced a blank
                // stop while cycling File Explorer: explorer.exe also owned a
                // full-screen, untitled, shell-cloaked `ApplicationFrameWindow`.
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

    /// Reference policy encoding the frozen decisions. replaces this
    /// with the production policy in `cycling/eligibility.rs`.
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
        // PID is absent from the contract by construction, so two windows from
        // separate processes of the same binary must still group.
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
        // Z-order [1 2 3], active 2 -> rotation [3 1] -> reversed [1 3].
        let candidates = ordered(vec![normal(1), normal(2), normal(3)]);
        let order = cycle_order(&candidates, &active(2));
        assert_eq!(order, vec![WindowId(1), WindowId(3)]);
    }

    #[test]
    fn repeated_cycles_reach_every_window() {
        // The regression that motivated the reversal: with three windows the
        // old first-after-active order ping-ponged between two of them and
        // never reached the third. Activation raises the target to the top of
        // the Z-order, which is what closed the loop.
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
        // Wrap covers 3 and 2; the active window 1 is never re-attempted.
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
        // Distinct processes, same binary: both must remain in the cycle.
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
        // Compile-time guarantee: the whole pass runs on injected data only.
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

        // Exceed session timeout (2000 ms)
        let targets2 = session.cycle_order(
            &candidates,
            &active(2),
            1000 + BACKWARD_CYCLE_SESSION_TIMEOUT_MS + 100,
        );
        // Resets and starts new sequence from active window 2 -> picks neighbor 3
        assert_eq!(targets2[0], WindowId(3));
    }

    #[test]
    fn backward_cycle_session_resets_on_focus_departure() {
        let candidates = ordered(vec![normal(1), normal(2), normal(3)]);
        let mut session = BackwardCycleSession::new();

        let targets1 = session.cycle_order(&candidates, &active(1), 1000);
        assert_eq!(targets1[0], WindowId(2));
        session.record_activation(WindowId(2), 1000);

        // Active foreground departed to unexpected window 99 (e.g. user clicked another app)
        let targets2 = session.cycle_order(&candidates, &active(99), 1100);
        // Session invalidates and treats candidate list afresh
        assert_eq!(targets2[0], WindowId(1));
    }

    #[test]
    fn backward_cycle_session_prunes_closed_window() {
        let candidates = ordered(vec![normal(1), normal(2), normal(3), normal(4)]);
        let mut session = BackwardCycleSession::new();

        let targets1 = session.cycle_order(&candidates, &active(1), 1000);
        assert_eq!(targets1[0], WindowId(2));
        session.record_activation(WindowId(2), 1000);

        // Window 3 closes before next backward tap: candidates become [2, 1, 4]
        let remaining_candidates = ordered(vec![normal(2), normal(1), normal(4)]);
        let targets2 = session.cycle_order(&remaining_candidates, &active(2), 1200);
        // Closed window 3 is pruned, next target is 4
        assert_eq!(targets2[0], WindowId(4));
        session.record_activation(WindowId(4), 1200);

        // Next tap from 4 wraps to 1
        let targets3 = session.cycle_order(&remaining_candidates, &active(4), 1400);
        assert_eq!(targets3[0], WindowId(1));
    }
}
