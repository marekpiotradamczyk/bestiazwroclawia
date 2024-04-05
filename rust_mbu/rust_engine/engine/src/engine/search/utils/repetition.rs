use sdk::position::Position;

use crate::engine::search::MAX_PLY;

pub const DEFAULT_TABLE_SIZE: usize = MAX_PLY * 10;

#[derive(Clone)]
pub struct RepetitionTable {
    pub table: [u64; DEFAULT_TABLE_SIZE],
    pub last_irreversible: [usize; DEFAULT_TABLE_SIZE],
    pub idx: usize,
}

impl Default for RepetitionTable {
    fn default() -> Self {
        Self {
            table: [0; DEFAULT_TABLE_SIZE],
            idx: 0,
            last_irreversible: [0; DEFAULT_TABLE_SIZE],
        }
    }
}

impl RepetitionTable {
    pub fn push(&mut self, pos: &Position, is_irreversible: bool) {
        self.table[self.idx] = pos.hash;
        self.idx += 1;

        if is_irreversible {
            self.last_irreversible[self.idx] = self.idx;
        } else {
            self.last_irreversible[self.idx] = self.last_irreversible[self.idx - 1];
        }
    }

    pub fn decrement(&mut self) {
        self.idx -= 1;
    }

    pub fn repetitions(&self) -> i32 {
        let mut count = 0;
        for i in 0..self.idx {
            if self.table[i] == self.table[self.idx - 1] {
                count += 1;
            }
        }

        count
    }

    pub fn is_draw_by_fifty_moves_rule(&self) -> bool {
        self.idx - self.last_irreversible[self.idx] >= 100
    }

    pub fn clear(&mut self) {
        self.idx = 0;
        self.table = [0; DEFAULT_TABLE_SIZE];
    }
}

#[cfg(test)]
mod tests {
    use move_gen::r#move::{make_move::MakeMove, Move, MoveKind};
    use sdk::{position::Position, square::Square};

    use super::RepetitionTable;

    #[test]
    fn test_repetition() {
        let mut rep = RepetitionTable::default();

        let mut pos = Position::default();

        for _ in 0..25 {
            rep.push(&pos, false);
            let _ = pos.make_move(&Move::new(Square::B1, Square::A3, None, &MoveKind::Quiet));
            rep.push(&pos, false);
            let _ = pos.make_move(&Move::new(Square::B8, Square::A6, None, &MoveKind::Quiet));
            rep.push(&pos, false);
            let _ = pos.make_move(&Move::new(Square::A3, Square::B1, None, &MoveKind::Quiet));
            rep.push(&pos, false);
            let _ = pos.make_move(&Move::new(Square::A6, Square::B8, None, &MoveKind::Quiet));
        }

        assert!(rep.repetitions() >= 2);
        assert!(rep.is_draw_by_fifty_moves_rule());
    }
}
