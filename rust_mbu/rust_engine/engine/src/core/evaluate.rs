use sdk::position::{Color, Piece, Position};

pub trait Evaluate {
    fn evaluate(&self, position: &Position) -> f64;
}

const PIECE_VALUES: [f64; 6] = [100.0, 300.0, 350.0, 500.0, 900.0, 10000.0];

pub fn evaluate(position: &Position) -> f64 {
    let mut score = 0.0;
    for piece in Piece::all() {
        let white_count = position.pieces[Color::White as usize][piece as usize].count() as f64;
        let black_count = position.pieces[Color::Black as usize][piece as usize].count() as f64;

        score += PIECE_VALUES[piece as usize] * (white_count - black_count);
    }

    if position.turn == Color::White {
        score
    } else {
        -score
    }
}
