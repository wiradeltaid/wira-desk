//! Same-App Visual Switcher domain module.

#![allow(dead_code)]

pub mod layout;
pub mod overlay;
pub mod selection;
pub mod thumbnail;

use crate::cycling::{ActiveContext, Candidate, WindowId};
use crate::switcher::layout::Rect;
use crate::switcher::overlay::SwitcherOverlay;
use crate::switcher::thumbnail::DwmThumbnailSink;
use shared::Shortcut;
use std::sync::{Arc, Mutex};

/// State of the visual switcher.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwitcherState {
    Inactive,
    Armed {
        chord: Shortcut,
        deadline_ms: u64,
    },
    Active {
        candidates: Vec<WindowId>,
        selected: usize,
        page: usize,
        origin_foreground: WindowId,
    },
}

/// Worker-side controller for visual switcher state and overlay lifecycle.
pub struct SwitcherController {
    overlay: SwitcherOverlay,
    origin_foreground: WindowId,
    candidates: Vec<WindowId>,
    open_time_ms: u64,
}

impl Default for SwitcherController {
    fn default() -> Self {
        Self::new()
    }
}

impl SwitcherController {
    pub fn new() -> Self {
        Self {
            overlay: SwitcherOverlay::new(Arc::new(Mutex::new(DwmThumbnailSink))),
            origin_foreground: WindowId(0),
            candidates: Vec::new(),
            open_time_ms: 0,
        }
    }

    pub fn is_open(&self) -> bool {
        self.overlay.is_visible()
    }

    pub fn open_time_ms(&self) -> u64 {
        self.open_time_ms
    }

    pub fn open(
        &mut self,
        origin: WindowId,
        work_area: Rect,
        candidates: Vec<WindowId>,
        aspects: &[f32],
        dpi: u32,
        selected_index: usize,
    ) {
        self.origin_foreground = origin;
        self.candidates = candidates;
        self.open_time_ms = crate::hook::tick_ms();
        self.overlay.show(
            work_area,
            self.candidates.clone(),
            aspects,
            dpi,
            selected_index,
        );
    }

    pub fn next(&mut self) {
        if self.candidates.is_empty() {
            return;
        }
        let (idx, _) = selection::select_next(
            self.candidates.len(),
            self.overlay.selected_index(),
            self.overlay.per_page(),
        );
        self.overlay.set_selected_index(idx);
    }

    pub fn prev(&mut self) {
        if self.candidates.is_empty() {
            return;
        }
        let (idx, _) = selection::select_prev(
            self.candidates.len(),
            self.overlay.selected_index(),
            self.overlay.per_page(),
        );
        self.overlay.set_selected_index(idx);
    }

    pub fn up(&mut self) {
        if self.candidates.is_empty() {
            return;
        }
        let idx = selection::select_up(
            self.candidates.len(),
            self.overlay.selected_index(),
            &self.overlay.layout().cards,
            self.overlay.page_start(),
        );
        self.overlay.set_selected_index(idx);
    }

    pub fn down(&mut self) {
        if self.candidates.is_empty() {
            return;
        }
        let idx = selection::select_down(
            self.candidates.len(),
            self.overlay.selected_index(),
            &self.overlay.layout().cards,
            self.overlay.page_start(),
        );
        self.overlay.set_selected_index(idx);
    }

    pub fn selected_index(&self) -> usize {
        self.overlay.selected_index()
    }

    pub fn set_selected_index(&mut self, idx: usize) {
        if idx < self.candidates.len() {
            self.overlay.set_selected_index(idx);
        }
    }

    pub fn overlay_mut(&mut self) -> &mut SwitcherOverlay {
        &mut self.overlay
    }

    pub fn selected_window(&self) -> Option<WindowId> {
        self.candidates.get(self.overlay.selected_index()).copied()
    }

    /// Returns candidate targets starting from `selected_index` for fallback activation.
    pub fn candidates_from_selection(&self) -> Vec<WindowId> {
        if self.candidates.is_empty() {
            return Vec::new();
        }
        let selected_index = self.overlay.selected_index();
        let mut res = Vec::with_capacity(self.candidates.len());
        for i in selected_index..self.candidates.len() {
            res.push(self.candidates[i]);
        }
        for i in 0..selected_index {
            res.push(self.candidates[i]);
        }
        res
    }

    pub fn origin_window(&self) -> WindowId {
        self.origin_foreground
    }

    pub fn dismiss(&mut self) {
        self.overlay.dismiss();
        self.candidates.clear();
        self.open_time_ms = 0;
    }
}

/// Returns the card order for candidates: active foreground window at index 0,
/// followed by remaining eligible windows in LRU order.
pub fn card_order_for_candidates(
    candidates: &[Candidate],
    active: &ActiveContext,
) -> Vec<WindowId> {
    let mut order = Vec::with_capacity(candidates.len());
    if candidates
        .iter()
        .any(|c| c.facts.window == active.foreground)
    {
        order.push(active.foreground);
    }
    let rest = crate::cycling::cycle_order(candidates, active);
    for w in rest {
        if w != active.foreground {
            order.push(w);
        }
    }
    order
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::context::fixtures::{FakeDesktops, FakeMonitors, MONITOR_A, MONITOR_B};
    use crate::cycling::eligibility::WindowEligibility;
    use crate::cycling::fixtures::*;
    use crate::cycling::{cycle_order, WindowId};
    use crate::worker::collect_eligible_candidates;

    #[test]
    fn switcher_candidate_set_diverges_from_blind_cycle_on_monitor_boundary() {
        let candidates = ordered(vec![normal(1), normal(2), normal(3), normal(4)]);
        let monitors = FakeMonitors(vec![
            (WindowId(1), Some(MONITOR_A)),
            (WindowId(2), Some(MONITOR_A)),
            (WindowId(3), Some(MONITOR_B)),
            (WindowId(4), Some(MONITOR_A)),
        ]);
        let desktops = FakeDesktops(vec![
            (WindowId(1), Some(true)),
            (WindowId(2), Some(false)),
            (WindowId(3), Some(true)),
            (WindowId(4), Some(true)),
        ]);
        let spatial = crate::context::SpatialContext {
            origin_monitor: Some(MONITOR_A),
        };
        let active = active(1);

        let (_blind_candidates, blind_eligible) = collect_eligible_candidates(
            &StaticSource(candidates.clone()),
            &WindowEligibility,
            &active,
            &monitors,
            Some(&desktops),
            &spatial,
            crate::context::SpatialScope::SameMonitor,
        );

        let (_switcher_candidates, switcher_eligible) = collect_eligible_candidates(
            &StaticSource(candidates.clone()),
            &WindowEligibility,
            &active,
            &monitors,
            Some(&desktops),
            &spatial,
            crate::context::SpatialScope::AnyMonitorOnCurrentDesktop,
        );

        // Under DEC-026, WindowId(3) on MONITOR_B is absent from blind cycle but present in switcher
        assert!(
            !blind_eligible.contains(&WindowId(3)),
            "Window on secondary monitor must be absent from blind cycle"
        );
        assert!(
            switcher_eligible.contains(&WindowId(3)),
            "Window on secondary monitor must be present in visual switcher (DEC-026)"
        );

        // WindowId(2) (on another virtual desktop) is absent from BOTH
        assert!(
            !blind_eligible.contains(&WindowId(2)),
            "Window on another virtual desktop must be absent from blind cycle"
        );
        assert!(
            !switcher_eligible.contains(&WindowId(2)),
            "Window on another virtual desktop must be absent from visual switcher"
        );

        // Visual switcher card order includes active foreground WindowId(1) at index 0
        let card_order = card_order_for_candidates(&candidates, &active);
        let switcher_cards: Vec<WindowId> = card_order
            .into_iter()
            .filter(|w| switcher_eligible.contains(w))
            .collect();
        assert_eq!(
            switcher_cards[0],
            WindowId(1),
            "Active foreground must be first card in visual switcher"
        );
        assert!(
            switcher_cards.contains(&WindowId(3)),
            "Secondary monitor window must be in visual switcher cards"
        );
        assert!(
            !switcher_cards.contains(&WindowId(2)),
            "Other virtual desktop window must not be in visual switcher cards"
        );
    }

    #[test]
    fn card_order_includes_active_foreground_window() {
        let candidates = ordered(vec![normal(1), normal(2), normal(3), normal(4)]);
        let active = active(2);

        let card_order = card_order_for_candidates(&candidates, &active);
        assert_eq!(card_order.len(), 4);
        assert_eq!(card_order[0], active.foreground);
        let cycle_rest = cycle_order(&candidates, &active);
        assert_eq!(&card_order[1..], cycle_rest.as_slice());
    }

    #[test]
    fn mouse_click_selection_updates_controller_candidate_selection() {
        let mut sw = SwitcherController::new();
        let work_area = Rect::new(0, 0, 1920, 1080);
        let candidates = vec![WindowId(10), WindowId(20), WindowId(30), WindowId(40)];
        let aspects = vec![1.6, 1.6, 1.6, 1.6];

        sw.open(WindowId(10), work_area, candidates, &aspects, 96, 0);
        assert_eq!(sw.selected_index(), 0);
        assert_eq!(sw.selected_window(), Some(WindowId(10)));

        // Simulate mouse clicking card index 2 (WindowId(30))
        sw.overlay_mut().set_selected_index(2);

        assert_eq!(sw.selected_index(), 2);
        assert_eq!(sw.selected_window(), Some(WindowId(30)));

        let targets = sw.candidates_from_selection();
        assert_eq!(
            targets[0],
            WindowId(30),
            "First candidate target must be the clicked window"
        );
        assert_eq!(
            targets,
            vec![WindowId(30), WindowId(40), WindowId(10), WindowId(20)]
        );

        // Multi-page test: test with 12 candidates across pages
        let multi_candidates: Vec<WindowId> = (1..=12).map(WindowId).collect();
        let multi_aspects = vec![1.6; 12];
        sw.open(
            WindowId(1),
            work_area,
            multi_candidates,
            &multi_aspects,
            96,
            0,
        );

        // Click a card on subsequent page (e.g. candidate index 7)
        sw.overlay_mut().set_selected_index(7);
        assert_eq!(sw.selected_index(), 7);
        assert_eq!(sw.selected_window(), Some(WindowId(8)));
        let multi_targets = sw.candidates_from_selection();
        assert_eq!(multi_targets[0], WindowId(8));

        sw.dismiss();
    }

    #[test]
    fn mouse_hover_updates_selection_for_subsequent_modifier_commit() {
        let mut sw = SwitcherController::new();
        let work_area = Rect::new(0, 0, 1920, 1080);
        let candidates = vec![WindowId(100), WindowId(200), WindowId(300), WindowId(400)];
        let aspects = vec![1.6; 4];

        sw.open(WindowId(100), work_area, candidates, &aspects, 96, 0);

        // User navigates once with keyboard to index 1 (WindowId(200))
        sw.next();
        assert_eq!(sw.selected_index(), 1);
        assert_eq!(sw.selected_window(), Some(WindowId(200)));

        // Mouse hovers over card 2 (WindowId(300))
        sw.overlay_mut().set_selected_index(2);
        assert_eq!(sw.selected_index(), 2);
        assert_eq!(sw.selected_window(), Some(WindowId(300)));

        // Keyboard modifier release triggers commit: targets must lead with hovered window
        let targets = sw.candidates_from_selection();
        assert_eq!(
            targets[0],
            WindowId(300),
            "Subsequent modifier commit must target the hovered window"
        );
        assert_eq!(
            targets,
            vec![WindowId(300), WindowId(400), WindowId(100), WindowId(200)]
        );

        // Subsequent keyboard navigation continues seamlessly from hovered card (2 -> 3)
        sw.next();
        assert_eq!(sw.selected_index(), 3);
        assert_eq!(sw.selected_window(), Some(WindowId(400)));

        sw.dismiss();
    }

    #[test]
    fn default_hold_delay_threshold_is_300ms() {
        assert_eq!(
            shared::config::SwitcherConfig::DEFAULT_HOLD_DELAY_MS,
            300,
            "Shared DEFAULT_HOLD_DELAY_MS must be 300 ms"
        );
        let default_config = shared::config::Config::default();
        assert_eq!(
            default_config.switcher.visual_hold_delay_ms, 300,
            "Default config visual_hold_delay_ms must be 300 ms"
        );
        // Clamping preservation
        assert_eq!(shared::config::SwitcherConfig::clamp_hold_delay(50), 100);
        assert_eq!(shared::config::SwitcherConfig::clamp_hold_delay(300), 300);
        assert_eq!(shared::config::SwitcherConfig::clamp_hold_delay(600), 500);
    }
}
