//! Frozen context-safety policy models and pure evaluation logic.

/// Opaque physical-monitor handle (`HMONITOR`), kept as a plain integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
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

/// Scope of spatial evaluation: blind cycling vs. visual switcher (DEC-026 §B-7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialScope {
    /// Lock evaluation to the active monitor. Used by blind cycling (FR-1, FR-2).
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

/// Decide spatial eligibility. **Fails closed on any uncertainty.**
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
    Passthrough(BypassReason),
    ContinueWiraDeskMatching,
}

impl BypassDecision {
    pub fn is_passthrough(&self) -> bool {
        matches!(self, BypassDecision::Passthrough(_))
    }
}

/// Immutable, pre-normalized bypass policy owned by the Hook Thread.
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

    /// Classify the foreground. **Fails open on uncertainty.**
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

fn normalize_identifiers(raw: &[String]) -> Vec<String> {
    raw.iter()
        .map(|s| s.trim().to_ascii_lowercase())
        .filter(|s| !s.is_empty())
        .collect()
}

fn contains_ignore_case(haystack: &[String], needle: &str) -> bool {
    haystack.iter().any(|e| e.eq_ignore_ascii_case(needle))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_bypass_policy_classification() {
        let policy = BypassPolicy::new(
            &["vmware.exe".to_owned(), "mstsc.exe".to_owned()],
            &["VMPlayerFrame".to_owned()],
        );

        let vm_id = ForegroundIdentity {
            process: Some("VMWare.exe".to_owned()),
            class: Some("NormalClass".to_owned()),
        };
        assert_eq!(
            policy.classify(&vm_id),
            BypassDecision::Passthrough(BypassReason::ProcessMatch)
        );

        let normal_id = ForegroundIdentity {
            process: Some("notepad.exe".to_owned()),
            class: Some("Notepad".to_owned()),
        };
        assert_eq!(
            policy.classify(&normal_id),
            BypassDecision::ContinueWiraDeskMatching
        );

        let missing_id = ForegroundIdentity {
            process: None,
            class: Some("Notepad".to_owned()),
        };
        assert_eq!(
            policy.classify(&missing_id),
            BypassDecision::Passthrough(BypassReason::IdentityUnavailable)
        );
    }
}
