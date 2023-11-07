#[derive(Clone, Copy)]
pub enum HashFlag {
    EXACT,
    LOWERBOUND,
    UPPERBOUND,
}

#[derive(Clone, Copy)]
pub struct TranspositionEntry {
    pub hash: u64,
    pub depth: usize,
    pub flag: HashFlag,
    pub score: isize,
}

const HASH_SIZE: usize = 1 << 20;

pub struct TranspositionTable {
    inner: [Option<TranspositionEntry>; HASH_SIZE],
}

impl Default for TranspositionTable {
    fn default() -> Self {
        Self {
            inner: [None; HASH_SIZE],
        }
    }
}

impl TranspositionTable {
    pub fn clear(&mut self) {
        self.inner = [None; HASH_SIZE];
    }

    pub fn read(&self, hash: u64, alpha: isize, beta: isize, depth: usize) -> Option<isize> {
        if let Some(entry) = self.inner[hash as usize % HASH_SIZE] {
            if entry.depth < depth {
                return None;
            }

            if entry.hash != hash {
                return None;
            }

            match entry.flag {
                HashFlag::EXACT => Some(entry.score),
                HashFlag::LOWERBOUND => {
                    if entry.score >= beta {
                        Some(beta)
                    } else {
                        None
                    }
                }
                HashFlag::UPPERBOUND => {
                    if entry.score <= alpha {
                        Some(alpha)
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        }
    }

    pub fn write(&mut self, hash: u64, score: isize, depth: usize, flag: HashFlag) {
        let entry = TranspositionEntry {
            hash,
            depth,
            flag,
            score,
        };

        let index = hash as usize % HASH_SIZE;

        if let Some(old_entry) = self.inner[index] {
            if old_entry.depth < depth {
                self.inner[index] = Some(entry);
            }
        } else {
            self.inner[index] = Some(entry);
        }
    }
}
