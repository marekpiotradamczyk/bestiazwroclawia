use itertools::Itertools;
use move_gen::{
    generators::movegen::MoveGen,
    r#move::{MakeMove, Move},
};
use sdk::position::Position;

use crate::core::evaluate::evaluate;

use super::{move_order::MoveUtils, Engine};

pub const MAX_PLY: usize = 128;

pub trait SearchEngine {
    fn search(&mut self, position: &Position, depth: usize) -> Option<BestMove>;
}

#[derive(Clone, Debug)]
pub struct BestMove {
    pub score: isize,
    pub mv: Move,
}

pub struct Search<'a> {
    pub nodes_evaluated: usize,
    pub quiesce_nodes_evaluated: usize,
    pub ply: usize,
    pub move_gen: &'a MoveGen,
    pub best: Option<BestMove>,
    pub killer_moves: [[Option<Move>; MAX_PLY]; 2],
    pub history_moves: [[[isize; 64]; 6]; 2],
}

impl<'a> Search<'a> {
    pub fn new(move_gen: &'a MoveGen) -> Self {
        Self {
            nodes_evaluated: 0,
            quiesce_nodes_evaluated: 0,
            ply: 0,
            move_gen,
            best: None,
            killer_moves: [[None; MAX_PLY]; 2],
            history_moves: [[[0; 64]; 6]; 2],
        }
    }
}

impl SearchEngine for Engine {
    fn search(&mut self, position: &Position, depth: usize) -> Option<BestMove> {
        let mut search = Search::new(&self.move_gen);

        let (alpha, beta) = (-1_000_000, 1_000_000);

        search.negamax(position, alpha, beta, depth);

        let current_total = search.nodes_evaluated + search.quiesce_nodes_evaluated;
        self.total_nodes_evaluated += current_total;
        self.nodes_evaluated = current_total;

        search.best
    }
}

impl<'a> Search<'a> {
    fn negamax(
        &mut self,
        node: &Position,
        mut alpha: isize,
        beta: isize,
        mut depth: usize,
    ) -> isize {
        if depth == 0 {
            return self.quiesce(node, alpha, beta);
            //return evaluate(node);
        }

        self.nodes_evaluated += 1;

        let mut best_so_far = None;
        let old_alpha = alpha;
        let mut child_nodes = self.move_gen.generate_legal_moves(node).collect_vec();

        let in_check = self.move_gen.is_check(node);

        if child_nodes.is_empty() {
            if in_check {
                return -10000 + self.ply as isize;
            } else {
                return 0;
            }
        }

        if in_check {
            depth += 1;
        }

        self.order_moves(&mut child_nodes, node);

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
                self.killer_moves[1][self.ply] = self.killer_moves[0][self.ply];
                self.killer_moves[0][self.ply] = Some(child);

                return beta;
            }

            if score > alpha {
                let (piece, color) = node.piece_at(&child.from()).unwrap();
                self.history_moves[color as usize][piece as usize][child.to() as usize] +=
                    depth as isize;

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

    fn quiesce(&mut self, node: &Position, mut alpha: isize, beta: isize) -> isize {
        self.quiesce_nodes_evaluated += 1;
        let stand_pat = evaluate(node);

        if stand_pat >= beta {
            return beta;
        }

        if stand_pat > alpha {
            alpha = stand_pat;
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
            let score = -self.quiesce(&child, -beta, -alpha);
            self.ply -= 1;

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
