pub mod eval_tables;

use sdk::{
    position::{Color, Piece, Position},
    square::Square,
};

use self::eval_tables::PIECE_TABLES;

pub trait Evaluate {
    fn evaluate(&self, position: &Position) -> f64;
}

pub const PIECE_VALUES: [i32; 6] = [100, 300, 350, 500, 900, 10000];

pub const fn evaluate(position: &Position, alpha: i32, beta: i32) -> i32 {
    let mut score = material(position);
    let side_multiplier = if matches!(position.turn, Color::White) {
        1
    } else {
        -1
    };

    // Lazy evaluation cutoff
    /*
    let cutoff = PIECE_VALUES[Piece::Bishop as usize];
    
    if score + cutoff < alpha {
        return alpha;
    }
    if score - cutoff > beta {
        return beta;
    }
    */

    let mut sq = 0;
    while sq < 64 {
        let square = Square::all()[sq];
        let piece = position.piece_at(&square);

        if let Some((piece, color)) = piece {
            score += PIECE_TABLES[color as usize][piece as usize][sq] * side_multiplier;
        }

        sq += 1;
    }

    score * side_multiplier
}

pub const fn material(position: &Position) -> i32 {
    let mut score = 0;

    let mut piece = 0;
    while piece < 5 {
        let white_count = position.pieces[Color::White as usize][piece].count() as i32;
        let black_count = position.pieces[Color::Black as usize][piece].count() as i32;

        score += PIECE_VALUES[piece as usize] * (white_count - black_count);

        piece += 1;
    }

    score
}
