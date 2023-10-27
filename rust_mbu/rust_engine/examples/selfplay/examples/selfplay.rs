use sdk::{fen::Fen, position::{Position, Piece, Color}, square::Square};

use engine::core::{search::Search, Engine};
use move_gen::{r#move::{MakeMove, Move}, generators::pieces::PinnerGenerator};
use std::thread;

fn run() {
    let fen = "r2q1b1r/1p2pk1p/3p1p2/pNnP4/P1QNPP2/8/1PP4p/2KR3R b - - 0 20";
    // position fen r2q1b1r/1p2pk1p/3p1p2/pNnP4/P1QNPP2/8/1PP4p/2KR3R b - - 0 20 moves e7e5 d5e6
    let mut engine = Engine::default();
    let mut pos = Position::from_fen(fen.to_string()).unwrap();
    let depth = 4;


    let x: Vec<String> = engine
        .move_gen
        .generate_legal_moves(&pos)
        .map(|m| m.to_string())
        .collect();
    dbg!(&x);
    println!("{pos}");
}

fn main() {
    let child = thread::Builder::new()
        .stack_size(32 * 1024 * 1024 * 2)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}
