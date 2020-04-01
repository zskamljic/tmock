#[cfg(test)]
mod tests;

use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use std::net::TcpStream;
use std::str;

pub fn http_get(url: &str, parameters: &str, extra_headers: Option<&str>) -> Result<String> {
    let (host, path) = get_host_and_path(url)?;
    let extra_headers = match extra_headers {
        Some(value) => format!("\n{}", value),
        None => "".to_string(),
    };

    let mut stream = TcpStream::connect(host)?;
    stream.write_all(
        format!(
            "GET {}{} HTTP/1.1\n{}{}\r\n\r\n",
            path, parameters, host, extra_headers
        )
        .as_bytes(),
    )?;

    let mut buffer = vec![];
    loop {
        match stream.read_to_end(&mut buffer) {
            Ok(_) => break,
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(e) => return Err(e),
        }
    }

    println!("Response bytes: {:?}", buffer);
    match str::from_utf8(&buffer) {
        Ok(value) => Ok(get_http_content(value)),
        Err(error) => Err(Error::new(ErrorKind::InvalidData, error)),
    }
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

fn get_http_content(response: &str) -> String {
    let mut body_string = String::new();
    let mut body_started = false;
    for line in response.lines() {
        if body_started {
            body_string.push_str(line);
            continue;
        }
        if line.is_empty() {
            body_started = true;
            continue;
        }
    }

    body_string
}
