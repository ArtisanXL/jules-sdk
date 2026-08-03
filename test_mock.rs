use std::sync::Mutex;

#[derive(Clone, Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn new(status: u16, headers: Vec<(String, String)>, body: Vec<u8>) -> Self {
        Self { status, headers, body }
    }
}

pub struct MockTransport {
    response: std::sync::Mutex<Option<HttpResponse>>,
}

impl MockTransport {
    fn send(&self) -> HttpResponse {
        self.response.lock().unwrap().take().unwrap()
    }
}

fn main() {
    let t = MockTransport {
        response: Mutex::new(Some(HttpResponse::new(200, vec![], vec![])))
    };
    let _r = t.send();
}
