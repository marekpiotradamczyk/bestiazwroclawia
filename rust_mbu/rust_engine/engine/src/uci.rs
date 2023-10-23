use std::io;

use itertools::Itertools;
use move_gen::r#move::{MakeMove, Move};
use sdk::{fen::Fen, position::Position};

use crate::core::{search::Search, Engine};

pub fn start_uci() {
    println!("ready");

    let mut engine = Engine::default();

    let stdin = io::stdin();

    loop {
        let mut line = String::new();
        stdin.read_line(&mut line).expect("Failed to read line");

        let split = line.split_whitespace().collect_vec();

        let (command, args) = split
            .split_first()
            .map(|(cmd, args)| (cmd, args.to_vec()))
            .unwrap();

        match command.to_lowercase().as_str() {
            "quit" => return,
            "isready" => println!("readyok"),
            "uci" => uci(),
            "ucinewgame" => {}
            "position" => position(args, &mut engine),
            "go" => go(&mut engine),
            "setoption" => {}
            "perft" => {}
            "profile" => {}
            "stop" => {}
            "ponderhit" => {}
            "printfen" => {}
            _ => println!("Unknown command: {}", command),
        }
    }
}

fn uci() {
    println!("id name NoName v0.1.0");
    println!("id author Mateusz Burdyna");
    println!("uciok");
}

fn position(args: Vec<&str>, engine: &mut Engine) {
    if args.is_empty() {
        println!("{}", engine.pos);
        return;
    }
    let fen = args.first().unwrap();

    engine.pos = if *fen == "startpos" {
        Position::default()
    } else {
        Position::from_fen(fen.to_string()).expect("Couldn't parse FEN")
    };

    let second = args.get(1);
    let mut pos = engine.pos.clone();

    if let Some(second) = second {
        if *second == "moves" {
            for (idx, mv) in args.iter().skip(2).enumerate() {
                if let Some(mv) = parse_move(mv.to_string(), engine) {
                    let _ = pos.make_move(&mv);
                } else {
                    println!("Invalid move: {}.{mv}", { idx + 1 });
                }
            }
        } else {
            println!("Expected 'moves', found '{second}'");
        }

        engine.pos = pos;
    }
}

fn go(engine: &mut Engine) {
    let result = engine.search(&engine.pos.clone(), 3);

    if let Some((_score, mv)) = result {
        println!("bestmove {}", mv);
    }
}

fn parse_move(mv: String, engine: &Engine) -> Option<Move> {
    engine
        .move_gen
        .generate_legal_moves(&engine.pos)
        .find(|m| m.to_string() == mv)
}
