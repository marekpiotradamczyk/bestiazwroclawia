#![warn(clippy::pedantic)]
#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
pub mod engine;
pub mod uci;

use uci::start;

pub fn main() {
    start();
}
