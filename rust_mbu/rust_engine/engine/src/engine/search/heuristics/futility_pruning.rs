pub const FUTILITY_MARGIN: i32 = 200;

#[allow(clippy::too_many_arguments)]
pub fn is_futile(
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

    static_eval + FUTILITY_MARGIN * depth as i32 <= alpha
}
