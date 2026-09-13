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
    selected_index: usize,
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
            selected_index: 0,
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
        self.selected_index = selected_index;
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
            self.selected_index,
            self.overlay.per_page(),
        );
        self.selected_index = idx;
        self.overlay.set_selected_index(idx);
    }

    pub fn prev(&mut self) {
        if self.candidates.is_empty() {
            return;
        }
        let (idx, _) = selection::select_prev(
            self.candidates.len(),
            self.selected_index,
            self.overlay.per_page(),
        );
        self.selected_index = idx;
        self.overlay.set_selected_index(idx);
    }

    pub fn up(&mut self) {
        if self.candidates.is_empty() {
            return;
        }
        let idx = selection::select_up(
            self.candidates.len(),
            self.selected_index,
            &self.overlay.layout().cards,
            self.overlay.page_start(),
        );
        self.selected_index = idx;
        self.overlay.set_selected_index(idx);
    }

    pub fn down(&mut self) {
        if self.candidates.is_empty() {
            return;
        }
        let idx = selection::select_down(
            self.candidates.len(),
            self.selected_index,
            &self.overlay.layout().cards,
            self.overlay.page_start(),
        );
        self.selected_index = idx;
        self.overlay.set_selected_index(idx);
    }

    pub fn selected_window(&self) -> Option<WindowId> {
        self.candidates.get(self.selected_index).copied()
    }

    /// Returns candidate targets starting from `selected_index` for fallback activation.
    pub fn candidates_from_selection(&self) -> Vec<WindowId> {
        if self.candidates.is_empty() {
            return Vec::new();
        }
        let mut res = Vec::with_capacity(self.candidates.len());
        for i in self.selected_index..self.candidates.len() {
            res.push(self.candidates[i]);
        }
        for i in 0..self.selected_index {
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
        self.selected_index = 0;
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
}
