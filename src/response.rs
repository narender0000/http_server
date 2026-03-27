use std::{io::Write, net::TcpStream};

use anyhow::{Context, Ok, Result};
use std::fmt::Display;
pub fn send_response(response_code: HttpCode, stream: &mut TcpStream) -> Result<()> {
    let response = format!("HTTP/1.1 {response_code} OK\r\n\r\n");
    stream
        .write_all(response.as_bytes())
        .context("writing all the response data")?;
    stream
        .flush()
        .context("flushing write so that everything goes out")?;
    Ok(())
}

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
