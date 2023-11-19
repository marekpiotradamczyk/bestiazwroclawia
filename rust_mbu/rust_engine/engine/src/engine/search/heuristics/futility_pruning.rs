use move_gen::r#move::Move;
use sdk::position::{Position, Piece, Color};

pub const FUTILITY_MARGIN: i32 = 200;

#[allow(clippy::too_many_arguments)]
pub fn is_futile(
    mv: &Move,
    pos: &Position,
    depth: usize,
    alpha: i32,
    beta: i32,
    is_capture: bool,
    in_check: bool,
    gives_check: bool,
    static_eval: i32,
    moves_tried: usize,
) -> bool {
    if in_check
        || moves_tried <= 1
        || is_capture
        || gives_check
        || alpha.abs() > 10000
        || beta.abs() > 10000
        || depth > 6
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

    static_eval + FUTILITY_MARGIN * depth as i32 <= alpha
}
