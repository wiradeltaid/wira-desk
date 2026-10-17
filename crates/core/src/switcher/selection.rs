//! Selection navigation across cards and pages in cycle_order.

use super::layout::CardLayout;

/// Move selection forward by one candidate.
/// Wrapping past the end returns to 0 on page 0.
pub fn select_next(
    candidate_count: usize,
    current_index: usize,
    per_page: usize,
) -> (usize, usize) {
    if candidate_count == 0 {
        return (0, 0);
    }
    let next_index = (current_index + 1) % candidate_count;
    let next_page = next_index.checked_div(per_page).unwrap_or(0);
    (next_index, next_page)
}

/// Move selection backward by one candidate.
/// Wrapping before 0 goes to last candidate and its corresponding page.
pub fn select_prev(
    candidate_count: usize,
    current_index: usize,
    per_page: usize,
) -> (usize, usize) {
    if candidate_count == 0 {
        return (0, 0);
    }
    let prev_index = if current_index == 0 {
        candidate_count - 1
    } else {
        current_index - 1
    };
    let prev_page = prev_index.checked_div(per_page).unwrap_or(0);
    (prev_index, prev_page)
}

/// Move selection up by one row within the current page, picking the card in the previous
/// row with the greatest horizontal overlap with the current card (DEC-026, SPEC-14-06).
/// Clamps at the top row.
pub fn select_up(
    candidate_count: usize,
    current_index: usize,
    cards: &[CardLayout],
    page_start: usize,
) -> usize {
    if candidate_count == 0 || cards.is_empty() {
        return current_index;
    }
    let in_page = current_index.saturating_sub(page_start);

    if in_page >= cards.len() {
        return current_index;
    }

    let mut rows: Vec<Vec<usize>> = Vec::new();
    let mut last_y = None;
    for (i, card) in cards.iter().enumerate() {
        if last_y == Some(card.chrome_rect.y) {
            rows.last_mut().unwrap().push(i);
        } else {
            last_y = Some(card.chrome_rect.y);
            rows.push(vec![i]);
        }
    }

    let cur_row_idx = rows.iter().position(|r| r.contains(&in_page)).unwrap_or(0);
    if cur_row_idx == 0 {
        return current_index;
    }

    let prev_row = &rows[cur_row_idx - 1];
    let cur_card = &cards[in_page];
    let cur_left = cur_card.chrome_rect.x;
    let cur_right = cur_left + cur_card.chrome_rect.width;
    let cur_center = cur_left + cur_card.chrome_rect.width / 2;

    let mut best_cand = prev_row[0];
    let mut best_overlap = -1;
    let mut best_dist = i32::MAX;

    for &cand in prev_row {
        let card = &cards[cand];
        let left = card.chrome_rect.x;
        let right = left + card.chrome_rect.width;
        let center = left + card.chrome_rect.width / 2;

        let overlap = (cur_right.min(right) - cur_left.max(left)).max(0);
        let dist = (cur_center - center).abs();

        if overlap > best_overlap || (overlap == best_overlap && dist < best_dist) {
            best_overlap = overlap;
            best_dist = dist;
            best_cand = cand;
        }
    }

    page_start + best_cand
}

/// Move selection down by one row within the current page, picking the card in the next
/// row with the greatest horizontal overlap with the current card (DEC-026, SPEC-14-06).
/// Clamps at the bottom row.
pub fn select_down(
    candidate_count: usize,
    current_index: usize,
    cards: &[CardLayout],
    page_start: usize,
) -> usize {
    if candidate_count == 0 || cards.is_empty() {
        return current_index;
    }
    let in_page = current_index.saturating_sub(page_start);

    if in_page >= cards.len() {
        return current_index;
    }

    let mut rows: Vec<Vec<usize>> = Vec::new();
    let mut last_y = None;
    for (i, card) in cards.iter().enumerate() {
        if last_y == Some(card.chrome_rect.y) {
            rows.last_mut().unwrap().push(i);
        } else {
            last_y = Some(card.chrome_rect.y);
            rows.push(vec![i]);
        }
    }

    let cur_row_idx = rows.iter().position(|r| r.contains(&in_page)).unwrap_or(0);
    if cur_row_idx + 1 >= rows.len() {
        return current_index;
    }

    let next_row = &rows[cur_row_idx + 1];
    let cur_card = &cards[in_page];
    let cur_left = cur_card.chrome_rect.x;
    let cur_right = cur_left + cur_card.chrome_rect.width;
    let cur_center = cur_left + cur_card.chrome_rect.width / 2;

    let mut best_cand = next_row[0];
    let mut best_overlap = -1;
    let mut best_dist = i32::MAX;

    for &cand in next_row {
        let card = &cards[cand];
        let left = card.chrome_rect.x;
        let right = left + card.chrome_rect.width;
        let center = left + card.chrome_rect.width / 2;

        let overlap = (cur_right.min(right) - cur_left.max(left)).max(0);
        let dist = (cur_center - center).abs();

        if overlap > best_overlap || (overlap == best_overlap && dist < best_dist) {
            best_overlap = overlap;
            best_dist = dist;
            best_cand = cand;
        }
    }

    page_start + best_cand
}

#[cfg(test)]
pub mod tests {
    use super::super::layout::Rect;
    use super::*;

    #[test]
    pub fn test_card_selection_navigation() {
        let candidate_count = 10;
        let per_page = 5;
        let (idx, page) = select_next(candidate_count, 0, per_page);
        assert_eq!(idx, 1);
        assert_eq!(page, 0);
        let (prev_idx, prev_page) = select_prev(candidate_count, 0, per_page);
        assert_eq!(prev_idx, 9);
        assert_eq!(prev_page, 1);
    }

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
