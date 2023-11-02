use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

use move_gen::generators::movegen::MoveGen;
use sdk::position::Position;

pub mod evaluate;
pub mod search;
pub mod move_order;

#[derive(Default)]
pub struct Engine {
    pub total_nodes_evaluated: usize,
    pub nodes_evaluated: usize,
    pub move_gen: MoveGen,
    pub pos: Position,
}

pub fn hash_pos(pos: &Position) -> u64 {
    let mut hasher = DefaultHasher::new();
    pos.hash(&mut hasher);
    hasher.finish()
}
