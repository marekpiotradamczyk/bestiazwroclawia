pub mod activity;
pub mod evaluation_table;
pub mod king_safety;
pub mod pawns;
pub mod pin_bonus;
pub mod positional_tables;
pub mod rooks;

use std::sync::Arc;

use move_gen::generators::movegen::MoveGen;
use sdk::position::{Color, Position};

pub trait Evaluate {
    fn evaluate(&self, position: &Position) -> f64;
}

use self::{
    activity::bonus_for_mobility,
    evaluation_table::EvaluationTable,
    king_safety::calc_king_safety,
    pawns::{
        isolated_pawns::penalty_for_isolated_pawns,
        protected_passed_pawnes::bonus_for_protected_passed_pawnes,
        stacked_pawns::penalty_for_stacked_pawns,
        strong_squares::{bonus_for_piece_on_strong_squares, bonus_for_strong_squares},
    },
    pin_bonus::bonus_for_absolute_pins,
    positional_tables::{game_phase, tapered_eval},
    rooks::{
        battery::bonus_for_rook_battery,
        rook_on_open_files::{bonus_rook_for_open_files, bonus_rook_for_semi_open_files},
    },
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

    let phase = game_phase(position);
    let mut score = tapered_eval(position, phase);
    //let score = material(position);

    score += calc_king_safety(position, move_gen.clone());
    score += penalty_for_isolated_pawns(position);
    score += penalty_for_stacked_pawns(position);
    score += bonus_for_protected_passed_pawnes(position);
    score += bonus_for_strong_squares(position);
    score += bonus_for_piece_on_strong_squares(position);
    score += bonus_rook_for_open_files(position);
    score += bonus_rook_for_semi_open_files(position);
    score += bonus_for_rook_battery(position);
    score += bonus_for_absolute_pins(position, move_gen.clone());
    score += bonus_for_mobility(position);

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

        score += PIECE_VALUES[piece] * (white_count - black_count);

        piece += 1;
    }

    score
}
