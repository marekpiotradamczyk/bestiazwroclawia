use sdk::position::{Color, Piece, Position};

pub trait Evaluate {
    fn evaluate(&self, position: &Position) -> f64;
}

pub const PIECE_VALUES: [i32; 6] = [100, 300, 350, 500, 900, 10000];

pub fn evaluate(position: &Position) -> i32 {
    let mut score = 0;
    for piece in Piece::all() {
        let white_count = position.pieces[Color::White as usize][piece as usize].count() as i32;
        let black_count = position.pieces[Color::Black as usize][piece as usize].count() as i32;

        score += PIECE_VALUES[piece as usize] * (white_count - black_count);
    }

    if position.turn == Color::White {
        score
    } else {
        -score
    }
}
