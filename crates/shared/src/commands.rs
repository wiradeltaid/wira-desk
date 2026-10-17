//! `u8` command enum flowing through the Hook → Worker ring buffer.
//! Backward-compatible facade re-exporting pure domain command from `wiradesk_core`.

pub use wiradesk_core::commands::Command;

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn switcher_commands_are_exempt_from_throttle() {
        assert!(Command::SwitcherArm.is_exempt_from_throttle());
        assert!(Command::SwitcherArmPrev.is_exempt_from_throttle());
        assert!(Command::SwitcherDisarm.is_exempt_from_throttle());
        assert!(Command::SwitcherNext.is_exempt_from_throttle());
        assert!(Command::SwitcherPrev.is_exempt_from_throttle());
        assert!(Command::SwitcherUp.is_exempt_from_throttle());
        assert!(Command::SwitcherDown.is_exempt_from_throttle());
        assert!(Command::SwitcherCommit.is_exempt_from_throttle());
        assert!(Command::SwitcherCancel.is_exempt_from_throttle());
    }

    #[test]
    fn non_switcher_commands_are_not_exempt_from_throttle() {
        assert!(!Command::Nop.is_exempt_from_throttle());
        assert!(!Command::Cycle.is_exempt_from_throttle());
        assert!(!Command::CyclePrev.is_exempt_from_throttle());
        assert!(!Command::SnapLeft.is_exempt_from_throttle());
        assert!(!Command::SnapRight.is_exempt_from_throttle());
        assert!(!Command::SnapMaximize.is_exempt_from_throttle());
        assert!(!Command::OverlappingStack.is_exempt_from_throttle());
    }
}
