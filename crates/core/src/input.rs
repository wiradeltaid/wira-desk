//! Input protocol and state definitions for sans-IO input matching.

use crate::commands::Command;

/// Disposition returned from input handling: whether to swallow or pass to next handler.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Disposition {
    PassToNext,
    Swallow,
}

/// Side-effects generated during input decision without performing OS syscalls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effect {
    None,
    Enqueue(Command),
    WakeWorker,
    EnqueueAndWake(Command),
}

/// Generic input event descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
    Key { vk: u32, key_down: bool },
    Mouse { msg: u32, mouse_data: u32 },
}

/// Collapsed modifier state derived from input events.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ModifierState {
    pub win: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

impl ModifierState {
    pub const fn new(win: bool, ctrl: bool, alt: bool, shift: bool) -> Self {
        Self {
            win,
            ctrl,
            alt,
            shift,
        }
    }

    pub fn apply_vk(&mut self, vk: u32, key_down: bool) {
        const VK_LWIN: u32 = 0x5B;
        const VK_RWIN: u32 = 0x5C;
        const VK_LCONTROL: u32 = 0xA2;
        const VK_RCONTROL: u32 = 0xA3;
        const VK_LMENU: u32 = 0xA4;
        const VK_RMENU: u32 = 0xA5;
        const VK_LSHIFT: u32 = 0xA0;
        const VK_RSHIFT: u32 = 0xA1;

        match vk {
            VK_LWIN | VK_RWIN => self.win = key_down,
            VK_LCONTROL | VK_RCONTROL => self.ctrl = key_down,
            VK_LMENU | VK_RMENU => self.alt = key_down,
            VK_LSHIFT | VK_RSHIFT => self.shift = key_down,
            _ => {}
        }
    }

    /// True while any modifier is still held.
    #[inline]
    pub fn any(&self) -> bool {
        self.win || self.ctrl || self.alt || self.shift
    }

    /// Returns true if this state still contains any of the modifiers present in `other`.
    #[inline]
    pub fn has_any_of(&self, other: &ModifierState) -> bool {
        (other.win && self.win)
            || (other.ctrl && self.ctrl)
            || (other.alt && self.alt)
            || (other.shift && self.shift)
    }

    #[inline]
    pub fn is_modifier_vk(vk: u32) -> bool {
        const VK_LWIN: u32 = 0x5B;
        const VK_RWIN: u32 = 0x5C;
        const VK_LCONTROL: u32 = 0xA2;
        const VK_RCONTROL: u32 = 0xA3;
        const VK_LMENU: u32 = 0xA4;
        const VK_RMENU: u32 = 0xA5;
        const VK_LSHIFT: u32 = 0xA0;
        const VK_RSHIFT: u32 = 0xA1;

        matches!(
            vk,
            VK_LWIN
                | VK_RWIN
                | VK_LCONTROL
                | VK_RCONTROL
                | VK_LMENU
                | VK_RMENU
                | VK_LSHIFT
                | VK_RSHIFT
        )
    }
}

/// Abstract input consumer / sink for event processing.
pub trait InputSink {
    fn on_input(&mut self, event: InputEvent) -> (Disposition, Effect);
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_modifier_state_evaluation() {
        let mut mods = ModifierState::default();
        assert!(!mods.any());

        // LWIN down
        mods.apply_vk(0x5B, true);
        assert!(mods.win);
        assert!(mods.any());

        // LALT down
        mods.apply_vk(0xA4, true);
        assert!(mods.alt);

        let other = ModifierState::new(false, false, true, false);
        assert!(mods.has_any_of(&other));

        // LWIN up
        mods.apply_vk(0x5B, false);
        assert!(!mods.win);
        assert!(mods.alt);

        // LALT up
        mods.apply_vk(0xA4, false);
        assert!(!mods.any());
    }

    #[test]
    fn test_is_modifier_vk() {
        assert!(ModifierState::is_modifier_vk(0x5B)); // LWIN
        assert!(ModifierState::is_modifier_vk(0xA2)); // LCONTROL
        assert!(ModifierState::is_modifier_vk(0xA4)); // LMENU (ALT)
        assert!(ModifierState::is_modifier_vk(0xA0)); // LSHIFT
        assert!(!ModifierState::is_modifier_vk(0x41)); // 'A'
    }
}
