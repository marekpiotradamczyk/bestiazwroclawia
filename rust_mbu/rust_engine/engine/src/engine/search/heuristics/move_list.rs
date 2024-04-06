use arrayvec::ArrayVec;
use move_gen::r#move::Move;

enum State {
    PvMove(Move),
    GenerateCaptures,
    Captures(ArrayVec<Move, 32>),
}

struct MoveList {
    pub pv_move: Move,
}

impl MoveList {
    pub fn next(&mut self) -> Option<Move> {
        todo!()
    }
}
