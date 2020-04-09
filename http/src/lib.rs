mod body;
mod headers;
#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use std::net::TcpStream;
use std::str;

pub struct Response {
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

pub fn http_get(url: &str, parameters: &str, extra_headers: Option<&str>) -> Result<Vec<u8>> {
    let (host, path) = get_host_and_path(url)?;
    let extra_headers = match extra_headers {
        Some(value) => format!("\n{}", value),
        None => "".to_string(),
    };

    let mut stream = TcpStream::connect(host)?;
    stream.write_all(
        format!(
            "GET {}{} HTTP/1.1\nHost: {}{}\r\n\r\n",
            path, parameters, host, extra_headers
        )
        .as_bytes(),
    )?;

    let mut content = stream.bytes();
    let headers = headers::handle_http_headers(&mut content)?;
    let body = body::handle_http_body(&headers.headers, &mut content)?;

    Ok(body)
}

fn get_host_and_path(url: &str) -> Result<(&str, &str)> {
    if !url.starts_with("http://") {
        return Err(Error::new(
            ErrorKind::AddrNotAvailable,
            format!("Schema for {} is not supported", url),
        ));
    }
    let host = &url[7..];
    let path_start = match host.find('/') {
        Some(value) => value,
        None => host.len(),
    };

    let host = &host[..path_start];
    let path = &url[7 + path_start..];
    Ok((host, path))
}
