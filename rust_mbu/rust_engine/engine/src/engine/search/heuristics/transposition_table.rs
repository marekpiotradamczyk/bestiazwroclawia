use move_gen::r#move::Move;
use sdk::position::Position;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::engine::search::MATE_SCORE;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashFlag {
    EXACT,
    ALPHA,
    BETA,
}

pub type TTEntry = [AtomicU64; 2];

const HASH_SIZE: usize = 1 << 21;

pub struct TranspositionTable {
    inner: [TTEntry; HASH_SIZE],
}

impl Default for TranspositionTable {
    fn default() -> Self {
        Self {
            inner: [DEFAULT_ENTRY; HASH_SIZE],
        }
    }
}
// 32 Bits for score
pub const SCORE_SHIFT: u64 = 64 - 32;
pub const SCORE_MASK: u64 = 0b11111111111111111111111111111111 << SCORE_SHIFT;

// 7 Bits for depth
pub const DEPTH_SHIFT: u64 = 64 - 32 - 7;
pub const DEPTH_MASK: u64 = 0b1111111 << DEPTH_SHIFT;

// 16 Bits for move
pub const MOVE_SHIFT: u64 = 64 - 32 - 7 - 16;
pub const MOVE_MASK: u64 = 0b1111111111111111 << MOVE_SHIFT;

//TODO: MORE???
// 7 Bits for age
pub const AGE_SHIFT: u64 = 64 - 32 - 7 - 16 - 7;
pub const AGE_MASK: u64 = 0b1111111 << AGE_SHIFT;

// 2 Bits for flag
pub const FLAG_MASK: u64 = 0b11;

fn pack_tt_entry(score: i32, mv: Option<Move>, depth: usize, age: usize, flag: HashFlag) -> u64 {
    let mut packed = 0;

    // Bits 0-13
    packed |= (score as u64) << SCORE_SHIFT;
    packed |= (depth as u64) << DEPTH_SHIFT;
    packed |= (mv.unwrap_or(Move::null()).inner as u64) << MOVE_SHIFT;
    packed |= (age as u64) << AGE_SHIFT;
    packed |= flag as u64;

    packed
}

#[allow(clippy::declare_interior_mutable_const)]
const DEFAULT_ENTRY: TTEntry = [AtomicU64::new(0), AtomicU64::new(0)];

#[allow(clippy::too_many_arguments)]
impl TranspositionTable {
    pub fn cashed_value(
        &self,
        node: &Position,
        ply: usize,
        pv_node: bool,
        depth: usize,
        alpha: i32,
        beta: i32,
    ) -> (Option<i32>, Option<Move>) {
        if ply > 0 && !pv_node {
            self.read(node.hash, alpha, beta, depth, ply)
        } else {
            (None, None)
        }
    }

    pub fn read(
        &self,
        hash: u64,
        alpha: i32,
        beta: i32,
        depth: usize,
        ply: usize,
    ) -> (Option<i32>, Option<Move>) {
        let [hash_lock, entry_lock] = &self.inner[hash as usize % HASH_SIZE];
        let tt_hash = hash_lock.load(Ordering::Relaxed);
        let tt_entry = entry_lock.load(Ordering::Relaxed);
        if tt_hash != 0 {
            if tt_hash != hash {
                return (None, None);
            }

            if get_depth(tt_entry) < depth {
                return (None, get_move(tt_entry));
            }

            let mut score = get_score(tt_entry);

            if score < -MATE_SCORE {
                score += ply as i32;
            } else if score > MATE_SCORE {
                score -= ply as i32;
            }

            (
                match get_flag(tt_entry) {
                    HashFlag::EXACT => Some(score),
                    HashFlag::BETA => (score >= beta).then_some(beta),
                    HashFlag::ALPHA => (score <= alpha).then_some(alpha),
                },
                get_move(tt_entry),
            )
        } else {
            (None, None)
        }
    }

    pub fn write(
        &self,
        hash: u64,
        mut score: i32,
        mv: Option<Move>,
        depth: usize,
        ply: usize,
        flag: HashFlag,
        age: usize,
    ) {
        if score < -MATE_SCORE {
            score -= ply as i32;
        } else if score > MATE_SCORE {
            score += ply as i32;
        }

        let new_entry = pack_tt_entry(score, mv, depth, age, flag);

        let index = hash as usize % HASH_SIZE;

        let [old_hash_lock, old_entry_lock] = &self.inner[index];
        let old_hash = old_hash_lock.load(Ordering::Relaxed);
        let old_entry = old_entry_lock.load(Ordering::Relaxed);

        if old_hash != 0 {
            let old_age = get_age(old_entry);
            let old_depth = get_depth(old_entry);

            let replace = (old_age < age) || (old_age == age && old_depth < depth);

            if replace {
                old_entry_lock.store(new_entry, Ordering::Relaxed);
                old_hash_lock.store(hash, Ordering::Relaxed);
            }
        } else {
            old_entry_lock.store(new_entry, Ordering::Relaxed);
            old_hash_lock.store(hash, Ordering::Relaxed);
        }
    }
}

pub fn get_depth(packed: u64) -> usize {
    ((packed & DEPTH_MASK) >> DEPTH_SHIFT) as usize
}

pub fn get_score(packed: u64) -> i32 {
    ((packed & SCORE_MASK) >> SCORE_SHIFT) as i32
}

pub fn get_move(packed: u64) -> Option<Move> {
    let inner = (packed & MOVE_MASK) >> MOVE_SHIFT;

    if inner == 0 {
        None
    } else {
        Some(Move {
            inner: inner as u16,
        })
    }
}

pub fn get_age(packed: u64) -> usize {
    ((packed & AGE_MASK) >> AGE_SHIFT) as usize
}

pub fn get_flag(packed: u64) -> HashFlag {
    match packed & FLAG_MASK {
        0 => HashFlag::EXACT,
        1 => HashFlag::ALPHA,
        2 => HashFlag::BETA,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use move_gen::r#move::{Move, MoveKind};
    use sdk::square::Square;

    use super::{pack_tt_entry, TranspositionTable};

    fn test_tt() {
        let tt = TranspositionTable::default();
        tt.write(11, 5, None, 4, 0, super::HashFlag::EXACT, 0);
        tt.write(11, 6, None, 3, 0, super::HashFlag::EXACT, 1);
        tt.write(11, 7, None, 5, 0, super::HashFlag::EXACT, 1);
        tt.write(9, 70, None, 5, 0, super::HashFlag::EXACT, 2);

        assert_eq!(tt.read(11, 0, 0, 0, 0), (Some(7), None));
    }

    #[test]
    fn test_pack() {
        let depth = 9;
        let score = -12345;
        let mv = Some(Move::new(Square::A3, Square::B7, None, &MoveKind::Capture));
        let age = 7;
        let flag = super::HashFlag::EXACT;

        let packed = pack_tt_entry(score, mv, depth, age, flag);

        assert_eq!(super::get_depth(packed), depth);
        assert_eq!(super::get_score(packed), score);
        assert_eq!(super::get_move(packed), mv);
        assert_eq!(super::get_age(packed), age);
        assert_eq!(super::get_flag(packed), flag);
    }
}
