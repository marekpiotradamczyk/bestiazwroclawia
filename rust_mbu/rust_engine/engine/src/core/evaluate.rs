use sdk::position::{Color, Piece, Position};

use super::Engine;

pub trait Evaluate {
    fn evaluate(&self, position: &Position) -> f64;
}

const PIECE_VALUES: [f64; 6] = [1.0, 3.0, 3.0, 5.0, 9.0, 100.0];

impl Evaluate for Engine {
    fn evaluate(&self, position: &Position) -> f64 {
        let mut score = 0.0;
        for piece in Piece::all() {
            let white_count = position.pieces[Color::White as usize][piece as usize].count() as f64;
            let black_count = position.pieces[Color::Black as usize][piece as usize].count() as f64;

            score += PIECE_VALUES[piece as usize] * (white_count - black_count);
        }

        score
    }
}
