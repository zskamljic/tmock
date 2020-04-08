#[cfg(test)]
mod tests;

use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use std::net::TcpStream;
use std::str;

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

    let mut buffer = vec![];
    loop {
        match stream.read_to_end(&mut buffer) {
            Ok(_) => break,
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(e) => return Err(e),
        }
    }

    Ok(get_content(&buffer))
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

fn get_content(buffer: &[u8]) -> Vec<u8> {
    let mut newline_prefix = false;
    for i in 1..buffer.len() {
        if buffer[i] == 10 && buffer[i - 1] == 13 {
            if newline_prefix {
                return buffer[i + 1..].to_vec();
            }
            newline_prefix = true;
        } else if buffer[i] != 13 {
            newline_prefix = false;
        }
    }

    println!("No content found!");
    Vec::new()
}
