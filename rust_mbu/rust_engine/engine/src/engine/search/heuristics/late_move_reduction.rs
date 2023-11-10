use move_gen::r#move::{Move, MoveKind};

pub const LMR_MIN_MOVES: usize = 4;
pub const LMR_LIMIT: usize = 3;

pub fn is_lmr_applicable(mv: &Move, depth: usize, moves_tried: usize, in_check: bool) -> bool {
    depth >= LMR_LIMIT
        && moves_tried >= LMR_MIN_MOVES
        && !in_check
        && !matches!(
            mv.kind(),
            MoveKind::Capture | MoveKind::PromotionCapture | MoveKind::Promotion
        )
}
