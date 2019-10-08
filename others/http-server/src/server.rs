use crate::{Method, Request, Response};
use std::str::FromStr;
use std::sync::{Mutex, Arc};
use std::thread;
use std::borrow::Borrow;
use tiny_http;

type RouteTable<'a> = Vec<(
    Box<Fn(&Request) -> bool + Send + 'a>,
    Box<Fn(&Request) -> Response + Send + 'a>,
)>;

pub struct Server {
    host: String,
    port: u16,
    table: RouteTable<'static>,
}

impl Server {
    pub fn new(host: &str, port: u16) -> Self {
        Server {
            host: host.to_string(),
            port,
            table: vec![],
        }
    }

    pub fn add<F, G>(&mut self, can_respond: F, respond: G)
    where
        F: Fn(&Request) -> bool + Send + 'static,
        G: Fn(&Request) -> Response + Send + 'static,
    {
        self.table
            .push((Box::new(can_respond), Box::new(respond)));
    }

    pub fn start(self) {
        let server = tiny_http::Server::http(format!("{}:{}", self.host, self.port)).unwrap();
        let mutex_table = Arc::new(Mutex::new(self.table));

        for mut tiny_req in server.incoming_requests() {
            let clone_table = mutex_table.clone();
            thread::spawn(move || {
                let req = convert_from_tiny_request(&mut tiny_req);
                let res = Self::respond(clone_table.lock().unwrap().borrow(), &req);
                let tiny_res = convert_to_tiny_response(&res);
                tiny_req.respond(tiny_res).unwrap();
            });
        }
    }

    fn respond(table: &RouteTable, req: &Request) -> Response {
        for (can, res) in table {
            if !can(&req) {
                continue;
            }

            return res(&req);
        }

        Self::default_response()
    }

    fn default_response() -> Response {
        Response::new(404)
    }
}

fn convert_from_tiny_request(tiny_req: &mut tiny_http::Request) -> Request {
    let path = tiny_req.url();
    let method = match tiny_req.method() {
        tiny_http::Method::Get => Method::Get,
        tiny_http::Method::Post => Method::Post,
        tiny_http::Method::Put => Method::Put,
        tiny_http::Method::Delete => Method::Delete,
        tiny_http::Method::Head => Method::Head,
        tiny_http::Method::Connect => Method::Connect,
        tiny_http::Method::Options => Method::Options,
        tiny_http::Method::Trace => Method::Trace,
        tiny_http::Method::Patch => Method::Patch,
        _ => Method::Get,
    };

    let mut req = Request::new(method, path);

    for header in tiny_req.headers() {
        req.headers_mut().put(
            header.field.as_str().to_string().as_str(),
            header.value.as_str().to_string().as_str(),
        );
    }

    let reader = tiny_req.as_reader();
    let mut body = vec![];

    match reader.read_to_end(&mut body) {
        Ok(_) => req.set_body(body),
        Err(_) => {}
    }

    req
}

fn convert_to_tiny_response<'a>(res: &'a Response) -> tiny_http::Response<&'a [u8]> {
    let mut headers = vec![];

    for key in res.headers().keys() {
        headers.push(
            tiny_http::Header::from_str(&format!("{:?}:{:?}", key, res.headers().get(&key)))
                .unwrap(),
        );
    }

    tiny_http::Response::new(
        tiny_http::StatusCode::from(res.status_code()),
        headers,
        &res.body()[..],
        Some(res.body().len()),
        None,
    )
}
