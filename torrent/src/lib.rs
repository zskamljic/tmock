mod client;
#[cfg(test)]
mod tests;
mod trackers;

use bencode::ByteString;
pub use bencode::{BencodeValue, Decodable, Encodable};
use bencode_derive::{Decodable, Encodable};
pub use client::Client;
use sha1;

#[derive(Decodable)]
pub struct Torrent {
    pub announce: String,
    pub info: Info,
}

impl Torrent {
    pub fn get_info_hash(&self) -> String {
        // value must be present if Torrent is valid
        let bencode = self.info.encode().unwrap();
        let hash = sha1::sha1_bytes_as_bytes(&bencode);
        trackers::url_encode(&hash)
    }
}

#[derive(Decodable, Encodable)]
pub struct Info {
    name: String,
    #[bencode("piece length")]
    piece_length: usize,
    pieces: ByteString,
    length: Option<usize>,
    files: Option<Vec<File>>,
}

#[derive(Decodable, Encodable)]
pub struct File {
    length: usize,
    path: Vec<String>,
}
