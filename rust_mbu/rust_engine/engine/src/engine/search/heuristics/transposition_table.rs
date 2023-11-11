use sdk::position::Position;

use crate::engine::search::MATE_SCORE;

#[derive(Clone, Copy)]
pub enum HashFlag {
    EXACT,
    ALPHA,
    BETA,
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

    pub fn cashed_value(
        &self,
        node: &Position,
        ply: usize,
        pv_node: bool,
        depth: usize,
        alpha: isize,
        beta: isize,
    ) -> Option<isize> {
        if ply > 0 && !pv_node {
            self.read(node.hash, alpha, beta, depth, ply)
        } else {
            None
        }
    }

    pub fn read(
        &self,
        hash: u64,
        alpha: isize,
        beta: isize,
        depth: usize,
        ply: usize,
    ) -> Option<isize> {
        if let Some(entry) = self.inner[hash as usize % HASH_SIZE] {
            if entry.depth < depth {
                return None;
            }

            if entry.hash != hash {
                return None;
            }

            let mut score = entry.score;

            if score < -MATE_SCORE {
                score += ply as isize;
            } else if score > MATE_SCORE {
                score -= ply as isize;
            }

            match entry.flag {
                HashFlag::EXACT => Some(score),
                HashFlag::BETA => (score >= beta).then_some(beta),
                HashFlag::ALPHA => (score <= alpha).then_some(alpha),
            }
        } else {
            None
        }
    }

    pub fn write(&mut self, hash: u64, score: isize, depth: usize, ply: usize, flag: HashFlag) {
        let mut entry = TranspositionEntry {
            hash,
            depth,
            flag,
            score,
        };

        if entry.score < -MATE_SCORE {
            entry.score -= ply as isize;
        } else if entry.score > MATE_SCORE {
            entry.score += ply as isize;
        }

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
