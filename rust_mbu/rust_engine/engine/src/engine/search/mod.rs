use std::sync::{atomic::AtomicBool, Arc};

use itertools::Itertools;
use move_gen::r#move::{MakeMove, Move};
use sdk::position::Position;

pub mod heuristics;
pub mod parallel;
pub mod principal_variation;
pub mod utils;

pub const MAX_PLY: usize = 300;
pub const MATE_VALUE: i32 = 900_000;
pub const MATE_SCORE: i32 = 800_000;
pub const INF: i32 = 1_000_000;
pub const DEFAULT_ALPHA: i32 = -INF;
pub const DEFAULT_BETA: i32 = INF;
pub const ASPIRATION_WINDOW_OFFSET: i32 = 50;
pub const REPEATED_POSITION_SCORE: i32 = 0;
pub const EXTEND_CHECK: usize = 1;

use lazy_static::lazy_static;

use self::{
    heuristics::{
        futility_pruning::is_futile, late_move_pruning::is_lmp_applicable,
        late_move_reduction::is_lmr_applicable, move_order::MoveUtils,
        static_exchange_evaluation::static_exchange_evaluation, transposition_table::HashFlag,
    },
    parallel::SearchData,
};

use super::eval::{evaluate, PIECE_VALUES};

lazy_static! {
    pub static ref STOPPED: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
}

pub trait SearchEngine {
    fn search(&mut self, position: &Position, depth: usize) -> Option<BestMove>;
}

#[derive(Clone, Debug, Copy)]
pub struct BestMove {
    pub score: i32,
    pub mv: Move,
}

impl SearchData {
    fn negamax(&mut self, node: &Position, alpha: i32, beta: i32, depth: usize) -> i32 {
        self.pv.init_length(self.ply);

        if self.repetition_table.is_repeated()
            || self.repetition_table.is_draw_by_fifty_moves_rule()
        {
            return REPEATED_POSITION_SCORE;
        }

        // Mate distance pruning
        let alpha = i32::max(alpha, -MATE_VALUE + self.ply as i32 - 1);
        let beta = i32::min(beta, MATE_VALUE - self.ply as i32);
        if alpha >= beta {
            return alpha;
        }

        let pv_node = beta - alpha > 1;
        // Transposition table lookup
        let (cached_alpha, best_move) = self
            .transposition_table
            .cashed_value(node, self.ply, pv_node, depth, alpha, beta);

        if let Some(cached_alpha) = cached_alpha {
            self.nodes_pruned += 1;
            return cached_alpha;
        }

        if depth == 0 {
            return self.quiesce(node, alpha, beta);
            //return evaluate(node);
        }

        if self.ply >= MAX_PLY {
            return evaluate(node);
        }

        self.nodes_evaluated += 1;

        let mut child_nodes = self.move_gen.generate_legal_moves(node).collect_vec();
        let in_check = self.move_gen.is_check(node);

        // Null move pruning
        if self.null_move_reduction(node, beta, depth, in_check, self.ply) {
            self.nodes_pruned += 1;
            return beta;
        }

        let static_eval = evaluate(node);

        // Razoring
        if let Some((score, fails_low)) =
            self.razoring(node, static_eval, alpha, beta, depth, in_check, pv_node)
        {
            if fails_low {
                self.nodes_pruned += 1;
                return score;
            }
        }

        self.order_moves(&mut child_nodes, node, best_move);

        if child_nodes.is_empty() {
            if in_check {
                return -MATE_VALUE + self.ply as i32;
            } else {
                return 0;
            }
        }

        self.search_move_list(
            node,
            &child_nodes,
            alpha,
            beta,
            depth,
            in_check,
            pv_node,
            static_eval,
            best_move,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn search_move_list(
        &mut self,
        node: &Position,
        move_list: &[Move],
        mut alpha: i32,
        beta: i32,
        depth: usize,
        in_check: bool,
        pv_node: bool,
        static_eval: i32,
        mut best_move: Option<Move>,
    ) -> i32 {
        let mut flag = HashFlag::ALPHA;
        for (moves_tried, child) in move_list.iter().enumerate() {
            let mut extend = 0;
            let mut reduce = 0;
            let child_pos = {
                let mut child_pos = node.clone();
                let _ = child_pos.make_move(child);
                child_pos
            };

            let gives_check = self.move_gen.is_check(&child_pos);
            if gives_check {
                extend = EXTEND_CHECK;
            }

            if is_futile(
                child,
                node,
                depth,
                alpha,
                beta,
                child.is_capture(),
                in_check,
                gives_check,
                static_eval,
                moves_tried,
                extend,
            ) {
                self.nodes_pruned += 1;
                break;
            }

            // TODO: Its glitched somehow
            /*
            if is_lmp_applicable(moves_tried, depth, pv_node, gives_check, alpha, child) {
                self.nodes_pruned += 1;
                break;
            }
            */

            // Check extension
            self.ply += 1;
            self.repetition_table
                .push(&child_pos, child.is_irreversible(node));

            // Calculate score with late move reduction
            //let score = -self.negamax(&child_pos, -beta, -alpha, depth - 1);

            let score = if moves_tried == 0 || extend > 0 {
                // Full depth search
                -self.negamax(&child_pos, -beta, -alpha, depth + extend - 1)
            } else {
                let score =
                    if is_lmr_applicable(child, depth, moves_tried, in_check, gives_check, pv_node)
                    {
                        // Try reduced depth search
                        -self.negamax(&child_pos, -alpha - 1, -alpha, depth - 2)
                    } else {
                        // Force full depth search
                        alpha + 1
                    };

                // If we found potentailly better move at lower depth, search it with full depth
                if score > alpha {
                    let score = -self.negamax(&child_pos, -alpha - 1, -alpha, depth - 1);

                    if score > alpha && score < beta {
                        // LMR failed, search normally with full depth
                        -self.negamax(&child_pos, -beta, -alpha, depth - 1)
                    } else {
                        score
                    }
                } else {
                    score
                }
            };
            self.repetition_table.decrement();
            self.ply -= 1;

            if self.stopped() {
                return 0;
            }

            if score > alpha {
                if !child.is_capture() {
                    let (piece, color) = node.piece_at(&child.from()).expect("No piece found");
                    self.history_moves[color as usize][piece as usize][child.to() as usize] +=
                        depth as i32;
                }

                flag = HashFlag::EXACT;
                alpha = score;
                self.pv.push_pv_move(self.ply, *child);
                best_move = Some(*child);

                if score >= beta {
                    // Fail high
                    self.transposition_table.write(
                        node.hash,
                        beta,
                        best_move,
                        depth,
                        self.ply,
                        HashFlag::BETA,
                        self.age,
                    );

                    if !child.is_capture() {
                        self.killer_moves[1][self.ply] = self.killer_moves[0][self.ply];
                        self.killer_moves[0][self.ply] = Some(*child);
                    }

                    self.nodes_pruned += 1;

                    return beta;
                }
            }
        }

        self.transposition_table
            .write(node.hash, alpha, best_move, depth, self.ply, flag, self.age);

        alpha
    }

    fn quiesce(&mut self, node: &Position, mut alpha: i32, beta: i32) -> i32 {
        self.nodes_evaluated += 1;

        if self.repetition_table.is_repeated()
            || self.repetition_table.is_draw_by_fifty_moves_rule()
        {
            return REPEATED_POSITION_SCORE;
        }

        if self.ply >= MAX_PLY {
            return evaluate(node);
        }

        let stand_pat = evaluate(node);

        if stand_pat >= beta {
            self.nodes_pruned += 1;
            return beta;
        }

        // Delta pruning
        let queen_value = 900;
        if stand_pat < alpha - queen_value {
            self.nodes_pruned += 1;
            return alpha;
        }

        if stand_pat > alpha {
            alpha = stand_pat;
        }

        if self.stopped() {
            return 0;
        }

        let mut moves = self.move_gen.generate_legal_moves(node).collect_vec();

        self.order_moves(&mut moves, node, None);

        for mv in moves {
            if !mv.is_capture() {
                continue;
            }

            if !mv.is_enpass_capture() {
                let attacking_piece = node.piece_at(&mv.from()).unwrap().0;
                let captured_piece = node.piece_at(&mv.to()).unwrap().0;

                if PIECE_VALUES[attacking_piece as usize] > PIECE_VALUES[captured_piece as usize]
                    && static_exchange_evaluation(&self.move_gen, node, &mv) < 0
                {
                    self.nodes_pruned += 1;
                    continue;
                }
            }

            let child = {
                let mut child = node.clone();
                let _ = child.make_move(&mv);
                child
            };

            self.ply += 1;
            self.repetition_table.push(&child, mv.is_irreversible(node));
            let score = -self.quiesce(&child, -beta, -alpha);
            self.repetition_table.decrement();
            self.ply -= 1;

            if self.stopped() {
                return 0;
            }

            if score > alpha {
                alpha = score;

                if score >= beta {
                    self.nodes_pruned += 1;
                    return beta;
                }
            }
        }

        alpha
    }
}
