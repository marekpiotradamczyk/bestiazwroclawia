use sdk::position::Position;

use crate::engine::search::parallel::SearchData;

impl SearchData {
    #[allow(clippy::too_many_arguments)]
    /// If the static evaluation indicates a fail-low node, but q-search fails high, the score of the reduced fail-high search is returned, since there was obviously a winning capture raising the score, and one assumes a quiet move near the horizon will not do better
    /// Returns `new_score`
    pub fn razoring(
        &mut self,
        node: &Position,
        static_eval: i32,
        alpha: i32,
        beta: i32,
        depth: usize,
        in_check: bool,
        pv_node: bool,
    ) -> Option<i32> {
        if depth > 3 || in_check || pv_node {
            return None;
        }

        let mut val = static_eval + 125;

        if val < beta && depth <= 3 {
            let new_score = self.quiesce(node, alpha, beta);
            if depth == 1 {
                return Some(i32::max(new_score, val));
            }

            val += 175;

            if val < beta && depth <= 3 && new_score < beta {
                    return Some(i32::max(new_score, val));
                }
        }

        None
    }
}
