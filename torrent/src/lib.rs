mod client;
#[cfg(test)]
mod tests;
mod trackers;

use bencode::ByteString;
pub use bencode::{BencodeValue, Decodable, Encodable};
use bencode_derive::{Decodable, Encodable};
pub use client::Client;

#[derive(Decodable)]
pub struct Torrent {
    announce: String,
    info: Info,
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
