mod client;
#[cfg(test)]
mod tests;
mod trackers;

use bencode::ByteString;
pub use bencode::{BencodeValue, Decodable, Encodable};
use bencode_derive::{Decodable, Encodable};
pub use client::Client;
use sha1;

/// Torrent struct that holds the url and `Info`
#[derive(Decodable)]
pub struct Torrent {
    /// The announce URL on which to report the status
    pub announce: String,
    /// The info containing the files
    pub info: Info,
}

impl Torrent {
    /// Calculates the info hash from the info field.
    pub fn get_info_hash(&self) -> String {
        // value must be present if Torrent is valid
        let bencode = self.info.encode().unwrap();
        let hash = sha1::sha1_bytes_as_bytes(&bencode);
        trackers::url_encode(&hash)
    }
}

/// Torrent info, containing files, name, etc.
#[derive(Decodable, Encodable)]
pub struct Info {
    /// The name of the torrent
    pub name: String,
    /// Piece length of contained data
    #[bencode("piece length")]
    pub piece_length: usize,
    /// Hashes of pieces
    pub pieces: ByteString,
    /// Length of file, if there is only one
    pub length: Option<usize>,
    /// A list of files present, omitted if there is only one file
    pub files: Option<Vec<File>>,
    /// Whether or not the file is on a private tracker
    pub private: Option<u16>,
    /// Private tracker source
    pub source: Option<String>,
}

/// Represents a file that can be transfered with the torrent
#[derive(Decodable, Encodable)]
pub struct File {
    /// The length of the file
    length: usize,
    /// The path components to the file
    path: Vec<String>,
}
