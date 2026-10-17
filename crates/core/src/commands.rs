//! `u8` command enum flowing through the Hook → Worker ring buffer.
//! The ring buffer carries only raw `u8` values — no heap allocation. Both sides
//! convert via `Command::from_u8` and `Command as u8`.

/// Command decoded by the Hook Thread from key combinations, then executed
/// by the Worker Thread. Discrete `u8` values for safe transfer via ring buffer.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Command {
    /// No command (empty slot).
    Nop = 0,
    /// Rotate focus among same-app windows (Win+Backtick).
    Cycle = 1,
    /// Snap the active window to the left half of the screen (Ctrl+Win+Left).
    SnapLeft = 2,
    /// Snap the active window to the right half of the screen (Ctrl+Win+Right).
    SnapRight = 3,
    /// Maximize the active window (Ctrl+Win+Enter).
    SnapMaximize = 4,
    /// Rearrange windows with overlapping stack layout.
    OverlappingStack = 5,
    /// Snap the active window to the top half of the screen.
    SnapTop = 6,
    /// Snap the active window to the bottom half of the screen.
    SnapBottom = 7,
    /// Move the active window to the next physical monitor, keeping its share
    /// of the work area.
    MoveToNextMonitor = 8,
    /// Snap the active window against the left edge at a configured percentage.
    SnapPercentLeft = 9,
    /// Snap the active window against the right edge at a configured percentage.
    SnapPercentRight = 10,
    /// Snap the active window against the top edge at a configured percentage.
    SnapPercentTop = 11,
    /// Snap the active window against the bottom edge at a configured percentage.
    SnapPercentBottom = 12,
    /// Snap the active window to the left third of the screen.
    SnapThirdLeft = 13,
    /// Snap the active window to the middle third of the screen.
    SnapThirdMiddle = 14,
    /// Snap the active window to the right third of the screen.
    SnapThirdRight = 15,
    /// Switch to the next virtual desktop (Ctrl+Win+Right).
    NextVirtualDesktop = 16,
    /// Switch to the previous virtual desktop (Ctrl+Win+Left).
    PrevVirtualDesktop = 17,
    /// Open Windows Task View (Win+Tab).
    TaskView = 18,
    /// Show Windows Desktop (Win+D).
    ShowDesktop = 19,
    /// Disarm switcher hold timer before overlay opens (main key released before deadline).
    SwitcherDisarm = 20,
    /// Advance switcher selection to next card.
    SwitcherNext = 21,
    /// Retreat switcher selection to previous card.
    SwitcherPrev = 22,
    /// Move switcher selection up one row.
    SwitcherUp = 23,
    /// Move switcher selection down one row.
    SwitcherDown = 24,
    /// Commit switcher selection (all modifiers released).
    SwitcherCommit = 25,
    /// Cancel switcher and dismiss overlay (Escape or watchdog).
    SwitcherCancel = 26,
    /// Rotate focus backward among same-app windows (Shift + Win+Backtick).
    CyclePrev = 27,
    /// Arm switcher hold timer on worker window (forward direction).
    SwitcherArm = 28,
    /// Arm switcher hold timer on worker window (backward direction).
    SwitcherArmPrev = 29,
}

impl Command {
    /// Convert from raw `u8` (read from ring buffer). Unknown values → `Nop`.
    #[inline]
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => Command::Cycle,
            2 => Command::SnapLeft,
            3 => Command::SnapRight,
            4 => Command::SnapMaximize,
            5 => Command::OverlappingStack,
            6 => Command::SnapTop,
            7 => Command::SnapBottom,
            8 => Command::MoveToNextMonitor,
            9 => Command::SnapPercentLeft,
            10 => Command::SnapPercentRight,
            11 => Command::SnapPercentTop,
            12 => Command::SnapPercentBottom,
            13 => Command::SnapThirdLeft,
            14 => Command::SnapThirdMiddle,
            15 => Command::SnapThirdRight,
            16 => Command::NextVirtualDesktop,
            17 => Command::PrevVirtualDesktop,
            18 => Command::TaskView,
            19 => Command::ShowDesktop,
            20 => Command::SwitcherDisarm,
            21 => Command::SwitcherNext,
            22 => Command::SwitcherPrev,
            23 => Command::SwitcherUp,
            24 => Command::SwitcherDown,
            25 => Command::SwitcherCommit,
            26 => Command::SwitcherCancel,
            27 => Command::CyclePrev,
            28 => Command::SwitcherArm,
            29 => Command::SwitcherArmPrev,
            _ => Command::Nop,
        }
    }

    /// Whether this command is exempt from `ANTI_MACRO_THROTTLE_MS`.
    /// Switcher navigation and state commands must never drop held repeat ticks.
    #[inline]
    pub fn is_exempt_from_throttle(self) -> bool {
        matches!(
            self,
            Command::SwitcherArm
                | Command::SwitcherArmPrev
                | Command::SwitcherDisarm
                | Command::SwitcherNext
                | Command::SwitcherPrev
                | Command::SwitcherUp
                | Command::SwitcherDown
                | Command::SwitcherCommit
                | Command::SwitcherCancel
        )
    }

    /// Convert to raw `u8` for writing to the ring buffer.
    #[inline]
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_command_serialization() {
        let cmd = Command::SnapLeft;
        let serialized = serde_json::to_string(&cmd).unwrap();
        let deserialized: Command = serde_json::from_str(&serialized).unwrap();
        assert_eq!(cmd, deserialized);
    }

    #[test]
    fn from_u8_roundtrip() {
        for val in 0..=29 {
            let cmd = Command::from_u8(val);
            assert_eq!(cmd.as_u8(), val);
        }
    }

    #[test]
    fn from_u8_unknown_yields_nop() {
        assert_eq!(Command::from_u8(30), Command::Nop);
        assert_eq!(Command::from_u8(255), Command::Nop);
    }
}
