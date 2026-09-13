//! Adaptive aspect-ratio tile layout and height-bounded row packing (SPEC-14-06, DEC-026).

pub const BASE_DPI: u32 = 96;
/// Logical card height at 96 DPI (uniform across all cards on a page).
pub const CARD_H_LOGICAL: i32 = 212;
/// Logical header height at 96 DPI.
pub const HEADER_H_LOGICAL: i32 = 32;
/// Logical gutter between cards at 96 DPI.
pub const GUTTER_LOGICAL: i32 = 16;
/// Logical margin around the overlay content at 96 DPI.
pub const MARGIN_LOGICAL: i32 = 24;
/// Maximum number of rows allowed on a page.
pub const MAX_ROWS: usize = 3;

/// Minimum and maximum aspect ratios bracketing tile dimensions (DEC-026).
pub const MIN_ASPECT: f32 = 0.60;
pub const MAX_ASPECT: f32 = 2.40;

/// Default aspect ratio fallback (16:9) when a window rect is missing or degenerate.
pub const DEFAULT_ASPECT: f32 = 16.0 / 9.0;

/// Scale logical units at 96 DPI to physical pixels based on monitor DPI.
#[inline]
pub fn scale(logical: i32, dpi: u32) -> i32 {
    if dpi == BASE_DPI || dpi == 0 {
        logical
    } else {
        (logical * dpi as i32 + (BASE_DPI as i32 / 2)) / BASE_DPI as i32
    }
}

/// Extract aspect ratio from an optional outer window rectangle, falling back to 16:9 on failure or zero height.
pub fn aspect_from_rect(rect: Option<Rect>) -> f32 {
    match rect {
        Some(r) if r.height > 0 && r.width > 0 => {
            (r.width as f32 / r.height as f32).clamp(MIN_ASPECT, MAX_ASPECT)
        }
        _ => DEFAULT_ASPECT,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CardLayout {
    pub chrome_rect: Rect,
    pub header_rect: Rect,
    pub preview_rect: Rect,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitcherLayout {
    pub overlay_rect: Rect,
    pub cols: usize,
    pub rows: usize,
    pub cards: Vec<CardLayout>,
    pub total_pages: usize,
    pub current_page: usize,
    pub page_start: usize,
    pub page_indicator_rect: Option<Rect>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileSpec {
    pub card_w: i32,
    pub card_h: i32,
    pub header_h: i32,
    pub preview_w: i32,
    pub preview_h: i32,
}

impl TileSpec {
    pub fn for_aspect(aspect: f32, dpi: u32) -> Self {
        let clamped = aspect.clamp(MIN_ASPECT, MAX_ASPECT);
        let preview_h_logical = CARD_H_LOGICAL - HEADER_H_LOGICAL - 8; // 172
        let preview_w_logical = (preview_h_logical as f32 * clamped).round() as i32;
        let card_w_logical = preview_w_logical + 8;

        Self {
            card_w: scale(card_w_logical, dpi),
            card_h: scale(CARD_H_LOGICAL, dpi),
            header_h: scale(HEADER_H_LOGICAL, dpi),
            preview_w: scale(preview_w_logical, dpi),
            preview_h: scale(preview_h_logical, dpi),
        }
    }
}

struct PackedRow {
    tiles: Vec<(usize, TileSpec)>,
    width: i32,
}

struct PackedPage {
    rows: Vec<PackedRow>,
}

/// Compute full layout geometry for the given work area and candidates page.
pub fn compute_layout(work_area: Rect, aspects: &[f32], page: usize, dpi: u32) -> SwitcherLayout {
    let dpi = if dpi == 0 { BASE_DPI } else { dpi };
    if aspects.is_empty() {
        return SwitcherLayout {
            overlay_rect: Rect::new(work_area.x, work_area.y, 0, 0),
            cols: 0,
            rows: 0,
            cards: Vec::new(),
            total_pages: 0,
            current_page: 0,
            page_start: 0,
            page_indicator_rect: None,
        };
    }

    let margin = scale(MARGIN_LOGICAL, dpi);
    let gutter = scale(GUTTER_LOGICAL, dpi);
    let min_card_w = scale((172.0 * MIN_ASPECT).round() as i32 + 8, dpi);
    let content_width = (work_area.width - 2 * margin).max(min_card_w);
    let content_height = (work_area.height - 2 * margin).max(scale(CARD_H_LOGICAL, dpi));
    let card_h = scale(CARD_H_LOGICAL, dpi);
    let page_indicator_h = scale(24, dpi);

    // Row budget derived from work-area height (bounded 1..=MAX_ROWS)
    let max_rows = ((content_height - page_indicator_h + gutter) / (card_h + gutter))
        .clamp(1, MAX_ROWS as i32) as usize;

    let mut pages: Vec<PackedPage> = Vec::new();
    let mut current_rows: Vec<PackedRow> = Vec::new();
    let mut current_row_tiles: Vec<(usize, TileSpec)> = Vec::new();
    let mut current_row_w: i32 = 0;

    for (idx, &asp) in aspects.iter().enumerate() {
        let tile = TileSpec::for_aspect(asp, dpi);
        let card_w = tile.card_w;

        if current_row_tiles.is_empty() {
            current_row_tiles.push((idx, tile));
            current_row_w = card_w;
        } else if current_row_w + gutter + card_w <= content_width {
            current_row_tiles.push((idx, tile));
            current_row_w += gutter + card_w;
        } else {
            // Close current row
            current_rows.push(PackedRow {
                tiles: std::mem::take(&mut current_row_tiles),
                width: current_row_w,
            });
            if current_rows.len() == max_rows {
                pages.push(PackedPage {
                    rows: std::mem::take(&mut current_rows),
                });
            }
            current_row_tiles.push((idx, tile));
            current_row_w = card_w;
        }
    }

    if !current_row_tiles.is_empty() {
        current_rows.push(PackedRow {
            tiles: current_row_tiles,
            width: current_row_w,
        });
    }
    if !current_rows.is_empty() {
        pages.push(PackedPage { rows: current_rows });
    }

    let total_pages = pages.len().max(1);
    let current_page = page.min(total_pages.saturating_sub(1));

    let mut page_starts = Vec::with_capacity(total_pages);
    let mut current_start = 0;
    for p in &pages {
        page_starts.push(current_start);
        let count: usize = p.rows.iter().map(|r| r.tiles.len()).sum();
        current_start += count;
    }
    let page_start = page_starts.get(current_page).copied().unwrap_or(0);

    let page_data = &pages[current_page];

    let rows = page_data.rows.len();
    let cols = page_data
        .rows
        .iter()
        .map(|r| r.tiles.len())
        .max()
        .unwrap_or(0);
    let page_content_w = page_data.rows.iter().map(|r| r.width).max().unwrap_or(0);
    let page_content_h = rows as i32 * card_h + (rows as i32 - 1).max(0) * gutter;
    let indicator_h = if total_pages > 1 { page_indicator_h } else { 0 };

    let overlay_w = page_content_w + 2 * margin;
    let overlay_h = page_content_h + 2 * margin + indicator_h;

    let overlay_x = work_area.x + (work_area.width - overlay_w).max(0) / 2;
    let overlay_y = work_area.y + (work_area.height - overlay_h).max(0) / 2;

    let mut cards = Vec::new();
    for (row_idx, row) in page_data.rows.iter().enumerate() {
        let row_y = overlay_y + margin + row_idx as i32 * (card_h + gutter);
        let mut card_x = overlay_x + margin;
        for &(_cand_idx, tile) in &row.tiles {
            let chrome_rect = Rect::new(card_x, row_y, tile.card_w, tile.card_h);
            let header_rect = Rect::new(
                card_x + scale(6, dpi),
                row_y + scale(4, dpi),
                tile.card_w - scale(12, dpi),
                tile.header_h,
            );
            let preview_rect = Rect::new(
                card_x + scale(4, dpi),
                row_y + tile.header_h + scale(4, dpi),
                tile.preview_w,
                tile.preview_h,
            );
            cards.push(CardLayout {
                chrome_rect,
                header_rect,
                preview_rect,
            });
            card_x += tile.card_w + gutter;
        }
    }

    let page_indicator_rect = if total_pages > 1 {
        Some(Rect::new(
            overlay_x + margin,
            overlay_y + margin + page_content_h,
            page_content_w,
            indicator_h,
        ))
    } else {
        None
    };

    SwitcherLayout {
        overlay_rect: Rect::new(overlay_x, overlay_y, overlay_w, overlay_h),
        cols,
        rows,
        cards,
        total_pages,
        current_page,
        page_start,
        page_indicator_rect,
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn tile_width_follows_the_source_window_aspect_ratio() {
        let work_area = Rect::new(0, 0, 1920, 1080);
        // 4:3 (1.333), 16:9 (1.778), 21:9 (2.333)
        let aspects = vec![4.0 / 3.0, 16.0 / 9.0, 21.0 / 9.0];
        let layout = compute_layout(work_area, &aspects, 0, 96);

        assert_eq!(layout.cards.len(), 3);
        // All share uniform height (CARD_H_LOGICAL = 212)
        assert_eq!(layout.cards[0].chrome_rect.height, CARD_H_LOGICAL);
        assert_eq!(layout.cards[1].chrome_rect.height, CARD_H_LOGICAL);
        assert_eq!(layout.cards[2].chrome_rect.height, CARD_H_LOGICAL);

        // Width increases with aspect ratio
        let w_4_3 = layout.cards[0].chrome_rect.width;
        let w_16_9 = layout.cards[1].chrome_rect.width;
        let w_21_9 = layout.cards[2].chrome_rect.width;

        assert!(
            w_16_9 > w_4_3,
            "16:9 card ({w_16_9}) must be wider than 4:3 card ({w_4_3})"
        );
        assert!(
            w_21_9 > w_16_9,
            "21:9 card ({w_21_9}) must be wider than 16:9 card ({w_16_9})"
        );
    }

    #[test]
    fn every_card_on_a_page_shares_the_scaled_card_height() {
        let work_area = Rect::new(0, 0, 1920, 1080);
        let aspects = vec![0.75, 1.25, 1.777, 2.333, 1.0];
        let dpi = 144; // 150%
        let layout = compute_layout(work_area, &aspects, 0, dpi);
        let expected_h = scale(CARD_H_LOGICAL, dpi);

        for (i, card) in layout.cards.iter().enumerate() {
            assert_eq!(
                card.chrome_rect.height, expected_h,
                "card {i} height must equal scaled CARD_H"
            );
        }
    }

    #[test]
    fn an_extreme_aspect_ratio_is_clamped_into_band() {
        // Ultrawide (3.56:1) clamped to MAX_ASPECT (2.40)
        let ultrawide_aspect = aspect_from_rect(Some(Rect::new(0, 0, 3560, 1000)));
        assert_eq!(ultrawide_aspect, MAX_ASPECT);

        // Tall portrait (0.50:1) clamped to MIN_ASPECT (0.60)
        let portrait_aspect = aspect_from_rect(Some(Rect::new(0, 0, 500, 1000)));
        assert_eq!(portrait_aspect, MIN_ASPECT);
    }

    #[test]
    fn an_unreadable_or_zero_height_rect_falls_back_to_sixteen_by_nine() {
        assert_eq!(aspect_from_rect(None), DEFAULT_ASPECT);
        assert_eq!(
            aspect_from_rect(Some(Rect::new(0, 0, 1000, 0))),
            DEFAULT_ASPECT
        );
        assert_eq!(
            aspect_from_rect(Some(Rect::new(0, 0, 0, 1000))),
            DEFAULT_ASPECT
        );
    }

    #[test]
    fn no_packed_row_ever_exceeds_the_content_width() {
        // Swept across widths [1366, 1920, 2560, 3840] and heights [768, 800, 1080] with mixed aspects
        let mixed_aspects = vec![
            1.778, 1.333, 2.333, 0.8, 1.6, 2.0, 1.778, 1.0, 1.5, 2.2, 1.778, 1.2, 1.8, 2.1, 1.333,
            1.778,
        ];

        for width in [1366, 1920, 2560, 3840] {
            for height in [768, 800, 1080] {
                let work_area = Rect::new(0, 0, width, height);
                let layout = compute_layout(work_area, &mixed_aspects, 0, 96);

                assert!(
                    layout.overlay_rect.width <= work_area.width,
                    "overlay width {} exceeds work area width {}",
                    layout.overlay_rect.width,
                    work_area.width
                );
                assert!(
                    layout.overlay_rect.height <= work_area.height,
                    "overlay height {} exceeds work area height {}",
                    layout.overlay_rect.height,
                    work_area.height
                );
                assert!(
                    layout.overlay_rect.y >= work_area.y,
                    "overlay_y {} must never be above work_area.y {}",
                    layout.overlay_rect.y,
                    work_area.y
                );

                // Group cards by row (distinct Y) and verify row width
                let mut current_y = None;
                let mut row_w = 0;
                for card in &layout.cards {
                    if current_y == Some(card.chrome_rect.y) {
                        row_w += card.chrome_rect.width + scale(GUTTER_LOGICAL, 96);
                    } else {
                        current_y = Some(card.chrome_rect.y);
                        row_w = card.chrome_rect.width;
                    }
                    assert!(
                        row_w <= work_area.width - 2 * scale(MARGIN_LOGICAL, 96),
                        "row width {row_w} exceeds content width"
                    );
                }
            }
        }
    }

    #[test]
    fn rows_derive_from_work_area_height_on_short_displays() {
        // Height 600 only has room for 1 or 2 rows
        let work_area_short = Rect::new(0, 0, 1920, 600);
        let aspects = vec![1.778; 15];
        let layout_short = compute_layout(work_area_short, &aspects, 0, 96);
        assert!(
            layout_short.rows <= 2,
            "short display must derive fewer rows (got {})",
            layout_short.rows
        );
        assert!(layout_short.overlay_rect.height <= work_area_short.height);
    }

    #[test]
    fn row_count_is_never_zero() {
        // Even on an impossibly small display, row count is at least 1
        let tiny_work_area = Rect::new(0, 0, 100, 100);
        let aspects = vec![1.778];
        let layout = compute_layout(tiny_work_area, &aspects, 0, 96);
        assert_eq!(layout.rows, 1);
        assert_eq!(layout.cols, 1);
    }

    #[test]
    fn total_pages_comes_from_packing_and_no_page_is_empty() {
        let work_area = Rect::new(0, 0, 1920, 1080);
        let aspects = vec![1.778; 25];
        let layout = compute_layout(work_area, &aspects, 0, 96);
        assert!(layout.total_pages >= 2);

        for p in 0..layout.total_pages {
            let page_layout = compute_layout(work_area, &aspects, p, 96);
            assert!(!page_layout.cards.is_empty(), "page {p} must not be empty");
        }
    }

    #[test]
    fn geometry_scales_with_monitor_dpi() {
        let work_area = Rect::new(0, 0, 3840, 2160);
        let aspects = vec![16.0 / 9.0];

        // 100% (96 DPI)
        let layout_100 = compute_layout(work_area, &aspects, 0, 96);
        assert_eq!(layout_100.cards[0].chrome_rect.width, 314);
        assert_eq!(layout_100.cards[0].chrome_rect.height, 212);

        // 200% (192 DPI)
        let layout_200 = compute_layout(work_area, &aspects, 0, 192);
        assert_eq!(layout_200.cards[0].chrome_rect.width, 628);
        assert_eq!(layout_200.cards[0].chrome_rect.height, 424);
    }

    #[test]
    fn candidates_beyond_one_page_paginate_rather_than_shrink() {
        let work_area = Rect::new(0, 0, 1920, 1080);
        let aspects_6 = vec![1.778; 6];
        let aspects_30 = vec![1.778; 30];

        let layout_small = compute_layout(work_area, &aspects_6, 0, 96);
        let layout_large = compute_layout(work_area, &aspects_30, 0, 96);

        // Height remains the invariant CARD_H_LOGICAL instead of shrinking
        assert_eq!(layout_small.cards[0].chrome_rect.height, CARD_H_LOGICAL);
        assert_eq!(layout_large.cards[0].chrome_rect.height, CARD_H_LOGICAL);
        assert!(layout_large.total_pages > 1);
    }
}
