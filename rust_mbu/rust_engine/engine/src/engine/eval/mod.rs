pub mod eval_tables;

use sdk::{
    position::{Color, Position},
    square::Square,
};

use self::eval_tables::PIECE_TABLES;

pub trait Evaluate {
    fn evaluate(&self, position: &Position) -> f64;
}

pub const PIECE_VALUES: [i32; 6] = [100, 300, 350, 500, 900, 10000];

pub fn evaluate(position: &Position) -> i32 {
    let mut score = 0;
    for sq in Square::iter() {
        if let Some((piece, color)) = position.piece_at(&sq) {
            let piece_value = if color == Color::White {
                PIECE_VALUES[piece as usize] + PIECE_TABLES[piece as usize][sq as usize]
            } else {
                -PIECE_VALUES[piece as usize] - PIECE_TABLES[piece as usize][63 - (sq as usize)]
            };
            score += piece_value;
        }
    }

    if position.turn == Color::White {
        score
    } else {
        -score
    }
}
