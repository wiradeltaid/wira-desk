//! Pure geometry arrangement models and planners.
//! Rectangles use **signed physical-pixel** coordinates with **half-open**
//! edges: `left <= x < right`, `top <= y < bottom`. Signed because a secondary
//! monitor legitimately sits at negative coordinates; half-open because it makes
//! "left half plus right half exactly tiles the work area" true by construction
//! rather than by an off-by-one convention everyone has to remember.
//! Every arithmetic step is checked. A geometry that cannot be represented
//! safely yields a [`PlanError`] rather than a partial or wrapped placement.

pub mod monitor;
pub mod snap;
pub mod stack;
pub mod thirds;

use crate::window::WindowId;

/// Half-open rectangle in signed physical pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    /// Construct a rectangle, rejecting empty or inverted geometry.
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Result<Rect, PlanError> {
        let rect = Rect {
            left,
            top,
            right,
            bottom,
        };
        rect.validate()?;
        Ok(rect)
    }

    pub fn checked_width(&self) -> Option<i32> {
        self.right.checked_sub(self.left)
    }

    pub fn checked_height(&self) -> Option<i32> {
        self.bottom.checked_sub(self.top)
    }

    /// Width. Only meaningful once the rectangle has been validated.
    pub fn width(&self) -> i32 {
        self.checked_width().unwrap_or(0)
    }

    pub fn height(&self) -> i32 {
        self.checked_height().unwrap_or(0)
    }

    /// Convenience predicate. Prefer [`Rect::validate`] on any path that
    /// reports an error: this collapses "overflowed" and "empty" into one
    /// `false` and cannot tell the caller which happened.
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    /// Validate extents, distinguishing an unrepresentable rectangle from a
    /// merely empty or inverted one.
    pub fn validate(&self) -> Result<(), PlanError> {
        let width = self
            .checked_width()
            .ok_or(PlanError::UnrepresentableGeometry)?;
        let height = self
            .checked_height()
            .ok_or(PlanError::UnrepresentableGeometry)?;
        if width <= 0 || height <= 0 {
            return Err(PlanError::EmptyOrInvertedWorkArea);
        }
        Ok(())
    }

    /// True when `self` fully encloses `other`.
    pub fn contains(&self, other: &Rect) -> bool {
        other.left >= self.left
            && other.top >= self.top
            && other.right <= self.right
            && other.bottom <= self.bottom
    }
}

/// Monitor work area plus its DPI context.
/// `rect` is the *usable* area — taskbar and reserved appbars already excluded
/// by the caller. `dpi` is carried for traceability only: coordinates arrive
/// already in physical pixels from a Per-Monitor-V2-aware process, so planners
/// must never scale by it a second time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WorkArea {
    pub rect: Rect,
    pub dpi: u32,
}

impl WorkArea {
    pub fn new(rect: Rect, dpi: u32) -> Result<WorkArea, PlanError> {
        rect.validate()?;
        Ok(WorkArea { rect, dpi })
    }
}

/// Deterministic planning failure. No partial placement ever accompanies one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanError {
    EmptyOrInvertedWorkArea,
    UnrepresentableGeometry,
    InvalidWidthPercent(u32),
}

/// One window and where it should go.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Placement {
    pub window: WindowId,
    pub rect: Rect,
}

/// A complete plan. An **empty** plan is a successful no-op, not a failure —
/// stack-disabled and zero-candidate cases both land here.
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct PlacementPlan {
    pub placements: Vec<Placement>,
}

impl PlacementPlan {
    pub fn empty() -> Self {
        PlacementPlan::default()
    }

    pub fn single(window: WindowId, rect: Rect) -> Self {
        PlacementPlan {
            placements: vec![Placement { window, rect }],
        }
    }

    pub fn is_noop(&self) -> bool {
        self.placements.is_empty()
    }

    /// Every placement lies inside `work`.
    pub fn fits_within(&self, work: &Rect) -> bool {
        self.placements.iter().all(|p| work.contains(&p.rect))
    }
}

pub type PlanResult = Result<PlacementPlan, PlanError>;

/// Applies a plan to windows.
pub trait WindowMover {
    fn restore(&mut self, window: WindowId);
    fn apply(&mut self, placement: &Placement) -> bool;
}

/// Maximum windows an overlapping stack may arrange.
pub const STACK_MAX_WINDOWS: usize = 3;

pub mod fixtures {
    use super::*;

    /// Primary 1920x1080 with a 40 px taskbar already excluded.
    pub fn primary_work_area() -> WorkArea {
        WorkArea::new(Rect::new(0, 0, 1920, 1040).unwrap(), 96).unwrap()
    }

    /// Secondary monitor at negative coordinates — a real multi-monitor layout.
    pub fn negative_origin_work_area() -> WorkArea {
        WorkArea::new(Rect::new(-1920, -200, 0, 880).unwrap(), 120).unwrap()
    }

    /// Odd width, to exercise the deterministic split.
    pub fn odd_width_work_area() -> WorkArea {
        WorkArea::new(Rect::new(0, 0, 1367, 769).unwrap(), 144).unwrap()
    }

    pub fn dpi_variants() -> Vec<WorkArea> {
        [96u32, 120, 144, 192]
            .into_iter()
            .map(|dpi| WorkArea::new(Rect::new(0, 0, 1920, 1040).unwrap(), dpi).unwrap())
            .collect()
    }

    pub fn windows(n: usize) -> Vec<WindowId> {
        (1..=n as isize).map(WindowId).collect()
    }
}
