use move_gen::{generators::movegen::MoveGen, r#move::{make_move::MakeMove, undo_move::UndoMove}};
use sdk::{fen::Fen, position::Position};

use crate::engine::search::MoveList;

pub fn perft(depth: u32, move_gen: &MoveGen, pos: &mut Position) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;

    let legal_moves = move_gen.generate_legal_moves(pos).collect::<MoveList>();

    dbg!(&pos.to_fen());
    for mv in legal_moves {
        let mut next = pos.clone();
        pos.make_move(&mv).unwrap();
        println!("{}", pos.to_fen());
        println!("Move: {:?}, kind: {:?}", mv, mv.kind());
        nodes += perft(depth - 1, move_gen, &mut next);
        pos.undo_move(&mv).unwrap();
    }

    nodes
}
