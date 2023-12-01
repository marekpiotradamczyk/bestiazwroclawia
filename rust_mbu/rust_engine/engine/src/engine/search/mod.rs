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
        futility_pruning::is_futile,
        late_move_pruning::is_lmp_applicable,
        late_move_reduction::is_lmr_applicable,
        move_order::MoveUtils,
        static_exchange_evaluation::{
            static_exchange_evaluation, static_exchange_evaluation_move_done,
        },
        transposition_table::HashFlag,
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
        if self.stopped() {
            return 0;
        }
        // Initialize PV table
        self.pv.init_length(self.ply);

        self.nodes_evaluated += 1;

        // Check for draw by repetition
        if self.repetition_table.is_repeated()
            || self.repetition_table.is_draw_by_fifty_moves_rule()
        {
            return REPEATED_POSITION_SCORE;
        }

        // Prune mate distance
        // If we can see mate, we don't need to search further
        let alpha = i32::max(alpha, -MATE_VALUE + self.ply as i32 - 1);
        let beta = i32::min(beta, MATE_VALUE - self.ply as i32);
        if alpha >= beta {
            return alpha;
        }

        // If aspiration window is null, we are in PV node
        let pv_node = beta - alpha > 1;

        // Transposition table lookup
        let (cached_alpha, best_move) = self
            .transposition_table
            .cashed_value(node, self.ply, pv_node, depth, alpha, beta);

        if let Some(cached_alpha) = cached_alpha {
            return cached_alpha;
        }

        // Run quiescence search on horizon
        if depth == 0 {
            return self.quiesce(node, alpha, beta);
            //return evaluate(node);
        }

        // Stop search if we are too deep
        if self.ply >= MAX_PLY {
            return evaluate(node, self.eval_table.clone(), self.move_gen.clone());
        }

        // Generate legal moves for current position
        let mut child_nodes = self.move_gen.generate_legal_moves(node).collect_vec();
        let in_check = self.move_gen.is_check(node);

        // Null move pruning
        if self.null_move_reduction(node, beta, depth, in_check, self.ply) {
            return beta;
        }

        // Statically evaluate current position. This is needed for pruning.
        let static_eval = evaluate(node, self.eval_table.clone(), self.move_gen.clone());

        // Razoring
        if let Some(score) = self.razoring(node, static_eval, alpha, beta, depth, in_check, pv_node)
        {
            return score;
        }

        // Order moves by probability of being good in order to improve alpha-beta pruning.
        self.order_moves(&mut child_nodes, node, best_move);

        // If there are no legal moves, we are in checkmate or stalemate
        if child_nodes.is_empty() {
            if in_check {
                return -MATE_VALUE + self.ply as i32;
            } else {
                return 0;
            }
        }

        // At this point we couldn't prune anything, so we need to start searching through legal
        // moves.
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
        if self.stopped() {
            return 0;
        }
        // Flag for transposition table indicating if we found exact score or not.
        let mut flag = HashFlag::ALPHA;

        for (moves_tried, child) in move_list.iter().enumerate() {
            let mut extend = 0;
            let mut reduce = 0;

            // Make a move
            let child_pos = {
                let mut child_pos = node.clone();
                let _ = child_pos.make_move(child);
                child_pos
            };

            let gives_check = self.move_gen.is_check(&child_pos);

            // Check extension. We don't extend if check is unsafe, that is oponnent can gain
            // material by series of captures. We check that using `static_exchange_evaluation`.
            if gives_check {
                let value_of_moved_piece =
                    PIECE_VALUES[node.piece_at(&child.from()).unwrap().0 as usize];

                let opponent_recapture_gain =
                    static_exchange_evaluation_move_done(&self.move_gen, node, child);

                let is_safe_check = opponent_recapture_gain <= value_of_moved_piece;

                if is_safe_check {
                    extend = EXTEND_CHECK;
                }
            }

            // Futulity pruning
            // We assume we can't improve in certain situations, so we prune the node.
            if is_futile(
                child,
                node,
                depth,
                alpha,
                beta,
                pv_node,
                child.is_capture(),
                in_check,
                gives_check,
                static_eval,
                moves_tried,
                extend,
            ) {
                break;
            }

            // Late move pruning
            // We assume that moves that are far in the move list, are less likely to be good, so we prune them.
            // Not applicable in PV nodes, in check, in captures and in positions with mate score.
            if is_lmp_applicable(moves_tried, depth, pv_node, in_check, alpha, child) {
                break;
            }

            // Check extension
            self.ply += 1;
            self.repetition_table
                .push(&child_pos, child.is_irreversible(node));

            // Calculate score with late move reduction
            //let score = -self.negamax(&child_pos, -beta, -alpha, depth - 1);
            if is_lmr_applicable(
                child,
                depth,
                moves_tried,
                in_check,
                gives_check,
                pv_node,
                extend,
            ) {
                reduce = 1;
            }

            // Search move
            let score = self.search_move(&child_pos, alpha, beta, depth, reduce, extend, pv_node);

            self.repetition_table.decrement();
            self.ply -= 1;

            // Do not update anything if we are stopped
            if self.stopped() {
                return 0;
            }
            // If we found better move, update alpha and best move
            if score > alpha {
                if !child.is_capture() {
                    let (piece, color) = node.piece_at(&child.from()).expect("No piece found");
                    // Update history moves, so we can order moves better next time
                    self.history_moves[color as usize][piece as usize][child.to() as usize] +=
                        (depth * depth) as i32;
                }

                flag = HashFlag::EXACT;
                alpha = score;
                self.pv.push_pv_move(self.ply, *child);
                best_move = Some(*child);

                // Fail high
                if score >= beta {
                    // Store beta cutoff in transposition table
                    self.transposition_table.write(
                        node.hash,
                        beta,
                        best_move,
                        depth,
                        self.ply,
                        HashFlag::BETA,
                        self.age,
                    );

                    // Update move order
                    if !child.is_capture() {
                        self.killer_moves[1][self.ply] = self.killer_moves[0][self.ply];
                        self.killer_moves[0][self.ply] = Some(*child);

                        // counter moves
                        if self.ply > 0 {
                            let previous_move_ply = self.ply - 1;

                            self.counter_moves[1][previous_move_ply] =
                                self.counter_moves[0][previous_move_ply];
                            self.counter_moves[0][previous_move_ply] = Some(*child);
                        }

                        // Pair moves
                        if self.ply > 1 {
                            self.pair_moves[1][self.ply - 2] = self.pair_moves[0][self.ply - 2];
                            self.pair_moves[0][self.ply - 2] = Some(*child);
                        }
                    }

                    return beta;
                }
            }
        }

        // Store alpha cutoff in transposition table
        self.transposition_table
            .write(node.hash, alpha, best_move, depth, self.ply, flag, self.age);

        alpha
    }

    #[allow(clippy::too_many_arguments)]
    fn search_move(
        &mut self,
        child_pos: &Position,
        alpha: i32,
        beta: i32,
        depth: usize,
        reduce: usize,
        extend: usize,
        pv_node: bool,
    ) -> i32 {
        if self.stopped() {
            return 0;
        }

        let final_depth = depth - reduce + extend - 1;
        // Do the PV search to check whether move is good or not
        let mut score = -self.negamax(child_pos, -alpha - 1, -alpha, final_depth);

        // If we found potentailly better move at lower depth, search it with full depth
        if score > alpha && reduce > 0 {
            score = -self.negamax(child_pos, -alpha - 1, -alpha, depth - 1);
        }

        if score > alpha && score < beta && pv_node {
            // LMR failed, search normally with full depth
            score = -self.negamax(child_pos, -beta, -alpha, depth - 1);
        }

        score
    }

    fn quiesce(&mut self, node: &Position, mut alpha: i32, beta: i32) -> i32 {
        if self.stopped() {
            return 0;
        }

        self.nodes_evaluated += 1;

        if self.repetition_table.is_repeated()
            || self.repetition_table.is_draw_by_fifty_moves_rule()
        {
            return REPEATED_POSITION_SCORE;
        }

        if self.ply >= MAX_PLY {
            return evaluate(node, self.eval_table.clone(), self.move_gen.clone());
        }

        let stand_pat = evaluate(node, self.eval_table.clone(), self.move_gen.clone());

        if stand_pat >= beta {
            return beta;
        }

        // Delta pruning
        let queen_value = 900;
        if stand_pat < alpha - queen_value {
            return alpha;
        }

        if stand_pat > alpha {
            alpha = stand_pat;
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

            if score > alpha {
                alpha = score;

                if score >= beta {
                    return beta;
                }
            }
        }

        alpha
    }
}
