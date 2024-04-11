#![warn(clippy::pedantic)]
#![allow(clippy::inline_always)]
#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

extern crate log;

pub mod bitboard;
pub mod fen;
pub mod hash;
pub mod lookup;
pub mod position;
pub mod square;
