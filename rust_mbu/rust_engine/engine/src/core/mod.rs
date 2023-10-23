use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

use move_gen::generators::movegen::MoveGen;
use sdk::position::Position;

use self::move_list::MoveList;

pub mod evaluate;
pub mod search;
mod move_list;

#[derive(Default)]
pub struct Engine {
    pub nodes_evaluated: usize,
    pub move_gen: MoveGen,
    pub move_list: MoveList,
    pub pos: Position,
}

pub fn hash_pos(pos: &Position) -> u64 {
    let mut hasher = DefaultHasher::new();
    pos.hash(&mut hasher);
    hasher.finish()
}
