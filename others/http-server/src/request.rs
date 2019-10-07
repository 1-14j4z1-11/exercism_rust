use crate::header::Headers;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Connect,
    Options,
    Trace,
    Patch,
}

pub struct Request {
    method: Method,
    path: String,
    headers: Headers,
    body: Vec<u8>,
}

impl Request {
    pub fn new(method: Method, path: &str) -> Self {
        Request {
            method,
            path: path.to_string(),
            headers: Headers::default(),
            body: vec![],
        }
    }

    pub fn method(&self) -> Method {
        self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn headers_mut(&mut self) -> &mut Headers {
        &mut self.headers
    }

    pub fn body(&self) -> &[u8] {
        &self.body
    }

    pub fn set_body(&mut self, body: Vec<u8>) {
        self.body = body;
    }
}
