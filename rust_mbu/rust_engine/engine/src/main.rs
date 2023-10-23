pub mod core;
mod uci;

use uci::start_uci;

use std::thread;

pub fn run() {
    start_uci();
}

pub fn main() {
    let child = thread::Builder::new()
        .stack_size(32 * 1024 * 1024 * 2)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}
