//! Adaptive aspect-ratio tile layout and height-bounded row packing (SPEC-14-06, DEC-026).
//! Backward-compatible facade re-exporting from `wiradesk_core::switcher::layout`.

pub use wiradesk_core::switcher::layout::*;

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn tile_width_follows_the_source_window_aspect_ratio() {
        let work_area = Rect::new(0, 0, 1920, 1080);
        let aspects = vec![4.0 / 3.0, 16.0 / 9.0, 21.0 / 9.0];
        let layout = compute_layout(work_area, &aspects, 0, 96);

        assert_eq!(layout.cards.len(), 3);
        assert_eq!(layout.cards[0].chrome_rect.height, CARD_H_LOGICAL);
        assert_eq!(layout.cards[1].chrome_rect.height, CARD_H_LOGICAL);
        assert_eq!(layout.cards[2].chrome_rect.height, CARD_H_LOGICAL);

        let w_4_3 = layout.cards[0].chrome_rect.width;
        let w_16_9 = layout.cards[1].chrome_rect.width;
        let w_21_9 = layout.cards[2].chrome_rect.width;

        assert!(w_16_9 > w_4_3);
        assert!(w_21_9 > w_16_9);
    }

    #[test]
    fn every_card_on_a_page_shares_the_scaled_card_height() {
        let work_area = Rect::new(0, 0, 1920, 1080);
        let aspects = vec![0.75, 1.25, 1.777, 2.333, 1.0];
        let dpi = 144;
        let layout = compute_layout(work_area, &aspects, 0, dpi);
        let expected_h = scale(CARD_H_LOGICAL, dpi);

        for card in &layout.cards {
            assert_eq!(card.chrome_rect.height, expected_h);
        }
    }

    #[test]
    fn an_extreme_aspect_ratio_is_clamped_into_band() {
        let ultrawide_aspect = aspect_from_rect(Some(Rect::new(0, 0, 3560, 1000)));
        assert_eq!(ultrawide_aspect, MAX_ASPECT);

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
        let mixed_aspects = vec![
            1.778, 1.333, 2.333, 0.8, 1.6, 2.0, 1.778, 1.0, 1.5, 2.2, 1.778, 1.2, 1.8, 2.1, 1.333,
            1.778,
        ];

        for width in [1366, 1920, 2560, 3840] {
            for height in [768, 800, 1080] {
                let work_area = Rect::new(0, 0, width, height);
                let layout = compute_layout(work_area, &mixed_aspects, 0, 96);

                assert!(layout.overlay_rect.width <= work_area.width);
                assert!(layout.overlay_rect.height <= work_area.height);
                assert!(layout.overlay_rect.y >= work_area.y);
            }
        }
    }

    #[test]
    fn rows_derive_from_work_area_height_on_short_displays() {
        let work_area_short = Rect::new(0, 0, 1920, 600);
        let aspects = vec![1.778; 15];
        let layout_short = compute_layout(work_area_short, &aspects, 0, 96);
        assert!(layout_short.rows <= 2);
        assert!(layout_short.overlay_rect.height <= work_area_short.height);
    }

    #[test]
    fn row_count_is_never_zero() {
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
            assert!(!page_layout.cards.is_empty());
        }
    }

    #[test]
    fn geometry_scales_with_monitor_dpi() {
        let work_area = Rect::new(0, 0, 3840, 2160);
        let aspects = vec![16.0 / 9.0];

        let layout_100 = compute_layout(work_area, &aspects, 0, 96);
        assert_eq!(layout_100.cards[0].chrome_rect.width, 314);
        assert_eq!(layout_100.cards[0].chrome_rect.height, 212);

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

        assert_eq!(layout_small.cards[0].chrome_rect.height, CARD_H_LOGICAL);
        assert_eq!(layout_large.cards[0].chrome_rect.height, CARD_H_LOGICAL);
        assert!(layout_large.total_pages > 1);
    }
}
