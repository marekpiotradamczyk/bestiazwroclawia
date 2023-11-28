use crate::{
    bitboard::{Bitboard, Direction},
    position::Color,
    square::Square,
};

use super::king::mask_king_attacks;

pub fn squares_near_king(king_sq: Square, king_color: Color) -> Bitboard {
    let mut squares = mask_king_attacks(king_sq.bitboard());

    let direction = if king_color == Color::White {
        Direction::South
    } else {
        Direction::North
    };

    squares |= squares.shift(&direction);

    squares
}

pub fn generate_squares_near_king() -> [[Bitboard; 64]; 2] {
    let mut squares = [[Bitboard(0); 64]; 2];

    for king_sq in Square::all() {
        for king_color in Color::all() {
            squares[king_color as usize][king_sq as usize] = squares_near_king(king_sq, king_color);
        }
    }

    squares
}
