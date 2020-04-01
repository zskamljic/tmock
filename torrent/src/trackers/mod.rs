#[cfg(test)]
mod tests;

use crate::{Info, Torrent};
use bencode::{Decodable, Encodable};
use bencode_derive::Decodable;
use http;
use std::io::{Error, ErrorKind, Result};

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
    let parameters = create_parameters(peer_id, port, &torrent.info);
    let result = http::http_get(&torrent.announce, &parameters, None)?;

    process_response(&result)
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

pub fn url_encode(data: &[u8]) -> String {
    data.iter()
        .map(|value| {
            if (b'a'..=b'z').contains(value)
                || (b'A'..=b'Z').contains(value)
                || (b'0'..=b'9').contains(value)
                || &b'.' == value
                || &b'-' == value
                || &b'_' == value
                || &b'~' == value
            {
                format!("{}", *value as char)
            } else {
                format!("%{:02X}", value)
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

fn process_response(response: &[u8]) -> Result<TrackerInfo> {
    let response = GetTrackers::read_bytes(response)?;

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
