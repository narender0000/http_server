use anyhow::{Context, Result, bail};
use bytes::{Buf, buf::Reader};
use std::{
    fmt::Display,
    io::{BufRead, Read, Write},
    net::{TcpListener, TcpStream},
};
pub fn run() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream?;
        let raw_request = read_stream(&mut stream).context("reading stream")?;
        let request = parse_raw_request(raw_request)?;

        let response_code = if request.path == "/" {
            HttpCode::Ok
        } else {
            HttpCode::NotFound
        };

        let response = format!("HTTP/1.1 {response_code} OK\r\n\r\n");
        stream
            .write_all(response.as_bytes())
            .context("writing all the response data")?;
        stream
            .flush()
            .context("flushing write so that everything goes out")?;
    }
    Ok(())
}

fn read_stream(stream: &mut TcpStream) -> Result<Vec<u8>> {
    let mut request = vec![];
    loop {
        const BUFFER_SIZE: usize = 9;
        let mut chunk = [0_u8; BUFFER_SIZE];
        let how_many_read = stream.read(&mut chunk).context("Reading request chunk")?;
        request.extend_from_slice(&chunk[..how_many_read]);

        if how_many_read < BUFFER_SIZE {
            break;
        };
    }
    Ok(request)
}

fn parse_raw_request(request: Vec<u8>) -> Result<Request> {
    let mut reader = request.reader();
    let method = parse_method_from_request(&mut reader).context("parsing method")?;

    let path = parser_path_from_request(&mut reader).context("parsing path")?;

    Ok(Request { method, path })
}

fn parse_method_from_request(request: &mut Reader<&[u8]>) -> Result<Method> {
    const SPACE: u8 = b' ';
    let mut method = vec![];
    request
        .read_until(SPACE, &mut method)
        .context("getting method bytes")?;
    Method::try_from(method)
}

fn parser_path_from_request(request: &mut Reader<&[u8]>) -> Result<String> {
    const SPACE: u8 = b' ';
    let mut path_bytes = vec![];
    request
        .read_until(SPACE, &mut path_bytes)
        .context("parsing  path from request")?;
    Ok(String::from_utf8(path_bytes)
        .context("converting path bytes to string")?
        .trim()
        .to_string())
}

#[derive(Debug)]
struct Request {
    method: Method,
    path: String,
}

#[derive(Debug)]
enum Method {
    Get,
}

impl TryFrom<Vec<u8>> for Method {
    type Error = anyhow::Error;
    fn try_from(value: Vec<u8>) -> std::prelude::v1::Result<Self, Self::Error> {
        let method_string =
            String::from_utf8(value).context("converting bytes to method string")?;
        Ok(match method_string.to_uppercase().trim() {
            "GET" => Self::Get,
            _ => bail!("Unkown Method"),
        })
    }
}
enum HttpCode {
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
