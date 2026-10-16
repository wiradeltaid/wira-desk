//! Pure switcher layout and selection calculation.

pub mod layout;
pub mod selection;

pub use layout::{
    aspect_from_rect, compute_layout, scale, CardLayout, Rect, SwitcherLayout, TileSpec, BASE_DPI,
    CARD_H_LOGICAL, DEFAULT_ASPECT, GUTTER_LOGICAL, HEADER_H_LOGICAL, MARGIN_LOGICAL, MAX_ASPECT,
    MAX_ROWS, MIN_ASPECT,
};
pub use selection::{select_down, select_next, select_prev, select_up};
