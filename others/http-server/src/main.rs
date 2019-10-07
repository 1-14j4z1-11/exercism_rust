use http_server::{Method, Response, Server};
use std::sync::{Arc, Mutex};

fn main() {
    let counter = Arc::new(Mutex::new(Counter::new()));
    let mut server = Server::new("localhost", 55001);
    server.add(
        |req| req.path() == "/counter" && req.method() == Method::Get,
        move |_req| {
            counter.lock().unwrap().incr();
            let mut res = Response::new(200);
            res.headers_mut().put("Content-Type", "text/plain");
            res.set_body(
                format!("Counter = {}", counter.lock().unwrap().get())
                    .as_bytes()
                    .to_vec(),
            );
            res
        },
    );
    server.start();
}

struct Counter {
    counter: u64,
}

impl Counter {
    pub fn new() -> Self {
        Counter { counter: 0 }
    }

    pub fn incr(&mut self) {
        self.counter += 1;
    }

    pub fn get(&self) -> u64 {
        self.counter
    }
}
