use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

use itertools::Itertools;
use move_gen::{
    generators::movegen::MoveGen,
    r#move::{MakeMove, Move},
};
use sdk::position::Position;

pub mod heuristics;
pub mod principal_variation;
pub mod utils;

pub const MAX_PLY: usize = 300;
pub const MATE_SCORE: isize = 10000;

use lazy_static::lazy_static;

use self::{
    heuristics::{
        late_move_reduction::is_lmr_applicable,
        move_order::MoveUtils,
        transposition_table::{HashFlag, TranspositionTable},
    },
    principal_variation::PrincipalVariation,
    utils::{
        repetition::RepetitionTable,
        time_control::{SearchOptions, TimeControl},
    },
};

use super::eval::evaluate;

lazy_static! {
    pub static ref STOPPED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

pub trait SearchEngine {
    fn search(&mut self, position: &Position, depth: usize) -> Option<BestMove>;
}

#[derive(Clone, Debug, Copy)]
pub struct BestMove {
    pub score: isize,
    pub mv: Move,
}

pub struct Search {
    pub nodes_evaluated: usize,
    pub quiesce_nodes_evaluated: usize,
    pub ply: usize,
    pub move_gen: Arc<MoveGen>,
    pub best: Option<BestMove>,
    pub killer_moves: [[Option<Move>; MAX_PLY]; 2],
    pub history_moves: [[[isize; 64]; 6]; 2],
    pub pv: PrincipalVariation,
    pub stopped: Arc<Mutex<bool>>,
    pub options: SearchOptions,
    pub time_control: TimeControl,
    pub repetition_table: RepetitionTable,
    pub transposition_table: TranspositionTable,
}

impl Search {
    pub fn new(
        options: SearchOptions,
        move_gen: Arc<MoveGen>,
        is_white: bool,
        repetition_table: RepetitionTable,
    ) -> Self {
        let time_control = options.time_control(is_white);

        Self {
            nodes_evaluated: 0,
            quiesce_nodes_evaluated: 0,
            ply: 0,
            move_gen,
            best: None,
            killer_moves: [[None; MAX_PLY]; 2],
            history_moves: [[[0; 64]; 6]; 2],
            pv: PrincipalVariation::default(),
            stopped: Arc::clone(&STOPPED),
            options,
            time_control,
            repetition_table,
            transposition_table: TranspositionTable::default(),
        }
    }

    pub fn total_nodes_evaluated(&self) -> usize {
        self.nodes_evaluated + self.quiesce_nodes_evaluated
    }

    pub fn reset(&mut self) {
        self.nodes_evaluated = 0;
        self.quiesce_nodes_evaluated = 0;
        self.ply = 0;
        self.best = None;
        self.killer_moves = [[None; MAX_PLY]; 2];
        self.history_moves = [[[0; 64]; 6]; 2];
        self.pv = PrincipalVariation::default();
    }

    pub fn search(&mut self, position: &Position) -> Option<BestMove> {
        let (alpha, beta) = (-1_000_000, 1_000_000);

        let depth = self.options.depth.unwrap_or(100);

        let mut best_move = None;
        for i in 1..=depth {
            self.reset();
            let best_score = self.negamax(position, alpha, beta, i);
            if self.best.is_some() {
                best_move = self.best;
            }

            if self.stopped() {
                break;
            }

            let current_nodes_count = self.nodes_evaluated + self.quiesce_nodes_evaluated;

            let score_str = mate_score(best_score)
                .map(|score| format!("mate {}", score))
                .unwrap_or_else(|| format!("cp {}", best_score));

            let time = self.time_control.search_time(Instant::now());
            let nps = if time == 0 {
                20000
            } else {
                (current_nodes_count as f64 / (time as f64 / 1000.0)) as usize
            };

            if self.best.is_none() {
                break;
            }

            println!(
                "info score {} depth {} nodes {} nps {} time {} pv {}",
                score_str,
                i,
                current_nodes_count,
                nps,
                time,
                self.pv.to_string()
            );
        }

        if let Some(best) = best_move {
            println!("bestmove {}", best.mv);
        }

        self.best
    }

    fn stopped(&self) -> bool {
        *self.stopped.lock().unwrap() || self.time_control.is_over()
    }

    fn negamax(
        &mut self,
        node: &Position,
        mut alpha: isize,
        beta: isize,
        mut depth: usize,
    ) -> isize {
        self.pv.init_length(self.ply);

        if self.repetition_table.is_repeated() {
            return 0;
        }

        let mut hash_flag = HashFlag::UPPERBOUND;

        if let Some(cached_alpha) = self.transposition_table.read(node.hash, alpha, beta, depth) {
            //println!("Found cached alpha: {cached_alpha} on depth {depth}");
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

        let mut best_so_far = None;
        let old_alpha = alpha;

        let in_check = self.move_gen.is_check(node);

        let mut child_nodes = self.move_gen.generate_legal_moves(node).collect_vec();
        if child_nodes.is_empty() {
            if in_check {
                return -MATE_SCORE + self.ply as isize;
            } else {
                return 0;
            }
        }

        if in_check {
            depth += 1;
        }

        // Null move pruning
        if depth >= 3 && !in_check {
            let child = {
                let mut child = node.clone();
                child.make_null_move();
                child
            };

            self.repetition_table.push(&child);
            self.ply += 1;
            let score = -self.negamax(&child, -beta, -beta + 1, depth - 3);
            self.ply -=1;
            self.repetition_table.decrement();

            if self.stopped() {
                return 0;
            }

            if score >= beta {
                return beta;
            }
        }

        self.order_moves(&mut child_nodes, node);

        for (moves_tried, child) in child_nodes.into_iter().enumerate() {
            let child_pos = {
                let mut child_pos = node.clone();
                let _ = child_pos.make_move(&child);
                child_pos
            };

            self.ply += 1;
            self.repetition_table.push(&child_pos);

            // Calculate score with late move reduction
            //let score = -self.negamax(&child_pos, -beta, -alpha, depth - 1);
            let score = if moves_tried == 0 {
                // Full depth search
                -self.negamax(&child_pos, -beta, -alpha, depth - 1)
            } else {
                let score = if is_lmr_applicable(&child, depth, moves_tried, in_check) {
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

            if score >= beta {
                self.transposition_table
                    .write(node.hash, score, depth, HashFlag::LOWERBOUND);

                if !child.is_capture() {
                    self.killer_moves[1][self.ply] = self.killer_moves[0][self.ply];
                    self.killer_moves[0][self.ply] = Some(child);
                }

                return beta;
            }

            if self.stopped() {
                return 0;
            }

            if score > alpha {
                hash_flag = HashFlag::EXACT;

                if !child.is_capture() {
                    let (piece, color) = node.piece_at(&child.from()).unwrap();
                    self.history_moves[color as usize][piece as usize][child.to() as usize] +=
                        depth as isize;
                }

                alpha = score;

                self.pv.push_pv_move(self.ply, child);

                if self.ply == 0 {
                    best_so_far = Some(child);
                }
            }
        }

        if old_alpha != alpha {
            self.best = best_so_far.map(|mv| BestMove { score: alpha, mv });
        }

        self.transposition_table
            .write(node.hash, alpha, depth, hash_flag);

        alpha
    }

    fn quiesce(&mut self, node: &Position, mut alpha: isize, beta: isize) -> isize {
        self.quiesce_nodes_evaluated += 1;

        if self.repetition_table.is_repeated() {
            return 0;
        }

        let stand_pat = evaluate(node);

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

        if self.stopped() {
            return 0;
        }

        let mut moves = self.move_gen.generate_legal_moves(node).collect_vec();

        self.order_moves(&mut moves, node);

        for mv in moves {
            if !mv.is_capture() {
                continue;
            }

            let child = {
                let mut child = node.clone();
                let _ = child.make_move(&mv);
                child
            };

            self.ply += 1;
            self.repetition_table.push(&child);
            let score = -self.quiesce(&child, -beta, -alpha);
            self.repetition_table.decrement();
            self.ply -= 1;

            if score >= beta {
                return beta;
            }

            if self.stopped() {
                return 0;
            }

            if score > alpha {
                alpha = score;
            }
        }

        alpha
    }
}

fn mate_score(score: isize) -> Option<isize> {
    if score > MATE_SCORE - MAX_PLY as isize {
        Some((MATE_SCORE - score + 1) / 2)
    } else if score < -MATE_SCORE + MAX_PLY as isize {
        Some((-MATE_SCORE - score + 1) / 2)
    } else {
        None
    }
}
