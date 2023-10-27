use std::io;

use itertools::Itertools;
use move_gen::r#move::{MakeMove, Move};
use sdk::{fen::Fen, position::Position};
use timeit::timeit_loops;

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
            .unwrap_or((&"", vec![]));

        match command.to_lowercase().as_str() {
            "quit" => return,
            "isready" => println!("readyok"),
            "uci" => uci(),
            "ucinewgame" => position(vec!["startpos"], &mut engine),
            "position" => position(args, &mut engine),
            "go" => go(args, &mut engine),
            "setoption" => {}
            "perft" => {}
            "profile" => {}
            "stop" => {}
            "ponderhit" => {}
            "printfen" => {}
            "bench" => bench(),
            _ => println!("Unknown command: {}", command),
        }
    }
}

fn uci() {
    println!("id name NoName v0.1.0");
    println!("id author Mateusz Burdyna");
    println!("uciok");
}

fn bench() {
    let mut engine = Engine::default();
    let mut pos = Position::default();
    let time = timeit_loops!(1, {
        while let Some((_, mv)) = engine.search(&pos, 3) {
            let _ = pos.make_move(&mv).unwrap();
        }
    });

    let nps = engine.nodes_evaluated as f64 / time;

    println!("{nps:.2} nps");
}

fn position(args: Vec<&str>, engine: &mut Engine) {
    if args.is_empty() {
        println!("{}", engine.pos);
        return;
    }
    let input = args.first().unwrap();

    let (mut pos, idx) = if *input == "startpos" {
        (Position::default(), 1)
    } else {
        let mut iter = args.iter().skip(1).take_while(|s| **s != "moves");
        let idx = iter.clone().count() + 1;
        let fen = iter.join(" ");

        if let Ok(pos) = Position::from_fen(fen) {
            (pos, idx)
        } else {
            println!("Invalid fen: {input}");
            return;
        }
    };

    let moves = &args[idx..];

    if let Some(first) = moves.first() {
        if *first == "moves" {
            for mv in moves.iter().skip(1) {
                if let Some(mv) = parse_move(mv.to_string(), engine, &pos) {
                    let _ = pos.make_move(&mv);
                } else {
                    println!("Invalid move {mv}");
                    return;
                }
            }
        } else {
            println!("Unexpected token {first}");
            return;
        }
    }
    // position fen rnbqkbnr/1ppp1ppp/p7/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 1 3 moves g8f6 h5f3

    engine.pos = pos;
}

fn go(args: Vec<&str>, engine: &mut Engine) {
    let depth = if let Some(first) = args.first() {
        if *first != "depth" {
            println!("Unexpected token {first}, expected: depth");
            return;
        }

        if let Some(depth_str) = args.get(1) {
            if let Ok(depth) = depth_str.parse::<usize>() {
                depth
            } else {
                println!("Invalid depth {depth_str}");
                return;
            }
        } else {
            println!("Missing depth");
            return;
        }
    } else {
        4
    };

    let (_, mv) = engine.search(&engine.pos.clone(), depth).unwrap();

    println!("bestmove {mv}");
}

fn parse_move(mv: String, engine: &Engine, pos: &Position) -> Option<Move> {
    engine
        .move_gen
        .generate_legal_moves(pos)
        .find(|m| m.to_string() == mv)
}
