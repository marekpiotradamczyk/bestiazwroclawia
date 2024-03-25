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
    lookup::king::mask_king_attacks,
    position::{Color, Piece, Position},
    square::Square,
};
use xray::XRayGenerator;

pub mod generators;
pub mod lookup;
pub mod r#move;
mod tests;
pub mod utils;
pub mod xray;
