use crate::compact_trackers::CompactTrackers;
use bencode::Decodable;

/// Stores data about the tracker
pub struct TrackerUpdates {
    /// Interval to use to report updates
    pub interval: u64,
    /// Number of seeders
    pub seeders: usize,
    /// Number of leechers
    pub leechers: usize,
}

impl TrackerUpdates {
    /// Used to decode `TrackerUpdates`.
    ///
    /// Underlying implementation uses `CompactTrackers`, with
    /// this class abstracting the underlying tracker response.
    pub fn decode(data: &[u8]) -> Option<TrackerUpdates> {
        let trackers = match CompactTrackers::read_bytes(data) {
            Ok(value) => value,
            Err(_) => {
                return None;
            }
        };

        Some(TrackerUpdates {
            interval: trackers.interval,
            seeders: trackers.complete,
            leechers: trackers.incomplete,
        })
    }
}
