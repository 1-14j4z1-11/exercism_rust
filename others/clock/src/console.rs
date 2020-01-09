use chrono::Local;
use std::cell::Cell;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn start(is_finished: &Arc<Mutex<Cell<bool>>>) {
    let is_finished = is_finished.clone();

    thread::spawn(move || {
        while !is_finished.lock().unwrap().get() {
            let time = Local::now().format("%Y-%m-%d %H:%M:%S");

            print!("{}\r", time);
            thread::sleep(Duration::from_millis(5));
        }
    });
}
