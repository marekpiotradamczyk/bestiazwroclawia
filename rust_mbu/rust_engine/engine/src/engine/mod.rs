use std::{
    sync::{
        atomic::Ordering,
        mpsc::{channel, Sender},
        Arc,
    },
    thread,
};

use crate::{
    engine::eval::{
        activity::bonus_for_mobility,
        evaluate,
        king_safety::{bonus_for_pieces_close_to_king, calc_king_safety},
        material,
        pawns::{
            isolated::isolated_pawns,
            protected_passed_pawnes::passed_pawns,
            stacked::stacked_pawns,
            strong_squares::{bonus, bonus_for_piece},
        },
        pin_bonus::bonus_for_absolute_pins,
        positional_tables::{game_phase, tapered_eval},
        rooks::{
            battery::bonus_for_rook_batteries,
            rook_on_open_files::{bonus_rook_for_open_files, bonus_rook_for_semi_open_files},
        },
    },
    uci::{commands::Command, Result},
};
use move_gen::{generators::movegen::MoveGen, r#move::MakeMove};
use nn::DenseNetwork;
use nn::readcsv::load_array_from_csv;

use sdk::{
    fen::Fen,
    position::{Color, Position},
};

use crate::engine::options::Options;

use anyhow::anyhow;

use self::{
    eval::evaluation_table::EvaluationTable,
    search::{
        heuristics::transposition_table::TranspositionTable,
        parallel::Search,
        utils::{repetition::Table, time_control::SearchOptions},
        STOPPED,
    },
};

lazy_static::lazy_static! {
    pub static ref MOVE_GEN: MoveGen = MoveGen::default();
}

use rand::prelude::*;
use rand::distributions::WeightedIndex;
use std::time::Instant;
use std::env;

pub mod eval;
pub mod options;
pub mod search;
pub mod nn;


pub struct Engine {
    pub root_pos: Position,
    pub repetition_table: Table,
    pub transposition_table: Arc<TranspositionTable>,
    pub evaluation_table: Arc<EvaluationTable>,
    pub options: Options,
    pub age: usize,
    pub ready: bool,
    pub dense: DenseNetwork,
    pub rng: ThreadRng,
    pub dist: WeightedIndex<i32>,
}

const CHOICES: &'static [f32] = &[0.0, 0.01, 0.02, 0.03, 0.04, 0.05, 0.06, 0.07, 0.08, 0.09, 0.1, 0.11, 0.12, 0.13, 0.14, 0.15, 0.16, 0.17, 0.18, 0.19, 0.2, 0.21, 0.22, 0.23, 0.24, 0.25, 0.26, 0.27, 0.28, 0.29, 0.3, 0.31, 0.32, 0.33, 0.34, 0.35, 0.36, 0.37, 0.38, 0.39, 0.4, 0.41, 0.42, 0.43, 0.44, 0.45, 0.46, 0.47, 0.48, 0.49, 0.5, 0.51, 0.52, 0.53, 0.54, 0.55, 0.56, 0.57, 0.58, 0.59, 0.6, 0.61, 0.62, 0.63, 0.64, 0.65, 0.66, 0.67, 0.68, 0.69, 0.7, 0.71, 0.72, 0.73, 0.74, 0.75, 0.76, 0.77, 0.78, 0.79, 0.8, 0.81, 0.82, 0.83, 0.84, 0.85, 0.86, 0.87, 0.88, 0.89, 0.9, 0.91, 0.92, 0.93, 0.94, 0.95, 0.96, 0.97, 0.98, 0.99, 1.0];

impl Default for Engine {
    fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut dist_path = args[1].clone();
        dist_path.push_str("dist.csv");
        let weights = load_array_from_csv(&dist_path);

        let rng = rand::thread_rng();
        let dist: WeightedIndex<i32> = WeightedIndex::new(weights.unwrap()).unwrap();
        let dense = DenseNetwork::new(args[1].as_str());

        // let dense = DenseNetwork::default();
        Self {
            root_pos: Default::default(),
            repetition_table: Default::default(),
            transposition_table: Arc::new(Default::default()),
            evaluation_table: Arc::new(Default::default()),
            options: Default::default(),
            age: Default::default(),
            ready: true,
            dense: dense,
            rng,
            dist,
        }
    }
}

impl Engine {
    pub fn handle_command(&mut self, command: Command) {
        match command {
            Command::Uci => uci_info(),
            Command::Go(options) => self.go(options),
            Command::Stop => self.stop(),
            Command::Position(pos, moves) => self.position(pos, moves),
            Command::SetOption(name, value) => self.set_option(&name, value),
            Command::IsReady => println!("readyok"),
            Command::Debug => self.debug(),
            Command::UciNewGame => self.uci_new_game(),
            Command::Test => self.test(),
            Command::Simulate(moves) => self.simulate(&moves),
            Command::Quit => {}
        };
    }

    #[must_use]
    pub fn start_loop_thread() -> Sender<Command> {
        let (tx, rx) = channel();

        let run = move || {
            let mut engine = Engine::default();

            loop {
                let command = rx.recv().expect("Failed to receive command");
                
                if matches!(command, Command::Quit) {
                    break;
                }

                engine.handle_command(command);
            }
        };

        thread::Builder::new().spawn(run).unwrap();

        tx
    }

    pub fn go(&mut self, mut options: SearchOptions) {
        STOPPED.store(false, Ordering::Relaxed);
        let pos = self.root_pos.clone();
        let is_white = pos.turn == Color::White;
        let rep_table = self.repetition_table.clone();
        let transposition_table = self.transposition_table.clone();
        let eval_table = self.evaluation_table.clone();
        let engine_options = self.options;
        let age = self.age;

        // let now = Instant::now();

        let result = self.dense.forward(&pos).get(0).unwrap().clone();

        // let result = CHOICES[self.dist.sample(&mut self.rng)];
        // println!("Elapsed: {:.2?}", now.elapsed());

        // let time_left = if is_white {
        //     options.wtime.unwrap_or(0)
        // } else {
        //     options.btime.unwrap_or(0)
        // };

        // if result < 0.2 {
        //     options.depth = Some(12);
        // } else {
        //     let limit = (time_left as f32 / (200.0 - (self.age as f32))).max(1.0) * (2.0 - result);
        //     options.movetime = Some(limit as isize);
        // }

        println!("depth_debug: {}", result);

        let run = move || {
            let mut search = Search::new(
                options,
                engine_options,
                is_white,
                rep_table,
                transposition_table,
                eval_table,
                age
            );
            search.search(&pos);
        };

        thread::Builder::new()
            .name("GoThread".to_string())
            .spawn(run)
            .unwrap();
    }

    pub fn stop(&mut self) {
        STOPPED.store(true, Ordering::Relaxed);
    }

    fn position(&mut self, mut pos: Position, moves: Vec<String>) {
        self.repetition_table.clear();
        // TODO: Temp fix
        self.transposition_table = Arc::new(TranspositionTable::new(self.options.hash));
        self.age += 1;
        match parse_uci_moves(moves, &mut pos) {
            Ok(repetition_table) => {
                self.root_pos = pos;
                self.repetition_table = repetition_table;
            }
            Err(e) => println!("{e}"),
        }
    }

    fn debug(&self) {
        println!("{}", self.root_pos);
        let moves = MOVE_GEN.generate_legal_moves(&self.root_pos);
        println!("Legal moves: ");
        for mv in moves {
            print!("{mv} ");
        }
        println!();
        println!();

        println!("RAW DIFF: {}", material(&self.root_pos));
        let phase = game_phase(&self.root_pos);
        println!("Game phase: {phase}");
        println!();
        println!("Tapered eval: {}", tapered_eval(&self.root_pos, phase));
        println!("Safety bonus: {}", calc_king_safety(&self.root_pos));
        println!(
            "Safety pieces bonus: {}",
            bonus_for_pieces_close_to_king(&self.root_pos)
        );
        println!("Isolated pawns penalty: {}", isolated_pawns(&self.root_pos));
        println!("Stacked pawns penalty: {}", stacked_pawns(&self.root_pos));
        println!(
            "Protected passed pawns bonus: {}",
            passed_pawns(&self.root_pos)
        );
        println!("Strong squares bonus: {}", bonus(&self.root_pos));
        println!(
            "Pieces on strong squares bonus: {}",
            bonus_for_piece(&self.root_pos)
        );
        println!(
            "Rook on open file bonus: {}",
            bonus_rook_for_open_files(&self.root_pos)
        );
        println!(
            "Rook on semi-open file bonus: {}",
            bonus_rook_for_semi_open_files(&self.root_pos)
        );
        println!(
            "Bonus for rook batteries: {}",
            bonus_for_rook_batteries(&self.root_pos)
        );
        println!(
            "Absolute pin bonus: {}",
            bonus_for_absolute_pins(&self.root_pos)
        );
        println!("Mobility bonus: {}", bonus_for_mobility(&self.root_pos));

        println!();
        println!(
            "Eval: {}",
            evaluate(&self.root_pos, &self.evaluation_table,)
        );
    }

    fn uci_new_game(&mut self) {
        self.root_pos = Position::default();
        self.repetition_table.clear();
        self.transposition_table = Arc::new(TranspositionTable::new(self.options.hash));
        self.age = 0;
    }

    fn test(&mut self) {
        let pos = Position::from_fen(
            "rnb2rk1/pp1p1pp1/4p2p/2b5/2PNN2q/4K3/PP1BP1PP/2RQ1B1R b - - 4 12".to_string(),
        )
        .unwrap();

        self.position(pos, vec![]);
        self.debug();
    }

    fn simulate(&mut self, moves: &[String]) {
        for i in 0..moves.len() {
            let opts = SearchOptions {
                wtime: Some(50000),
                btime: Some(50000),
                ..Default::default()
            };
            let mvs = &moves[..=i];
            dbg!(&mvs);
            let pos = Position::default();
            self.position(pos, mvs.to_vec());
            println!("{}", self.root_pos);
            self.go(opts);
            thread::sleep(std::time::Duration::from_secs(2));
            dbg!();
            dbg!();
            dbg!(self.age);
        }
    }
}

fn uci_info() {
    println!("id name NoName");
    println!("id author Mateusz Burdyna");
    println!("option name Move Overhead type spin default 10 min 0 max 5000");
    println!("option name Threads type spin default 10 min 1 max 1024");
    println!("option name Hash type spin default 16 min 1 max 33554432");
    println!("uciok");
}

fn parse_uci_moves(moves: Vec<String>, pos: &mut Position) -> Result<Table> {
    let mut repetition_table = Table::default();
    repetition_table.last_irreversible[0] = pos.halfmove_clock as usize;

    for mv_str in moves {
        let mv = MOVE_GEN
            .generate_legal_moves(pos)
            .into_iter()
            .find(|mv| mv.to_string() == mv_str)
            .ok_or(anyhow!("Invalid move: {mv_str}"))?;

        let old_pos = pos.clone();
        let _ = pos.make_move(&mv).map_err(anyhow::Error::from);
        repetition_table.push(pos, mv.is_irreversible(&old_pos));
    }

    Ok(repetition_table)
}
