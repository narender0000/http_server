mod method;
mod request;
mod response;
use anyhow::{Context, Result};
use std::net::TcpListener;

use request::process_request;
use response::HttpCode;

use crate::response::send_response;
pub fn run() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream?;
        let request = process_request(&mut stream).context("getting request")?;
        let response_code = if request.path == "/" {
            HttpCode::Ok
        } else {
            HttpCode::NotFound
        };
        send_response(response_code, &mut stream).context("sending response")?;
    }
    Ok(())
}
