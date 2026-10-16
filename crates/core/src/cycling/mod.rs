//! Window cycling algorithms and data structures — pure domain logic, sans-IO.

pub mod eligibility;
pub mod selection;

pub use eligibility::evaluate_facts;
pub use selection::{attempt_order, select_after, select_next};

use crate::window::WindowId;

/// Application identity.
/// Identity is the case-insensitive executable basename.
#[derive(Debug, Clone, PartialEq, Eq)]
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

/// Standard Windows shell surfaces for default classification.
pub const SHELL_SURFACE_CLASSES: &[&str] = &[
    "Shell_TrayWnd",
    "Shell_SecondaryTrayWnd",
    "Progman",
    "WorkerW",
];

/// Standard helper surfaces (WinUI/XAML popup hosts) for default classification.
pub const HELPER_SURFACE_CLASSES: &[&str] = &[
    "PopupHost",
    "Xaml_WindowedPopupClass",
    "Microsoft.UI.Content.PopupWindowSiteBridge",
];

pub const CLASS_GHOST: &str = "Ghost";

/// Immutable facts captured for one window during one snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
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

/// Active-window context, sampled once per cycling command.
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

/// Direction of window cycling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Backward,
}

/// Order the candidates least-recently-used first or directed.
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
