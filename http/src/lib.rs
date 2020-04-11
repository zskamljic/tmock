//! # http
//!
//! Basic http client implementation to allow retrieval of
//! headers and body as `Vec<u8>`. Only supports basic and
//! chunked encoding, non-repeating header values and plain
//! http.
mod body;
mod headers;
#[cfg(test)]
mod tests;

use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use std::net::TcpStream;
use std::str;

/// Basic http get request.
///
/// Creates a GET request to the given URL, returning `Vec<u8>` containing the
/// response body.
///
/// # Arguments
///
/// * `url` - the url to request, e.g. 'http://google.com/'
/// * `parameters` - the query parameter string
/// * `extra_headers` - the extra headers to provide, None if no headers are required.
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

/// Splits the URL into host and path parts.
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
