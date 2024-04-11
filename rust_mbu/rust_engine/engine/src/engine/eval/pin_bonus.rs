use sdk::position::{Color, Position};

use crate::engine::MOVE_GEN;

pub const PINNED_PIECE_BONUS: i32 = 10;

#[must_use]
pub fn bonus_for_absolute_pins(pos: &Position) -> i32 {
    let white_pinned_count = i32::from(MOVE_GEN.pinned_pieces(pos, Color::White).count());
    let black_pinned_count = i32::from(MOVE_GEN.pinned_pieces(pos, Color::Black).count());

    (black_pinned_count - white_pinned_count) * PINNED_PIECE_BONUS
}
