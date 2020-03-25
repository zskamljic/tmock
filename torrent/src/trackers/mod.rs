#[cfg(test)]
mod tests;

use crate::{Info, Torrent};
use bencode::{Decodable, Encodable};
use bencode_derive::Decodable;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use std::net::TcpStream;
use std::str;

#[derive(Decodable)]
struct GetTrackers {
    #[bencode("failure reason")]
    failure_reason: Option<String>,
    interval: Option<u32>,
    peers: Option<Vec<Peer>>,
}

#[derive(Decodable)]
pub struct Peer {
    pub ip: String,
    pub port: u16,
}

pub struct TrackerInfo {
    pub interval: u32,
    pub peers: Vec<Peer>,
}

pub fn request_trackers(torrent: &Torrent, peer_id: &[u8; 20], port: u16) -> Result<TrackerInfo> {
    let (host, path) = get_host_and_path(&torrent.announce)?;
    let parameters = create_parameters(peer_id, port, &torrent.info);
    let mut stream = TcpStream::connect(host)?;

    stream
        .write_all(format!("GET {}{} HTTP/1.1\n{}\r\n\r\n", path, parameters, host).as_bytes())?;

    let mut buffer = vec![];
    loop {
        match stream.read_to_end(&mut buffer) {
            Ok(_) => break,
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(e) => return Err(e),
        }
    }

    match str::from_utf8(&buffer) {
        Ok(value) => process_response(value),
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

fn create_parameters(peer_id: &[u8; 20], port: u16, info: &Info) -> String {
    let mut result = String::new();
    result.push_str("?downloaded=0");
    result.push_str("&info_hash=");

    // Was decoded from bencode and never modified
    // so it MUST work, otherwise bencode implementation is wrong
    let encoded = info.encode().unwrap();

    let hashed = sha1::sha1_bytes_as_bytes(&encoded);
    result.push_str(&url_encode(&hashed));

    result.push_str(&format!("&left={}", info.length.unwrap()));
    result.push_str("&peer_id=");
    result.push_str(&url_encode(peer_id));
    result.push_str("&port=");
    result.push_str(&format!("{}", port));
    result.push_str("&uploaded=0");
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

fn process_response(response: &str) -> Result<TrackerInfo> {
    let mut content = match response.lines().last() {
        Some(value) => value.as_bytes(),
        None => return Err(Error::new(ErrorKind::InvalidData, "no lines")),
    };

    let response = GetTrackers::read(&mut content)?;
    if let Some(failure) = response.failure_reason {
        return Err(Error::new(ErrorKind::Other, failure));
    }

    let interval = match response.interval {
        Some(value) => value,
        None => return Err(Error::new(ErrorKind::InvalidData, "interval missing")),
    };

    let peers = match response.peers {
        Some(value) => value,
        None => return Err(Error::new(ErrorKind::InvalidData, "peers missing")),
    };

    Ok(TrackerInfo { interval, peers })
}
