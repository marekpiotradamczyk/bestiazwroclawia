use std::sync::atomic::{AtomicI32, AtomicU64, Ordering};

pub type TableEntry = (AtomicU64, AtomicI32);

pub struct EvaluationTable {
    pub table: Vec<TableEntry>,
    pub size: usize,
}

impl Default for EvaluationTable {
    fn default() -> Self {
        Self::new(16)
    }
}

impl EvaluationTable {
    pub fn new(size_in_mb: usize) -> Self {
        let size = size_in_mb * 1024 * 1024 / std::mem::size_of::<TableEntry>();
        let mut table = Vec::with_capacity(size);
        for _ in 0..size {
            table.push((AtomicU64::new(0), AtomicI32::new(0)));
        }
        EvaluationTable { table, size }
    }

    pub fn write(&self, hash: u64, value: i32) {
        let index = (hash % self.size as u64) as usize;
        self.table[index].0.store(hash, Ordering::Relaxed);
        self.table[index].1.store(value, Ordering::Relaxed);
    }

    pub fn read(&self, hash: u64) -> Option<i32> {
        let index = (hash % self.size as u64) as usize;
        let entry = &self.table[index];
        if entry.0.load(Ordering::Relaxed) == hash {
            Some(entry.1.load(Ordering::Relaxed))
        } else {
            None
        }
    }
}
