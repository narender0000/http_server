use std::{io::Write, net::TcpStream};

use anyhow::{Context, Ok, Result};
use std::fmt::Display;
pub fn send_response(response: Response, stream: &mut TcpStream) -> Result<()> {
    write!(stream, "HTTP/1.1 ").context("writing protocol")?;
    write!(stream, "{}\r\n", response.code).context("writing protocol")?;
    if response.body.is_some() {
        write!(
            stream,
            "{}{}",
            response.content_type_header(),
            response.content_length_header().unwrap()
        )
        .context("writing headers")?;
    }
    write!(stream, "\r\n").context("writing crlf for headers")?;

    if let Some(body) = &response.body {
        write!(stream, "{body}").context("writing body")?;
    }

    stream
        .flush()
        .context("flushing write so that everything goes out")?;
    Ok(())
}

#[derive(Debug)]
pub enum HttpCode {
    Ok,
    NotFound,
}

impl Display for HttpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (number, message) = match self {
            Self::Ok => (200, "OK"),
            Self::NotFound => (404, "NotFound"),
        };
        write!(f, "{number} {message}")
    }
}

#[derive(Debug)]
pub struct Response {
    pub code: HttpCode,
    pub body: Option<String>,
}

impl Response {
    // pub fn build(&self) -> Result<String> {
    //     let code = &self.code;
    //     let body = &self.body;
    //     let response = format!(
    //         "HTTP/1.1 {code} OK\r\n\
    //          Content-Length: {}\r\n\
    //          Content-Type: text/plain\r\n\
    //          \r\n\
    //          {}",
    //         body.len(),
    //         body
    //     );
    //     Ok(response)
    // }

    fn content_type_header(&self) -> String {
        "Content-Type: text/plain\r\n".to_owned()
    }
    fn content_length_header(&self) -> Option<String> {
        let Some(body) = self.body.as_ref() else {
            return None;
        };
        let length = body.as_bytes().len();
        Some(format!("Content-Length: {length}\r\n"))
    }
}
