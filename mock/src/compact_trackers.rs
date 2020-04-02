use bencode::{ByteString, Decodable};
use bencode_derive::Decodable;

#[derive(Decodable)]
pub struct CompactTrackers {
    pub complete: usize,
    pub interval: u64,
    pub incomplete: usize,
    #[bencode("min interval")]
    pub min_interval: usize,
    pub peers: ByteString,
}
