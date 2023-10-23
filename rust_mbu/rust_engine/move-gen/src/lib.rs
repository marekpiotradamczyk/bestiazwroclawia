#![allow(unused_imports)]
use std::thread;

#[macro_use]
extern crate log;

use flexi_logger::{DeferredNow, Logger, WriteMode};
use log::Record;
use pretty_env_logger::env_logger::fmt::Style;
use sdk::{
    bitboard::Bitboard,
    fen::Fen,
    position::{Color, Piece, Position},
    square::Square, lookup::king::mask_king_attacks,
};
use xray::XRayGenerator;

pub mod lookup;
mod tests;
pub mod utils;
pub mod xray;
pub mod r#move;
pub mod generators;
