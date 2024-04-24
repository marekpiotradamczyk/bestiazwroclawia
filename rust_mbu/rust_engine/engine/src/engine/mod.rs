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
            strong_squares::{bonus, bonus_for_piece, strong_squares},
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

use derivative::Derivative;

pub mod eval;
pub mod options;
pub mod search;

#[derive(Derivative)]
#[derivative(Default)]
pub struct Engine {
    pub root_pos: Position,
    pub repetition_table: Table,
    pub transposition_table: Arc<TranspositionTable>,
    pub evaluation_table: Arc<EvaluationTable>,
    pub options: Options,
    pub age: usize,
    #[derivative(Default(value = "true"))]
    pub ready: bool,
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

    pub fn go(&mut self, options: SearchOptions) {
        STOPPED.store(false, Ordering::Relaxed);
        let pos = self.root_pos.clone();
        let is_white = pos.turn == Color::White;
        let rep_table = self.repetition_table.clone();
        let transposition_table = self.transposition_table.clone();
        let eval_table = self.evaluation_table.clone();
        let engine_options = self.options;
        let age = self.age;

        let run = move || {
            let mut search = Search::new(
                options,
                engine_options,
                is_white,
                rep_table,
                transposition_table,
                eval_table,
                age,
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
        println!(
            "Strong squares bonus: {}",
            bonus_for_strong_squares(&self.root_pos, self.root_pos.turn)
        );
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
