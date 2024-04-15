use std::{
    mem::MaybeUninit,
    sync::{atomic::Ordering, Arc},
    time::Instant,
};

use sdk::position::Position;

use super::{
    heuristics::transposition_table::TranspositionTable,
    principal_variation::PrincipalVariation,
    utils::{
        repetition::Table,
        time_control::{SearchOptions, TimeControl},
    },
    MATE_VALUE,
};

use crate::engine::{eval::evaluation_table::EvaluationTable, search::STOPPED};
use crate::engine::{options::Options, search::MAX_PLY};
use move_gen::r#move::Move;
pub const INF: i32 = 1_000_000;
pub const DEFAULT_ALPHA: i32 = -INF;
pub const DEFAULT_BETA: i32 = INF;
pub const ASPIRATION_WINDOW_OFFSET: i32 = 50;

pub struct Search {
    pub time_control: Arc<TimeControl>,
    pub options: SearchOptions,
    pub engine_options: Options,
    pub repetion_table: Table,
    pub transposition_table: Arc<TranspositionTable>,
    pub eval_table: Arc<EvaluationTable>,
    pub age: usize,
}

pub struct SearchThread {
    pub data: SearchData,
    pub transposition_table: Arc<TranspositionTable>,
    pub eval_table: Arc<EvaluationTable>,
    pub depth: usize,
    pub id: usize,
}

#[derive(Clone)]
pub struct SearchData {
    pub nodes_evaluated: usize,
    pub ply: usize,
    pub killer_moves: [[Option<Move>; MAX_PLY]; 2],
    pub history_moves: [[[i32; 64]; 6]; 2],
    pub counter_moves: [[Option<Move>; MAX_PLY]; 2],
    pub pair_moves: [[Option<Move>; MAX_PLY]; 2],
    pub pv: PrincipalVariation,
    pub repetition_table: Table,
    pub transposition_table: Arc<TranspositionTable>,
    pub eval_table: Arc<EvaluationTable>,
    pub time_control: Arc<TimeControl>,
    pub age: usize,
    pub current_move: MaybeUninit<Move>,
}

#[allow(clippy::too_many_arguments)]
impl Search {
    #[must_use]
    pub fn new(
        options: SearchOptions,
        engine_options: Options,
        is_white: bool,
        rep_table: Table,
        transposition_table: Arc<TranspositionTable>,
        eval_table: Arc<EvaluationTable>,
        age: usize,
    ) -> Self {
        Self {
            time_control: Arc::new(options.time_control(is_white)),
            options,
            repetion_table: rep_table,
            transposition_table,
            eval_table,
            engine_options,
            age,
        }
    }

    pub fn search(&mut self, position: &Position) {
        let mut threads = vec![];
        let threads_cnt = self.engine_options.threads;
        for id in 0..threads_cnt {
            let data = SearchData {
                nodes_evaluated: 0,
                ply: 0,
                killer_moves: [[None; MAX_PLY]; 2],
                history_moves: [[[0; 64]; 6]; 2],
                pv: PrincipalVariation::default(),
                repetition_table: self.repetion_table.clone(),
                transposition_table: self.transposition_table.clone(),
                time_control: self.time_control.clone(),
                age: self.age,
                eval_table: self.eval_table.clone(),
                counter_moves: [[None; MAX_PLY]; 2],
                pair_moves: [[None; MAX_PLY]; 2],
                current_move: MaybeUninit::uninit(),
            };

            let mut thread = SearchThread {
                data: data.clone(),
                transposition_table: data.transposition_table.clone(),
                depth: self.options.depth.unwrap_or(150),
                id,
                eval_table: self.eval_table.clone(),
            };

            let pos = position.clone();

            threads.push(std::thread::spawn(move || {
                thread.go(&pos);
            }));
        }

        for thread in threads {
            thread.join().unwrap();
        }
    }
}

impl SearchThread {
    pub fn go(&mut self, position: &Position) {
        let is_prime_thread = self.id == 0;
        let (mut alpha, mut beta) = (DEFAULT_ALPHA, DEFAULT_BETA);

        let mut best_move = None;
        for depth in 1..=self.depth {
            if self.data.stopped() {
                break;
            }
            self.data.reset();
            let mut best_score = self.data.negamax(position, alpha, beta, depth);

            // Try full search if aspiration window failed
            if best_score <= alpha || best_score >= beta {
                alpha = DEFAULT_ALPHA;
                beta = DEFAULT_BETA;
                best_score = self.data.negamax(position, alpha, beta, depth);
            }

            // Adjust aspiration window
            alpha = best_score - ASPIRATION_WINDOW_OFFSET;
            beta = best_score + ASPIRATION_WINDOW_OFFSET;

            if is_prime_thread {
                let current_nodes_count = self.data.nodes_evaluated;

                let time = self.data.time_control.search_time(Instant::now());

                let nps = if time == 0 {
                    20000
                } else {
                    (current_nodes_count as f64 / (time as f64 / 1000.0)) as usize
                };

                let score_str = mate_score(best_score).map_or_else(
                    || format!("cp {best_score}"),
                    |score| format!("mate {score}"),
                );

                if self.data.stopped() {
                    break;
                }

                println!(
                    "info score {} depth {} nodes {} nps {} time {} pv {}",
                    score_str,
                    depth,
                    current_nodes_count,
                    nps,
                    time,
                    self.data.pv.to_string()
                );
            }

            if self.data.pv.best().is_some() && !self.data.stopped() {
                best_move = self.data.pv.best();
            }
        }
        if is_prime_thread {
            if let Some(best_move) = best_move {
                println!("bestmove {best_move}");
            } else {
                // Log null move, just to satisfy the protocol
                if let Some(best) = self.data.pv.best() {
                    println!("bestmove {best}");
                }
            }
        }
    }
}

impl SearchData {
    pub fn reset(&mut self) {
        //*self.nodes_evaluated.lock().unwrap() = 0;
        self.ply = 0;
        self.killer_moves = [[None; MAX_PLY]; 2];
        self.history_moves = [[[0; 64]; 6]; 2];
        self.pv = PrincipalVariation::default();
        //self.age += 1;
    }

    #[must_use]
    pub fn stopped(&self) -> bool {
        self.time_control.is_over() || STOPPED.load(Ordering::Relaxed)
    }
}

fn mate_score(score: i32) -> Option<i32> {
    if score > MATE_VALUE - MAX_PLY as i32 {
        Some((MATE_VALUE - score + 1) / 2)
    } else if score < -MATE_VALUE + MAX_PLY as i32 {
        Some((-MATE_VALUE - score + 1) / 2)
    } else {
        None
    }
}
