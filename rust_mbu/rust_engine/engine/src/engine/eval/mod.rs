pub mod evaluation_table;
pub mod king_safety;
pub mod pawns;
pub mod positional_tables;
pub mod rooks;

use std::sync::Arc;

use move_gen::generators::movegen::MoveGen;
use sdk::position::{Color, Position};

pub trait Evaluate {
    fn evaluate(&self, position: &Position) -> f64;
}

use self::{
    evaluation_table::EvaluationTable,
    king_safety::calc_king_safety,
    pawns::{isolated_pawns::penalty_for_isolated_pawns, stacked_pawns::penalty_for_stacked_pawns}, rooks::rook_on_open_files::{bonus_rook_for_open_files, bonus_rook_for_semi_open_files}, positional_tables::tapered_eval,
};

pub const PIECE_VALUES: [i32; 6] = [100, 300, 320, 500, 900, 10000];

pub fn evaluate(
    position: &Position,
    eval_table: Arc<EvaluationTable>,
    move_gen: Arc<MoveGen>,
) -> i32 {
    if let Some(value) = eval_table.read(position.hash) {
        return value;
    }

    let side_multiplier = if matches!(position.turn, Color::White) {
        1
    } else {
        -1
    };

    let mut score = tapered_eval(position);

    score += calc_king_safety(position, move_gen.clone());
    score += penalty_for_isolated_pawns(position);
    score += penalty_for_stacked_pawns(position);
    score += bonus_rook_for_open_files(position);
    score += bonus_rook_for_semi_open_files(position);
    /*
    let mut sq = 0;

    while sq < 64 {
        let square = Square::all()[sq];
        let piece = position.piece_at(&square);

        if let Some((piece, color)) = piece {
            score += PIECE_TABLES[color as usize][piece as usize][sq] * side_multiplier;
        }

        sq += 1;
    } */

    let final_score = score * side_multiplier;

    eval_table.write(position.hash, final_score);

    final_score
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
