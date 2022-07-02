use std::{io::Write, fmt::Display};

use super::{StatusCode, status_code};

#[derive(Debug)]
pub struct ResponseHeader {
    status_code: StatusCode,
    content_length: u64,
}

impl Display for ResponseHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let content_length_str = self.content_length.to_string();
        let content_length_fmt = ["Content-Length: ", &content_length_str[..]].concat();
        write!(
            f, "{}\r\n{}",
            self.status_code,
            content_length_fmt,
        )
    }
}

#[derive(Debug)]
pub struct Response {
    response_header: ResponseHeader,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        let content_length: u64 = match &body {
            Some(b) => b.len() as u64,
            None => 0
        };

        let response_header = ResponseHeader { status_code, content_length };
        Response { response_header, body }
    }

    pub fn send(&self, stream: &mut impl Write) -> std::io::Result<()> {
        let body = match &self.body {
            Some(s) => s,
            None => "",
        };
        write!(
            stream,
            "HTTP/1.1 {}\r\n\r\n{}",
            self.response_header,
            body
        )
    }
}
