#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]

extern crate log;

pub mod bitboard;
pub mod fen;
pub mod hash;
pub mod lookup;
pub mod position;
pub mod square;
