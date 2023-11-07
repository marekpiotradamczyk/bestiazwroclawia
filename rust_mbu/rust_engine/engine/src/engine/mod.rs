use std::{
    sync::{
        mpsc::{channel, Sender},
        Arc,
    },
    thread,
};

use crate::uci::{uci_commands::Command, Result};
use move_gen::{generators::movegen::MoveGen, r#move::MakeMove};
use sdk::position::{Color, Position};

use crate::engine::engine_options::EngineOptions;

use anyhow::anyhow;

use self::search::{
    utils::{repetition::RepetitionTable, time_control::SearchOptions},
    Search, STOPPED,
};

pub mod engine_options;
pub mod eval;
pub mod search;

#[derive(Default)]
pub struct Engine {
    pub root_pos: Position,
    pub move_gen: Arc<MoveGen>,
    pub repetition_table: RepetitionTable,
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
            .stack_size(32 * 1024 * 1024 * 2)
            .spawn(run)
            .unwrap();

        tx
    }

    pub fn go(&mut self, options: SearchOptions) {
        *STOPPED.lock().unwrap() = false;
        let pos = self.root_pos.clone();
        let move_gen = self.move_gen.clone();
        let is_white = pos.turn == Color::White;
        let rep_table = self.repetition_table.clone();

        let run = move || {
            let mut search = Search::new(options, move_gen, is_white, rep_table);
            search.search(&pos);
        };

        thread::Builder::new()
            .stack_size(32 * 1024 * 1024 * 2 * 8)
            .spawn(run)
            .unwrap();
    }

    pub fn stop(&mut self) {
        *STOPPED.lock().unwrap() = true;
    }

    fn position(&mut self, mut pos: Position, moves: Vec<String>) {
        match parse_uci_moves(moves, &mut pos, &self.move_gen) {
            Ok(repetition_table) => {
                self.root_pos = pos;
                self.repetition_table = repetition_table;
            }
            Err(e) => println!("{e}"),
        }
    }
}

fn uci_info() {
    println!("id name NoName");
    println!("id author Mateusz Burdyna");
    println!("option name Move Overhead type spin default 10 min 0 max 5000");
    println!("option name Threads type spin default 1 min 1 max 1024");
    println!("option name Hash type spin default 16 min 1 max 33554432");
    println!("uciok");
}

fn parse_uci_moves(
    moves: Vec<String>,
    pos: &mut Position,
    move_gen: &MoveGen,
) -> Result<RepetitionTable> {
    let mut repetition_table = RepetitionTable::default();

    for mv_str in moves {
        let mv = move_gen
            .generate_legal_moves(pos)
            .find(|mv| mv.to_string() == mv_str)
            .ok_or(anyhow!("Invalid move: {mv_str}"))?;

        let _ = pos.make_move(&mv).map_err(anyhow::Error::from);
        repetition_table.push(pos);
    }

    Ok(repetition_table)
}
