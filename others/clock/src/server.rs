extern crate iron;

use chrono::Local;
use iron::prelude::*;
use iron::status;
use std::cell::Cell;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn start(host: &str, port: u16, is_finished: &Arc<Mutex<Cell<bool>>>) {
    let is_finished = is_finished.clone();
    let host = host.to_string();

    thread::spawn(move || {
        let mut server = Iron::new(handle_request)
            .http(format!("{}:{}", host, port))
            .unwrap();

        while !is_finished.lock().unwrap().get() {
            thread::sleep(Duration::from_millis(5));
        }

        server.close().unwrap();
    });
}

fn handle_request(_req: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();
    let time = Local::now().format("%Y-%m-%d %H:%M:%S");

    res.set_mut(status::Ok).set_mut(format!("{}", time));

    Ok(res)
}
