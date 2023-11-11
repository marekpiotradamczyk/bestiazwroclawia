use move_gen::r#move::MakeMove;
use sdk::position::Position;

use crate::engine::search::Search;

impl Search {
    /// Checks if after giving a free move to opponent, the score is still so good that it exceeds
    /// beta. If so,
    /// [Source](https://web.archive.org/web/20071031095933/http://www.brucemo.com/compchess/programming/nullmove.htm)
    pub fn null_move_reduction(
        &mut self,
        node: &Position,
        beta: isize,
        depth: usize,
        in_check: bool,
        ply: usize,
    ) -> bool {
        if !is_null_move_reduction_applicable(node, depth, in_check, ply) {
            return false;
        }

        let child = {
            let mut child = node.clone();
            child.make_null_move();
            child
        };

        self.repetition_table.push(&child);
        self.ply += 1;
        let score = -self.negamax(&child, -beta, -beta + 1, depth - 3);
        self.ply -= 1;
        self.repetition_table.decrement();

        score >= beta
    }
}

pub fn is_null_move_reduction_applicable(
    pos: &Position,
    depth: usize,
    in_check: bool,
    ply: usize,
) -> bool {
    pos.occupied.count() > 10 && depth >= 3 && !in_check && ply > 0
}
