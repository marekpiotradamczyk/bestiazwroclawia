use move_gen::r#move::{Move, MoveKind};

pub const LMR_MIN_MOVES: usize = 4;
pub const LMR_LIMIT: usize = 3;

#[must_use]
pub fn is_lmr_applicable(
    mv: &Move,
    depth: usize,
    moves_tried: usize,
    in_check: bool,
    gives_check: bool,
    pv_node: bool,
    extend: usize,
) -> bool {
    depth >= LMR_LIMIT
        && extend == 0
        && moves_tried >= LMR_MIN_MOVES
        && !in_check
        && !gives_check
        && !pv_node
        && !matches!(
            mv.kind(),
            MoveKind::Capture | MoveKind::PromotionCapture | MoveKind::Promotion
        )
}
