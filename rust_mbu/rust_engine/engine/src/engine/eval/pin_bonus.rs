use sdk::position::{Color, Position};

use crate::engine::MOVE_GEN;

pub const PINNED_PIECE_BONUS: i32 = 10;

#[must_use]
pub fn bonus_for_absolute_pins(pos: &Position) -> i32 {
    let white_pinned_count = i32::from(MOVE_GEN.pinned_pieces(pos, Color::White).count());
    let black_pinned_count = i32::from(MOVE_GEN.pinned_pieces(pos, Color::Black).count());

    (black_pinned_count - white_pinned_count) * PINNED_PIECE_BONUS
}

#[cfg(test)]
mod tests {
    use sdk::position::tests::*;

    use crate::engine::eval::pin_bonus::PINNED_PIECE_BONUS;

    #[test]
    fn test_pinned_piece_bonus() {
        #[rustfmt::skip]
        let board = [
                     0, 0, k, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0, q,
                     0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, P, 0, 0,
                     0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, K, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0, 0
                    ];
        let pos = test_board(&board);
        assert_eq!(super::bonus_for_absolute_pins(&pos), -PINNED_PIECE_BONUS);
    }
}
