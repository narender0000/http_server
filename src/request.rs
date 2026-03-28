use super::method::Method;
use anyhow::Context;
use anyhow::Result;
use bytes::{Buf, buf::Reader};
use std::collections::HashMap;
use std::{
    io::{BufRead, Read},
    net::TcpStream,
};

pub fn process_request(stream: &mut TcpStream) -> Result<Request> {
    let raw_request = read_stream(stream).context("reading stream")?;
    let request = parse_raw_request(raw_request).context("parsing raw request")?;
    Ok(request)
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
    let protocol =
        parse_protocol_from_request(&mut reader).context("parsing protocol from request")?;
    println!("protocol: {protocol}");

    let headers = parse_headers_from_request(&mut reader).context("parsing headers")?;
    println!("headers: {headers:?}");

    Ok(Request {
        method,
        path,
        protocol,
        headers,
    })
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
        .to_string()
        .to_owned())
}
fn parse_protocol_from_request(request: &mut Reader<&[u8]>) -> Result<String> {
    let mut protocol_bytes = String::new();
    request
        .read_line(&mut protocol_bytes)
        .context("reading protocol bytes")?;
    Ok(protocol_bytes.to_owned())
}

pub type Headers = HashMap<String, String>;
fn parse_headers_from_request(request: &mut Reader<&[u8]>) -> Result<Headers> {
    let mut headers = HashMap::new();
    loop {
        let mut raw_header = String::new();
        request
            .read_line(&mut raw_header)
            .context("readaing header bytes")?;
        let header = raw_header.trim();
        if header.is_empty() {
            break;
        }
        let mut header_parts = raw_header.splitn(2, ":");
        let header_name = header_parts
            .next()
            .map(|header| header.trim().to_lowercase());
        let header_value = header_parts.next().map(|header| header.trim().to_owned());

        if header_name.is_none() || header_value.is_none() {
            continue;
        }
        headers.insert(header_name.unwrap(), header_value.unwrap());
    }
    Ok(headers)
}

#[derive(Debug)]
pub struct Request {
    #[allow(dead_code)]
    pub method: Method,
    pub path: String,
    pub protocol: String,
    pub headers: Headers,
}
