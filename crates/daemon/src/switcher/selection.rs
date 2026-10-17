//! Selection navigation across cards and pages in cycle_order.
//! Backward-compatible facade re-exporting from `wiradesk_core::switcher::selection`.

pub use wiradesk_core::switcher::selection::*;

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::switcher::layout::CardLayout;
    use crate::switcher::layout::Rect;

    #[test]
    fn advancing_past_a_page_edge_turns_the_page() {
        let candidate_count = 14;
        let per_page = 6;

        let (idx, page) = select_next(candidate_count, 5, per_page);
        assert_eq!(idx, 6);
        assert_eq!(page, 1);

        let (prev_idx, prev_page) = select_prev(candidate_count, 6, per_page);
        assert_eq!(prev_idx, 5);
        assert_eq!(prev_page, 0);
    }

    #[test]
    fn up_and_down_clamp_within_page() {
        let make_card = |x: i32, y: i32| CardLayout {
            chrome_rect: Rect::new(x, y, 200, 212),
            header_rect: Rect::new(x + 6, y + 4, 188, 32),
            preview_rect: Rect::new(x + 4, y + 36, 192, 172),
        };
        let cards = vec![
            make_card(100, 100),
            make_card(320, 100),
            make_card(540, 100),
            make_card(100, 330),
            make_card(320, 330),
            make_card(540, 330),
        ];

        let up_idx = select_up(6, 4, &cards, 0);
        assert_eq!(up_idx, 1);

        let up_clamp_idx = select_up(6, 1, &cards, 0);
        assert_eq!(up_clamp_idx, 1);
    }

    #[test]
    fn up_and_down_pick_the_greatest_horizontal_overlap_across_ragged_rows() {
        let make_card = |x: i32, y: i32, w: i32| CardLayout {
            chrome_rect: Rect::new(x, y, w, 212),
            header_rect: Rect::new(x + 6, y + 4, w - 12, 32),
            preview_rect: Rect::new(x + 4, y + 36, w - 8, 172),
        };

        let cards = vec![
            make_card(100, 100, 400),
            make_card(520, 100, 300),
            make_card(100, 330, 200),
            make_card(320, 330, 300),
            make_card(640, 330, 200),
        ];

        let up_3 = select_up(5, 3, &cards, 0);
        assert_eq!(up_3, 0);

        let up_4 = select_up(5, 4, &cards, 0);
        assert_eq!(up_4, 1);

        let down_0 = select_down(5, 0, &cards, 0);
        assert_eq!(down_0, 2);

        let down_1 = select_down(5, 1, &cards, 0);
        assert_eq!(down_1, 4);
    }
}
