use sdk::position::Position;

#[derive(Clone)]
pub struct RepetitionTable {
    pub table: [u64; 200],
    pub idx: usize,
}

impl Default for RepetitionTable {
    fn default() -> Self {
        Self {
            table: [0; 200],
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
            if self.table[i] == self.table[self.idx] {
                count += 1;
            }
        }

        count >= 3
    }
}
