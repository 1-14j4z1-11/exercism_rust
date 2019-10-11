use crate::header::Headers;

pub struct Response {
    status_code: u32,
    headers: Headers,
    body: Vec<u8>,
}

impl Response {
    pub fn new(status_code: u32) -> Self {
        Response {
            status_code,
            headers: Headers::default(),
            body: vec![],
        }
    }

    pub fn status_code(&self) -> u32 {
        self.status_code
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
