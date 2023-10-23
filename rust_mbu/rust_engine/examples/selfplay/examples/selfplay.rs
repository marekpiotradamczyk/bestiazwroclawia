use sdk::position::Position;

use engine::core::{search::Search, Engine};
use move_gen::r#move::MakeMove;
use std::thread;

fn run() {
    let mut engine = Engine::default();
    let mut pos = Position::default();
    let depth = 3;

    while let Some((eval, mv)) = engine.search(&pos, depth) {
        let _ = pos.make_move(&mv);
        println!("{pos}");
        println!("bestmove: {mv}, score: {eval}");
    }

    println!("{}", engine.move_list.to_string());
}

fn main() {
    let child = thread::Builder::new()
        .stack_size(32 * 1024 * 1024 * 2)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}
