use bencode::{ByteString, Decodable};
use bencode_derive::Decodable;

/// Struct that stores compact information about individual trackers
#[derive(Decodable)]
pub struct CompactTrackers {
    /// Number of completed transfers (seeders)
    pub complete: usize,
    /// Interval for updates
    pub interval: u64,
    /// Number of incomplete transfers (leechers)
    pub incomplete: usize,
    /// Minimal interval to use for announcement
    #[bencode("min interval")]
    pub min_interval: usize,
    /// Peers to connect to in format 4 bytes for IP, 2 bytes for port
    pub peers: ByteString,
}
