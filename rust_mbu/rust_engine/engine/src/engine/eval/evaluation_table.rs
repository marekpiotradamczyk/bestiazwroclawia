use std::sync::atomic::{AtomicI32, AtomicU64, Ordering};

pub const EVAL_TABLE_SIZE: usize = 1 << 20;

pub type TableEntry = (AtomicU64, AtomicI32);

pub struct EvaluationTable {
    pub table: [TableEntry; EVAL_TABLE_SIZE],
}

pub const DEFAULT_ENTRY: TableEntry = (AtomicU64::new(0), AtomicI32::new(0));

impl Default for EvaluationTable {
    fn default() -> Self {
        EvaluationTable {
            table: [DEFAULT_ENTRY; EVAL_TABLE_SIZE],
        }
    }
}

impl EvaluationTable {
    pub fn write(&self, hash: u64, value: i32) {
        let index = (hash % EVAL_TABLE_SIZE as u64) as usize;
        self.table[index].0.store(hash, Ordering::Relaxed);
        self.table[index].1.store(value, Ordering::Relaxed);
    }

    pub fn read(&self, hash: u64) -> Option<i32> {
        let index = (hash % EVAL_TABLE_SIZE as u64) as usize;
        let entry = &self.table[index];
        if entry.0.load(Ordering::Relaxed) == hash {
            Some(entry.1.load(Ordering::Relaxed))
        } else {
            None
        }
    }
}
