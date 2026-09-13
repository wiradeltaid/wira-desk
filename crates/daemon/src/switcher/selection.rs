//! Selection navigation across cards and pages in cycle_order.

use crate::switcher::layout::CardLayout;

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
    use super::*;
    use crate::switcher::layout::Rect;

    #[test]
    fn advancing_past_a_page_edge_turns_the_page() {
        let candidate_count = 14;
        let per_page = 6;

        // Start at last card of page 0 (index 5)
        let (idx, page) = select_next(candidate_count, 5, per_page);
        assert_eq!(idx, 6);
        assert_eq!(page, 1, "moving past page 0 edge turns to page 1");

        // Start at card 0 of page 1 (index 6) and move prev
        let (prev_idx, prev_page) = select_prev(candidate_count, 6, per_page);
        assert_eq!(prev_idx, 5);
        assert_eq!(
            prev_page, 0,
            "moving back past page 1 edge returns to page 0"
        );
    }

    #[test]
    fn up_and_down_clamp_within_page() {
        let make_card = |x: i32, y: i32| CardLayout {
            chrome_rect: Rect::new(x, y, 200, 212),
            header_rect: Rect::new(x + 6, y + 4, 188, 32),
            preview_rect: Rect::new(x + 4, y + 36, 192, 172),
        };
        // 6 cards: row 0 (y=100) has 0, 1, 2; row 1 (y=330) has 3, 4, 5
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
        // Row 0: 2 cards
        // Card 0: x: 100, width: 400 (x: 100..500)
        // Card 1: x: 520, width: 300 (x: 520..820)
        // Row 1: 3 cards
        // Card 2: x: 100, width: 200 (x: 100..300)
        // Card 3: x: 320, width: 300 (x: 320..620)
        // Card 4: x: 640, width: 200 (x: 640..840)
        let make_card = |x: i32, y: i32, w: i32| CardLayout {
            chrome_rect: Rect::new(x, y, w, 212),
            header_rect: Rect::new(x + 6, y + 4, w - 12, 32),
            preview_rect: Rect::new(x + 4, y + 36, w - 8, 172),
        };

        let cards = vec![
            make_card(100, 100, 400), // Card 0 (Row 0)
            make_card(520, 100, 300), // Card 1 (Row 0)
            make_card(100, 330, 200), // Card 2 (Row 1)
            make_card(320, 330, 300), // Card 3 (Row 1)
            make_card(640, 330, 200), // Card 4 (Row 1)
        ];

        // Navigating UP from Card 3 (x: 320..620):
        // Card 0 (x: 100..500) overlap is 320..500 = 180px.
        // Card 1 (x: 520..820) overlap is 520..620 = 100px.
        // Card 0 has greater overlap (180 > 100), so Up from 3 selects Card 0!
        let up_3 = select_up(5, 3, &cards, 0);
        assert_eq!(
            up_3, 0,
            "Up from Card 3 must select Card 0 (180px overlap vs 100px)"
        );

        // Navigating UP from Card 4 (x: 640..840):
        // Card 0 (x: 100..500) overlap = 0.
        // Card 1 (x: 520..820) overlap = 640..820 = 180px.
        // Selects Card 1!
        let up_4 = select_up(5, 4, &cards, 0);
        assert_eq!(up_4, 1, "Up from Card 4 must select Card 1");

        // Navigating DOWN from Card 0 (x: 100..500):
        // Card 2 (x: 100..300) overlap = 200px.
        // Card 3 (x: 320..620) overlap = 180px.
        // Card 4 (x: 640..840) overlap = 0px.
        // Card 2 has greatest overlap (200px > 180px), so Down from 0 selects Card 2!
        let down_0 = select_down(5, 0, &cards, 0);
        assert_eq!(
            down_0, 2,
            "Down from Card 0 must select Card 2 (200px overlap vs 180px)"
        );

        // Navigating DOWN from Card 1 (x: 520..820):
        // Card 2 overlap = 0.
        // Card 3 (x: 320..620) overlap = 520..620 = 100px.
        // Card 4 (x: 640..840) overlap = 640..820 = 180px.
        // Card 4 has greatest overlap (180px > 100px), so Down from 1 selects Card 4!
        let down_1 = select_down(5, 1, &cards, 0);
        assert_eq!(down_1, 4, "Down from Card 1 must select Card 4");
    }
}
