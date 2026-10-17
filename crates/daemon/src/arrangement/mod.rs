//! Frozen arrangement contract — pure geometry, no User32.
//! Win32 mover and window positioning implementations remain here in `win32.rs`.
//! Pure models and geometry planners are re-exported from `wiradesk_core::arrangement`.

#![allow(dead_code)]

pub use wiradesk_core::arrangement::*;
#[cfg(test)]
pub use wiradesk_core::arrangement::fixtures;
#[cfg(test)]
pub use wiradesk_core::WindowId;

pub mod monitor;
pub mod snap;
pub mod stack;
pub mod thirds;
pub mod win32;

#[cfg(test)]
mod tests {
    use shared::Command;

    // --- frozen command contract, verified from the consumer ----

    #[test]
    fn arrangement_commands_retain_frozen_wire_values() {
        assert_eq!(Command::SnapLeft.as_u8(), 2);
        assert_eq!(Command::SnapRight.as_u8(), 3);
        assert_eq!(Command::SnapMaximize.as_u8(), 4);
        assert_eq!(Command::OverlappingStack.as_u8(), 5);
        assert_eq!(Command::SnapTop.as_u8(), 6);
        assert_eq!(Command::SnapBottom.as_u8(), 7);
        assert_eq!(Command::MoveToNextMonitor.as_u8(), 8);
        assert_eq!(Command::SnapPercentLeft.as_u8(), 9);
        assert_eq!(Command::SnapPercentRight.as_u8(), 10);
        assert_eq!(Command::SnapPercentTop.as_u8(), 11);
        assert_eq!(Command::SnapPercentBottom.as_u8(), 12);
        assert_eq!(Command::SnapThirdLeft.as_u8(), 13);
        assert_eq!(Command::SnapThirdMiddle.as_u8(), 14);
        assert_eq!(Command::SnapThirdRight.as_u8(), 15);
    }

    #[test]
    fn unknown_command_values_still_decode_as_nop() {
        assert_eq!(Command::from_u8(0), Command::Nop);
        assert_eq!(Command::from_u8(255), Command::Nop);
    }
}
