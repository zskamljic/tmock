#[cfg(test)]
mod tests;

use crate::client::Client;
use crate::{Info, Torrent};
use bencode::Encodable;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use std::net::TcpStream;
use std::str;

pub fn request_trackers(torrent: &Torrent, client: &Client) -> Result<()> {
    let (host, path) = get_host_and_path(&torrent.announce)?;
    let parameters = create_parameters(&client, &torrent.info);
    let mut stream = TcpStream::connect(host)?;

    stream.write_all(format!("GET {}{}\r\n\r\n", path, parameters).as_bytes())?;

    let mut buffer = [0u8; 512];
    let read = stream.read(&mut buffer)?;

    println!(
        "Read: {}: {}",
        read,
        str::from_utf8(&buffer).expect("wasn't a valid string")
    );

    Ok(())
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

fn create_parameters(client: &Client, info: &Info) -> String {
    let mut result = String::new();
    result.push_str("?info_hash=");

    let encoded = info.encode().unwrap();

    let hashed = sha1::sha1_bytes_as_bytes(&encoded);
    result.push_str(&url_encode(&hashed));

    result.push_str("&peer_id=");
    result.push_str(&url_encode(&client.peer_id));
    result.push_str("&port=");
    result.push_str(&format!("{}", client.port));
    result.push_str("&uploaded=0");
    result.push_str("&downloaded=0");
    result.push_str(&format!("&left={}", info.length.unwrap()));
    result
}

fn url_encode(data: &[u8]) -> String {
    data.iter()
        .map(|value| {
            if (b'a'..=b'z').contains(value)
                || (b'A'..=b'Z').contains(value)
                || (b'0'..=b'9').contains(value)
            {
                format!("{}", *value as char)
            } else {
                format!("%{:02X}", value)
            }
        })
        .collect::<Vec<String>>()
        .join("")
}
