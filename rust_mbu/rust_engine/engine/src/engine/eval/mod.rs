pub mod activity;
pub mod evaluation_table;
pub mod king_safety;
pub mod pawns;
pub mod pin_bonus;
pub mod positional_tables;
pub mod rooks;

use std::sync::Arc;

use sdk::position::{Color, Position};

pub trait Evaluate {
    fn evaluate(&self, position: &Position) -> f64;
}

use self::{
    activity::bonus_for_mobility,
    evaluation_table::EvaluationTable,
    king_safety::{bonus_for_pieces_close_to_king, calc_king_safety},
    pawns::{
        isolated::isolated_pawns,
        protected_passed_pawnes::passed_pawns,
        stacked::stacked_pawns,
        strong_squares::{
            bonus as bonus_for_strong_squares, bonus_for_piece as bonus_for_piece_on_strong_squares,
        },
    },
    pin_bonus::bonus_for_absolute_pins,
    positional_tables::{game_phase, tapered_eval},
    rooks::{
        battery::bonus_for_rook_batteries,
        rook_on_open_files::{bonus_rook_for_open_files, bonus_rook_for_semi_open_files},
    },
};

pub const PIECE_VALUES: [i32; 6] = [100, 300, 320, 500, 900, 10000];

#[must_use]
pub fn evaluate(position: &Position, eval_table: &Arc<EvaluationTable>) -> i32 {
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
    //let phase_factor = f64::from(phase) / 24.0;

    score += calc_king_safety(position);
    score += isolated_pawns(position);
    score += stacked_pawns(position);
    score += passed_pawns(position);
    score += bonus_for_strong_squares(position);
    score += bonus_for_piece_on_strong_squares(position);
    score += bonus_rook_for_open_files(position);
    score += bonus_rook_for_semi_open_files(position);
    score += bonus_for_rook_batteries(position);
    score += bonus_for_absolute_pins(position);
    score += bonus_for_mobility(position);
    //score += (f64::from(bonus_for_pieces_close_to_king(position)) * phase_factor) as i32;

    let final_score = score * side_multiplier;

    eval_table.write(position.hash, final_score);

    final_score
}

#[must_use]
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
