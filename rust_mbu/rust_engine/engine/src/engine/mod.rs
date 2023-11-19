use std::{
    sync::{
        atomic::Ordering,
        mpsc::{channel, Sender},
        Arc,
    },
    thread,
};

use crate::{
    engine::{eval::evaluate, search::heuristics::static_exchange_evaluation::static_exchange_evaluation},
    uci::{uci_commands::Command, Result},
};
use move_gen::{generators::movegen::MoveGen, r#move::{MakeMove, Move, MoveKind}};
use sdk::{
    fen::Fen,
    position::{Color, Position}, square::Square,
};

use crate::engine::engine_options::EngineOptions;

use anyhow::anyhow;

use self::search::{
    heuristics::transposition_table::TranspositionTable,
    parallel::Search,
    utils::{repetition::RepetitionTable, time_control::SearchOptions},
    STOPPED,
};

pub mod engine_options;
pub mod eval;
pub mod search;

#[derive(Default)]
pub struct Engine {
    pub root_pos: Position,
    pub move_gen: Arc<MoveGen>,
    pub repetition_table: RepetitionTable,
    pub transposition_table: Arc<TranspositionTable>,
    pub options: EngineOptions,
}

impl Engine {
    pub fn handle_command(&mut self, command: Command) {
        match command {
            Command::Uci => uci_info(),
            Command::Go(options) => self.go(options),
            Command::Stop => self.stop(),
            Command::Position(pos, moves) => self.position(pos, moves),
            Command::SetOption(name, value) => self.set_option(name, value),
            Command::IsReady => println!("readyok"),
            Command::Debug => self.debug(),
            Command::UciNewGame => self.uci_new_game(),
            Command::Test => self.test(),

            _ => {}
        };
    }

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

        thread::Builder::new()
            .stack_size(32 * 1024 * 1024 * 4)
            .spawn(run)
            .unwrap();

        tx
    }

    pub fn go(&mut self, options: SearchOptions) {
        STOPPED.store(false, Ordering::Relaxed);
        let pos = self.root_pos.clone();
        let move_gen = self.move_gen.clone();
        let is_white = pos.turn == Color::White;
        let rep_table = self.repetition_table.clone();
        let transposition_table = self.transposition_table.clone();
        let engine_options = self.options;

        let run = move || {
            let mut search = Search::new(
                options,
                engine_options,
                move_gen,
                is_white,
                rep_table,
                transposition_table,
            );
            search.search(&pos);
        };

        thread::Builder::new()
            .stack_size(32 * 1024 * 1024 * 2 * 8)
            .spawn(run)
            .unwrap();
    }

    pub fn stop(&mut self) {
        STOPPED.store(true, Ordering::Relaxed);
    }

    fn position(&mut self, mut pos: Position, moves: Vec<String>) {
        self.repetition_table.clear();
        match parse_uci_moves(moves, &mut pos, &self.move_gen) {
            Ok(repetition_table) => {
                self.root_pos = pos;
                self.repetition_table = repetition_table;
            }
            Err(e) => println!("{e}"),
        }
    }

    fn debug(&self) {
        println!("{}", self.root_pos);
        let moves = self.move_gen.generate_legal_moves(&self.root_pos);
        for mv in moves {
            print!("{} ", mv);
        }
        println!("Eval: {}", evaluate(&self.root_pos));
    }

    fn uci_new_game(&mut self) {
        self.root_pos = Position::default();
        self.repetition_table.clear();
        self.transposition_table = Arc::new(TranspositionTable::default());
    }

    fn test(&mut self) {
        let pos = Position::from_fen("1k1r1br1/p1p1pppp/7q/1PpP4/Q3PPPP/P7/4N2R/R4K2 w - - 1 29".to_string()).unwrap();

        let mv = Move::new(Square::D5, Square::D6, None, &MoveKind::Capture);

        println!("{}", pos);
        println!("{}", mv);

        dbg!(static_exchange_evaluation(&self.move_gen, &pos, &mv));
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

fn parse_uci_moves(
    moves: Vec<String>,
    pos: &mut Position,
    move_gen: &MoveGen,
) -> Result<RepetitionTable> {
    let mut repetition_table = RepetitionTable::default();
    repetition_table.last_irreversible[0] = pos.halfmove_clock as usize;

    for mv_str in moves {
        let mv = move_gen
            .generate_legal_moves(pos)
            .find(|mv| mv.to_string() == mv_str)
            .ok_or(anyhow!("Invalid move: {mv_str}"))?;

        let old_pos = pos.clone();
        let _ = pos.make_move(&mv).map_err(anyhow::Error::from);
        repetition_table.push(pos, mv.is_irreversible(&old_pos));
    }

    Ok(repetition_table)
}
