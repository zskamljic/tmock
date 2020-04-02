use crate::compact_trackers::CompactTrackers;
use bencode::Decodable;

pub struct TrackerUpdates {
    pub interval: u64,
    pub seeders: usize,
    pub leechers: usize,
}

impl TrackerUpdates {
    pub fn decode(data: &[u8]) -> Option<TrackerUpdates> {
        let trackers = match CompactTrackers::read_bytes(data) {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Error decoding response: {}", error);
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
