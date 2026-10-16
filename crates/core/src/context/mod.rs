//! Spatial eligibility and foreground bypass policies — sans-IO.

use crate::window::WindowId;

/// Opaque physical-monitor handle, kept as a plain integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MonitorId(pub isize);

// ── Spatial eligibility ─────────────────────────────────────────────────────

/// Origin facts resolved once per cycle operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpatialContext {
    pub origin_monitor: Option<MonitorId>,
}

/// Per-candidate spatial facts, captured live for the current command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpatialFacts {
    pub candidate_monitor: Option<MonitorId>,
    pub on_current_virtual_desktop: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialRejection {
    DifferentMonitor,
    NotOnCurrentVirtualDesktop,
    MonitorUnavailable,
    VirtualDesktopUnavailable,
}

/// Scope of spatial evaluation: blind cycling vs. visual switcher.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialScope {
    /// Lock evaluation to the active monitor. Used by blind cycling.
    SameMonitor,
    /// Enumerate candidates across all physical monitors on the current virtual desktop.
    AnyMonitorOnCurrentDesktop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialDecision {
    Eligible,
    Ineligible(SpatialRejection),
}

impl SpatialDecision {
    pub fn is_eligible(&self) -> bool {
        matches!(self, SpatialDecision::Eligible)
    }
}

/// Decide spatial eligibility. Fails closed on any uncertainty.
pub fn evaluate_spatial(
    scope: SpatialScope,
    ctx: &SpatialContext,
    facts: &SpatialFacts,
) -> SpatialDecision {
    match scope {
        SpatialScope::SameMonitor => {
            let Some(origin) = ctx.origin_monitor else {
                return SpatialDecision::Ineligible(SpatialRejection::MonitorUnavailable);
            };
            let Some(candidate) = facts.candidate_monitor else {
                return SpatialDecision::Ineligible(SpatialRejection::MonitorUnavailable);
            };
            if origin != candidate {
                return SpatialDecision::Ineligible(SpatialRejection::DifferentMonitor);
            }
        }
        SpatialScope::AnyMonitorOnCurrentDesktop => {}
    }
    match facts.on_current_virtual_desktop {
        None => SpatialDecision::Ineligible(SpatialRejection::VirtualDesktopUnavailable),
        Some(false) => SpatialDecision::Ineligible(SpatialRejection::NotOnCurrentVirtualDesktop),
        Some(true) => SpatialDecision::Eligible,
    }
}

// ── Foreground bypass classification ────────────────────────────────────────

/// What we know about the foreground window when a shortcut arrives.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ForegroundIdentity {
    pub process: Option<String>,
    pub class: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BypassReason {
    ProcessMatch,
    ClassMatch,
    IdentityUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BypassDecision {
    /// Let the physical key combination through untouched.
    Passthrough(BypassReason),
    /// Confirmed non-match — Wira Desk may handle the shortcut.
    ContinueWiraDeskMatching,
}

impl BypassDecision {
    pub fn is_passthrough(&self) -> bool {
        matches!(self, BypassDecision::Passthrough(_))
    }
}

/// Immutable, pre-normalized bypass policy.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BypassPolicy {
    processes: Vec<String>,
    classes: Vec<String>,
}

impl BypassPolicy {
    pub fn new(processes: &[String], classes: &[String]) -> Self {
        BypassPolicy {
            processes: normalize_identifiers(processes),
            classes: normalize_identifiers(classes),
        }
    }

    pub fn processes(&self) -> &[String] {
        &self.processes
    }

    pub fn classes(&self) -> &[String] {
        &self.classes
    }

    /// Classify the foreground. Fails open on uncertainty.
    pub fn classify(&self, identity: &ForegroundIdentity) -> BypassDecision {
        if let Some(process) = identity.process.as_deref() {
            if contains_ignore_case(&self.processes, process) {
                return BypassDecision::Passthrough(BypassReason::ProcessMatch);
            }
        }
        if let Some(class) = identity.class.as_deref() {
            if contains_ignore_case(&self.classes, class) {
                return BypassDecision::Passthrough(BypassReason::ClassMatch);
            }
        }
        if identity.process.is_none() || identity.class.is_none() {
            return BypassDecision::Passthrough(BypassReason::IdentityUnavailable);
        }
        BypassDecision::ContinueWiraDeskMatching
    }
}

pub fn normalize_identifiers(raw: &[String]) -> Vec<String> {
    raw.iter()
        .map(|s| s.trim().to_ascii_lowercase())
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn contains_ignore_case(haystack: &[String], needle: &str) -> bool {
    haystack.iter().any(|e| e.eq_ignore_ascii_case(needle))
}

pub trait MonitorSource {
    fn monitor_of(&self, window: WindowId) -> Option<MonitorId>;
}

pub trait VirtualDesktopSource {
    fn is_on_current_desktop(&self, window: WindowId) -> Option<bool>;
}

pub trait ForegroundIdentitySource {
    fn foreground_identity(&self) -> ForegroundIdentity;
}

pub fn collect_spatial_facts<M, V>(monitors: &M, desktops: &V, candidate: WindowId) -> SpatialFacts
where
    M: MonitorSource + ?Sized,
    V: VirtualDesktopSource + ?Sized,
{
    SpatialFacts {
        candidate_monitor: monitors.monitor_of(candidate),
        on_current_virtual_desktop: desktops.is_on_current_desktop(candidate),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spatial_fails_closed_when_different_monitor() {
        let ctx = SpatialContext {
            origin_monitor: Some(MonitorId(1)),
        };
        let facts = SpatialFacts {
            candidate_monitor: Some(MonitorId(2)),
            on_current_virtual_desktop: Some(true),
        };
        assert_eq!(
            evaluate_spatial(SpatialScope::SameMonitor, &ctx, &facts),
            SpatialDecision::Ineligible(SpatialRejection::DifferentMonitor)
        );
    }

    #[test]
    fn spatial_eligible_when_same_monitor_and_desktop() {
        let ctx = SpatialContext {
            origin_monitor: Some(MonitorId(1)),
        };
        let facts = SpatialFacts {
            candidate_monitor: Some(MonitorId(1)),
            on_current_virtual_desktop: Some(true),
        };
        assert_eq!(
            evaluate_spatial(SpatialScope::SameMonitor, &ctx, &facts),
            SpatialDecision::Eligible
        );
    }

    #[test]
    fn bypass_matches_configured_process_case_insensitively() {
        let policy = BypassPolicy::new(&["vmware.exe".to_string()], &[]);
        let id = ForegroundIdentity {
            process: Some("VMWARE.EXE".to_string()),
            class: Some("SomeClass".to_string()),
        };
        assert_eq!(
            policy.classify(&id),
            BypassDecision::Passthrough(BypassReason::ProcessMatch)
        );
    }
}
