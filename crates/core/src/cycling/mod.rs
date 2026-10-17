//! Pure window cycling domain types, facts, and ordering algorithms.

pub mod eligibility;
pub mod selection;

pub use crate::window::WindowId;
pub use eligibility::evaluate_facts;
pub use selection::{attempt_order, select_after, select_next};

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
/// *query* the process but never as the primary same-application key.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AppIdentity {
    /// Normalized lowercase executable basename, e.g. `notepad.exe`.
    Executable(String),
    /// The process vanished or denied access. Never matches anything.
    Unavailable,
}

impl AppIdentity {
    /// Build an identity from a full process image path.
    pub fn from_process_path(path: Option<&str>) -> Self {
        match path.and_then(normalize_executable) {
            Some(name) => AppIdentity::Executable(name),
            None => AppIdentity::Unavailable,
        }
    }

    /// Same-application test.
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
pub fn normalize_executable(path: &str) -> Option<String> {
    let basename = path.rsplit(['\\', '/']).next().unwrap_or(path).trim();
    if basename.is_empty() {
        return None;
    }
    Some(basename.to_ascii_lowercase())
}

/// Immutable facts captured for one window during one snapshot.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WindowFacts {
    pub window: WindowId,
    pub visible: bool,
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    pub z_index: usize,
    pub facts: WindowFacts,
}

/// Active-window context, sampled once per cycle operation.
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
    Activated,
    InvalidTarget,
    Failed,
}

/// Terminal result of one cycle pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CycleOutcome {
    Activated(WindowId),
    Exhausted,
    NoEligibleTarget,
}

/// Direction of window cycling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Backward,
}

/// Order candidates according to cycling direction.
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
pub const BACKWARD_CYCLE_SESSION_TIMEOUT_MS: u64 = 2000;

/// Tracks visitation sequence for blind backward cycling across activations.
#[derive(Debug, Clone, Default)]
pub struct BackwardCycleSession {
    pub windows: Vec<WindowId>,
    pub cursor: usize,
    pub last_activated: Option<WindowId>,
    pub last_tick_ms: u64,
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
            self.windows.retain(|w| candidate_ids.contains(w));
            for &cid in &candidate_ids {
                if !self.windows.contains(&cid) {
                    self.windows.push(cid);
                }
            }

            if self.windows.is_empty() {
                self.reset();
                return Vec::new();
            }

            let current_pos = self
                .windows
                .iter()
                .position(|w| *w == active.foreground)
                .unwrap_or(self.cursor);
            self.cursor = (current_pos + 1) % self.windows.len();
        } else {
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

    pub fn record_activation(&mut self, target: WindowId, now_ms: u64) {
        self.last_activated = Some(target);
        self.last_tick_ms = now_ms;
        if let Some(pos) = self.windows.iter().position(|w| *w == target) {
            self.cursor = pos;
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn test_candidate(id: isize) -> Candidate {
        Candidate {
            z_index: 0,
            facts: WindowFacts {
                window: WindowId(id),
                visible: true,
                cloaked: false,
                iconic: false,
                tool_window: false,
                class_name: "Notepad".to_string(),
                identity: AppIdentity::Executable("notepad.exe".to_string()),
                has_title: true,
                has_nonzero_extent: true,
                is_owned: false,
            },
        }
    }

    #[test]
    fn test_executable_normalization() {
        assert_eq!(
            normalize_executable(r"C:\Windows\System32\NOTEPAD.EXE"),
            Some("notepad.exe".to_string())
        );
        assert_eq!(
            normalize_executable("C:/Program Files/App/App.Exe"),
            Some("app.exe".to_string())
        );
        assert_eq!(normalize_executable(""), None);
    }

    #[test]
    fn test_cycle_order_least_recently_used() {
        let candidates = vec![test_candidate(1), test_candidate(2), test_candidate(3)];
        let active = ActiveContext {
            foreground: WindowId(2),
            identity: AppIdentity::Executable("notepad.exe".to_string()),
        };
        let order = cycle_order(&candidates, &active);
        assert_eq!(order, vec![WindowId(1), WindowId(3)]);
    }

    #[test]
    fn test_backward_cycle_session_traversal() {
        let candidates = vec![
            test_candidate(1),
            test_candidate(2),
            test_candidate(3),
            test_candidate(4),
        ];
        let mut session = BackwardCycleSession::new();
        let active = ActiveContext {
            foreground: WindowId(1),
            identity: AppIdentity::Executable("notepad.exe".to_string()),
        };

        let targets = session.cycle_order(&candidates, &active, 1000);
        assert_eq!(targets[0], WindowId(2));
        session.record_activation(WindowId(2), 1000);

        let active2 = ActiveContext {
            foreground: WindowId(2),
            identity: AppIdentity::Executable("notepad.exe".to_string()),
        };
        let targets2 = session.cycle_order(&candidates, &active2, 1200);
        assert_eq!(targets2[0], WindowId(3));
    }
}
