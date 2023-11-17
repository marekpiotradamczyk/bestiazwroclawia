use sdk::position::Position;

use crate::engine::search::parallel::SearchData;

impl SearchData {
    #[allow(clippy::too_many_arguments)]
    /// Returns `(new_score, fails_low)`
    pub fn razoring(
        &mut self,
        node: &Position,
        static_eval: i32,
        alpha: i32,
        beta: i32,
        depth: usize,
        in_check: bool,
        pv_node: bool,
    ) -> Option<(i32, bool)> {
        if depth > 3 || in_check || pv_node {
            return None;
        }

        let mut val = static_eval + 125;

        if val < beta {
            if depth == 1 {
                let new_score = self.quiesce(node, alpha, beta);

                return Some((i32::max(new_score, val), true));
            }

            val += 175;

            if val < beta && depth <= 2 {
                let new_score = self.quiesce(node, alpha, beta);

                return Some((i32::max(new_score, val), true));
            }
        }

        Some((val, false))
    }
}
