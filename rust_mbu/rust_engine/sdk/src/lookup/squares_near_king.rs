use crate::{
    bitboard::{Bitboard, Direction},
    position::Color,
    square::Square,
};

use super::king::mask_king_attacks;

#[must_use]
pub fn squares_near_king(king_sq: Square, king_color: Color) -> Bitboard {
    let mut squares = mask_king_attacks(king_sq.bitboard());

    let direction = if king_color == Color::White {
        Direction::North
    } else {
        Direction::South
    };

    squares |= squares.shift(&direction);

    squares
}

#[must_use]
pub fn generate_square_close_to_king() -> Vec<Vec<Bitboard>> {
    let mut squares = vec![vec![Bitboard(0); 64]; 2];

    for king_sq in Square::iter() {
        for king_color in Color::all() {
            squares[king_color as usize][king_sq as usize] = squares_near_king(king_sq, king_color);
        }
    }

    squares
}
