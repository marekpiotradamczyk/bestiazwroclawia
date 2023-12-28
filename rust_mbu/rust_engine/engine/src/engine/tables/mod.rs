use std::cmp::Ordering;

use sdk::position::Position;

#[derive(Default, Clone)]
pub enum TTEntryFlag {
    #[default]
    LowerBound,
    UpperBound,
}

#[derive(Default)]
pub struct TTEntry {
    key: u16,
    score: i32,
    value: i32,
    depth: u8,
    flag: TTEntryFlag,
    age: usize,
    checksum: u64,
}

pub struct TT {
    data: Vec<TTEntry>,
    count: usize,
}

impl TT {
    pub fn new(size_in_mb: usize) -> TT {
        let count = size_in_mb * 1024 * 1024 / std::mem::size_of::<TTEntry>();
        let mut data = Vec::with_capacity(count);
        for _ in 0..count {
            data.push(TTEntry {
                key: 0,
                score: 0,
                value: 0,
                depth: 0,
                flag: TTEntryFlag::LowerBound,
                age: 0,
                checksum: 0,
            });
        }
        TT { data, count }
    }

    pub fn clear(&mut self) {
        for i in 0..self.count {
            self.data[i].key = 0;
        }
    }

    pub fn probe(&self, pos: &Position) -> Option<&TTEntry> {
        let entry = &self.data[pos.hash as usize % self.count]; 

        entry.is_valid(pos).then_some(entry)
    }

    pub fn write(&mut self, pos: &Position, score: i32, value: i32, depth: u8, flag: TTEntryFlag, age: usize) {
        let old_entry = &mut self.data[pos.hash as usize % self.count];
        let mut new_entry = TTEntry {
            key: key(pos),
            score,
            value,
            depth,
            flag,
            age,
            checksum: 0,
        };

        if new_entry > *old_entry {
            new_entry.checksum = new_entry.checksum();
            *old_entry = new_entry;
        }
    }
}

impl TTEntry {
    pub fn is_valid(&self, pos: &Position) -> bool {
        self.key != 0 && self.key == key(pos) && self.checksum() == self.checksum
    }

    pub fn checksum(&self) -> u64 {
        self.key as u64
            ^ self.score as u64
            ^ self.value as u64
            ^ self.depth as u64
            ^ self.flag.clone() as u64
            ^ self.age as u64
    }
}

impl PartialOrd for TTEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.age
                .cmp(&other.age)
                .then_with(|| self.depth.cmp(&other.depth)),
        )
    }
}

impl PartialEq for TTEntry {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.age == other.age && self.depth == other.depth
    }
}

pub const fn key(pos: &Position) -> u16 {
    (pos.hash & 0xFFFF) as u16
}
