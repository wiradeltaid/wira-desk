//! InputSink trait, modifier tracking, and effect definitions — sans-IO.

use crate::commands::Command;

/// Disposition returned from input processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Disposition {
    PassToNext,
    Swallow,
}

/// Abstract effect produced by input processing without immediate side-effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effect {
    /// Enqueue a command to be executed by worker.
    EnqueueCommand(Command),
    /// Wake up the worker/executor.
    WakeWorker,
    /// Disarm switcher overlay timer.
    DisarmSwitcher,
    /// Commit switcher overlay selection.
    CommitSwitcher,
    /// Cancel switcher overlay without activating.
    CancelSwitcher,
    /// Arm switcher timer (forward: true/false, deadline_ms).
    ArmSwitcher { forward: bool, deadline_ms: u64 },
}

/// Represents the physical state of modifier keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ModifierState {
    pub win: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

impl ModifierState {
    #[inline]
    pub fn new(win: bool, ctrl: bool, alt: bool, shift: bool) -> Self {
        Self {
            win,
            ctrl,
            alt,
            shift,
        }
    }

    #[inline]
    pub fn any(&self) -> bool {
        self.win || self.ctrl || self.alt || self.shift
    }

    #[inline]
    pub fn has_any_of(&self, other: &ModifierState) -> bool {
        (other.win && self.win)
            || (other.ctrl && self.ctrl)
            || (other.alt && self.alt)
            || (other.shift && self.shift)
    }

    #[inline]
    pub fn is_modifier_vk(vk: u32) -> bool {
        matches!(
            vk,
            0x5B | 0x5C // VK_LWIN | VK_RWIN
                | 0xA2 | 0xA3 // VK_LCONTROL | VK_RCONTROL
                | 0xA4 | 0xA5 // VK_LMENU | VK_RMENU
                | 0xA0 | 0xA1 // VK_LSHIFT | VK_RSHIFT
        )
    }

    pub fn apply_vk(&mut self, vk: u32, key_down: bool) {
        match vk {
            0x5B | 0x5C => self.win = key_down,
            0xA2 | 0xA3 => self.ctrl = key_down,
            0xA4 | 0xA5 => self.alt = key_down,
            0xA0 | 0xA1 => self.shift = key_down,
            _ => {}
        }
    }
}

/// Bounded throttle check against anti-macro repetition.
#[inline]
pub fn throttle_allows(last_accept_ms: u64, now_ms: u64, throttle_window_ms: u64) -> bool {
    last_accept_ms == 0 || now_ms.saturating_sub(last_accept_ms) >= throttle_window_ms
}

/// Abstract input event fed into an `InputSink`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
    Key {
        vk: u32,
        key_down: bool,
        now_ms: u64,
    },
    MouseWheel {
        delta: i32,
        horizontal: bool,
        now_ms: u64,
    },
    MouseButton {
        button: u32,
        down: bool,
        now_ms: u64,
    },
}

/// Pure sans-IO trait for processing input events in bounded time (<10ms).
pub trait InputSink {
    fn on_input(&mut self, event: InputEvent) -> (Disposition, Vec<Effect>);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modifier_state_tracking() {
        let mut mods = ModifierState::default();
        assert!(!mods.any());

        mods.apply_vk(0x5B, true); // VK_LWIN down
        assert!(mods.win);
        assert!(mods.any());

        mods.apply_vk(0xA2, true); // VK_LCONTROL down
        assert!(mods.ctrl);

        let other = ModifierState::new(false, true, false, false);
        assert!(mods.has_any_of(&other));

        mods.apply_vk(0x5B, false); // VK_LWIN up
        assert!(!mods.win);
        assert!(mods.ctrl);
    }

    #[test]
    fn throttle_boundary() {
        assert!(throttle_allows(0, 100, 50));
        assert!(!throttle_allows(100, 120, 50));
        assert!(throttle_allows(100, 150, 50));
        assert!(throttle_allows(100, 160, 50));
    }
}
