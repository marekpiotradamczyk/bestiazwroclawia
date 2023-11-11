use sdk::position::Position;

use crate::engine::search::MAX_PLY;

#[derive(Clone)]
pub struct RepetitionTable {
    pub table: [u64; MAX_PLY],
    pub idx: usize,
}

impl Default for RepetitionTable {
    fn default() -> Self {
        Self {
            table: [0; MAX_PLY],
            idx: 0,
        }
    }
}

impl RepetitionTable {
    pub fn push(&mut self, pos: &Position) {
        self.table[self.idx] = pos.hash;
        self.idx += 1;
    }

    pub fn decrement(&mut self) {
        self.idx -= 1;
    }

    pub fn is_repeated(&self) -> bool {
        let mut count = 0;
        for i in 0..self.idx {
            if self.table[i] == self.table[self.idx - 1] {
                count += 1;
            }
        }

        count >= 3
    }

    pub fn clear(&mut self) {
        self.idx = 0;
        self.table = [0; MAX_PLY];
    }
}

#[cfg(test)]
mod tests {
    use move_gen::r#move::{MakeMove, Move, MoveKind};
    use sdk::{position::Position, square::Square};

    use super::RepetitionTable;

    #[test]
    fn test_repetition() {
        let mut rep = RepetitionTable::default();

        let mut pos = Position::default();

        for _ in 0..4 {
            rep.push(&pos);
            let _ = pos.make_move(&Move::new(Square::B1, Square::A3, None, &MoveKind::Quiet));
            rep.push(&pos);
            let _ = pos.make_move(&Move::new(Square::B8, Square::A6, None, &MoveKind::Quiet));
            rep.push(&pos);
            let _ = pos.make_move(&Move::new(Square::A3, Square::B1, None, &MoveKind::Quiet));
            rep.push(&pos);
            let _ = pos.make_move(&Move::new(Square::A6, Square::B8, None, &MoveKind::Quiet));
        }

        dbg!(&rep.table[0..rep.idx]);
        assert!(rep.is_repeated());
    }
}
