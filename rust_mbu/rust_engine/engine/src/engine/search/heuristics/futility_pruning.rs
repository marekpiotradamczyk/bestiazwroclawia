use move_gen::r#move::Move;
use sdk::position::{Color, Piece, Position};

use crate::engine::search::MATE_SCORE;

pub const FUTILITY_MARGIN: [i32; 16] = [
    0, 100, 150, 200, 250, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300,
];

pub const FUTILITY_DEPTH: usize = 6;

#[allow(clippy::too_many_arguments)]
pub fn is_futile(
    mv: &Move,
    pos: &Position,
    depth: usize,
    alpha: i32,
    beta: i32,
    pv_node: bool,
    is_capture: bool,
    in_check: bool,
    gives_check: bool,
    static_eval: i32,
    moves_tried: usize,
    extend: usize,
) -> bool {
    // Check, extended moves, captures, moves which gives check, positions with mate score and
    // positions above depth 6 shouldn't be pruned.
    if in_check
        || extend > 0
        || moves_tried <= 1
        || is_capture
        || gives_check
        || alpha.abs() > MATE_SCORE
        || beta.abs() > MATE_SCORE
        || depth > FUTILITY_DEPTH
        || pv_node
    {
        return false;
    }

    let piece = pos.piece_at(&mv.from()).unwrap().0;

    if piece == Piece::Pawn {
        let rank = mv.from().rank();
        if (rank as usize) >= 5 {
            return false;
        }
    }

    let white_pieces_without_king = pos.occupation(&Color::White).count_ones() - 1;
    let black_pieces_without_king = pos.occupation(&Color::Black).count_ones() - 1;

    if white_pieces_without_king == 0 || black_pieces_without_king == 0 {
        return false;
    }

    static_eval + FUTILITY_MARGIN[depth] <= alpha
}
