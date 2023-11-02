use sdk::position::{Color, Piece, Position};

pub trait Evaluate {
    fn evaluate(&self, position: &Position) -> f64;
}

const PIECE_VALUES: [isize; 6] = [100, 300, 350, 500, 900, 10000];

pub fn evaluate(position: &Position) -> isize {
    let mut score = 0;
    for piece in Piece::all() {
        let white_count = position.pieces[Color::White as usize][piece as usize].count() as isize;
        let black_count = position.pieces[Color::Black as usize][piece as usize].count() as isize;

        score += PIECE_VALUES[piece as usize] * (white_count - black_count);
    }

    if position.turn == Color::White {
        score
    } else {
        -score
    }
}
