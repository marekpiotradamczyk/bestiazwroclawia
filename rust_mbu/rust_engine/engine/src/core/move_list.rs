use move_gen::r#move::Move;
use smallvec::SmallVec;

#[derive(Default)]
pub struct MoveList {
    pub moves: SmallVec<[Move; 1024]>,
    pub root_ply: u16,
}

impl MoveList {
    pub fn count_occurrences(&self, mv: &Move) -> usize {
        self.moves
            .iter()
            .skip(u16::max(0, self.root_ply) as usize)
            .filter(|&m| *m == *mv)
            .count()
    }

    pub fn push_root(&mut self, mv: Move) {
        self.moves.push(mv);
        self.root_ply += 1;
    }

    pub fn push(&mut self, mv: Move) {
        self.moves.push(mv);
    }

    pub fn pop(&mut self) {
        self.moves.pop();
    }
}

impl ToString for MoveList {
    fn to_string(&self) -> String {
        self.moves
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    }
}
