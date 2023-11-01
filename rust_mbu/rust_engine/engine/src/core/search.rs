use itertools::Itertools;
use move_gen::{
    generators::movegen::MoveGen,
    r#move::{MakeMove, Move, MoveKind},
};
use sdk::position::{self, Color, Position};

use crate::core::evaluate::evaluate;

use super::Engine;

pub trait SearchEngine {
    fn search(&mut self, position: &Position, depth: usize) -> Option<BestMove>;
}

#[derive(Clone, Debug)]
pub struct BestMove {
    pub score: f64,
    pub mv: Move,
}

pub struct Search<'a> {
    pub nodes_evaluated: usize,
    pub quiesce_nodes_evaluated: usize,
    pub ply: usize,
    pub move_gen: &'a MoveGen,
    pub best: Option<BestMove>,
}

impl<'a> Search<'a> {
    pub fn new(move_gen: &'a MoveGen) -> Self {
        Self {
            nodes_evaluated: 0,
            quiesce_nodes_evaluated: 0,
            ply: 0,
            move_gen,
            best: None,
        }
    }
}

impl SearchEngine for Engine {
    fn search(&mut self, position: &Position, depth: usize) -> Option<BestMove> {
        let mut search = Search::new(&self.move_gen);

        let (alpha, beta) = (f64::MIN, f64::MAX);

        search.negamax(position, alpha, beta, depth);

        let current_total = search.nodes_evaluated + search.quiesce_nodes_evaluated;
        self.total_nodes_evaluated += current_total;
        self.nodes_evaluated = current_total;

        search.best
    }
}

impl<'a> Search<'a> {
    fn negamax(&mut self, node: &Position, mut alpha: f64, beta: f64, depth: usize) -> f64 {
        self.nodes_evaluated += 1;

        if depth == 0 {
            return self.quiesce(node, alpha, beta);
        }

        let mut best_so_far = None;
        let old_alpha = alpha;
        let child_nodes = self.move_gen.generate_legal_moves(node).collect_vec();

        if child_nodes.is_empty() {
            if self.move_gen.is_check(node) {
                return -10000.0 + self.ply as f64;
            } else {
                return 0.0;
            }
        }

        for child in child_nodes {
            let child_pos = {
                let mut child_pos = node.clone();
                let _ = child_pos.make_move(&child);
                child_pos
            };

            self.ply += 1;
            let score = -self.negamax(&child_pos, -beta, -alpha, depth - 1);
            self.ply -= 1;

            if score >= beta {
                return beta;
            }

            if score > alpha {
                alpha = score;

                if self.ply == 0 {
                    best_so_far = Some(child);
                }
            }
        }

        if old_alpha != alpha {
            self.best = best_so_far.map(|mv| BestMove { score: alpha, mv });
        }

        alpha
    }

    fn quiesce(&mut self, node: &Position, mut alpha: f64, beta: f64) -> f64 {
        self.quiesce_nodes_evaluated += 1;
        let stand_pat = evaluate(node);

        if stand_pat >= beta {
            return beta;
        }

        if stand_pat > alpha {
            alpha = stand_pat;
        }

        let moves = self
            .move_gen
            .generate_legal_moves(node);
            //.filter(Move::is_capture);

        for mv in moves {
            if mv.is_capture() {
                continue;
            }
            let child = {
                let mut child = node.clone();
                let _ = child.make_move(&mv);
                child
            };

            let score = -self.quiesce(&child, -beta, -alpha);

            if score >= beta {
                return beta;
            }

            if score > alpha {
                alpha = score;
            }
        }

        alpha
    }
}
