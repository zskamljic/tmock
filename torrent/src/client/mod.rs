use crate::trackers::TrackerInfo;
use crate::{trackers, Torrent};
use rand;
use std::io::Result;

pub struct Client {
    pub peer_id: [u8; 20],
    pub port: u16,
    pub tracker_info: TrackerInfo,
}

impl Client {
    pub fn new(torrent: &Torrent, port: u16) -> Result<Client> {
        let peer_id = random_bytes_id();
        let tracker_info = trackers::request_trackers(&torrent, &peer_id, port)?;

        let client = Client {
            peer_id,
            port,
            tracker_info,
        };

        Ok(client)
    }
}

#[cfg(unix)]
fn random_bytes_id() -> [u8; 20] {
    let mut peer_id = [0u8; 20];

    rand::bytes(&mut peer_id);
    peer_id
}
