extern crate clock;

use clock::console;
use clock::server;
use std::cell::Cell;
use std::io;
use std::sync::{Arc, Mutex};

fn main() {
    let is_finished = Arc::new(Mutex::new(Cell::new(false)));

    server::start("10.60.91.136", 55100, &is_finished);
    console::start(&is_finished);
    
    wait_until_stdin_input(&is_finished);
}

fn wait_until_stdin_input(is_finished: &Arc<Mutex<Cell<bool>>>) {
    let is_finished = is_finished.clone();
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).unwrap();

    is_finished.lock().unwrap().set(true);
}
