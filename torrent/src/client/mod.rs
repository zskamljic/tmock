use crate::{trackers, Torrent};
use std::fs::File;
use std::io::prelude::*;

pub struct Client {
    pub(crate) peer_id: [u8; 20],
    pub(crate) port: u16,
}

impl Client {
    pub fn new(torrent: &Torrent, port: u16) -> Client {
        let mut client = Client {
            peer_id: random_bytes_id(),
            port: 6881,
        };
        let peers = trackers::request_trackers(&torrent, &client);

        client
    }
}

#[cfg(unix)]
fn random_bytes_id() -> [u8; 20] {
    let mut file = File::open("/dev/urandom").expect("urandom not found on the filesystem");

    let mut peer_id = [0u8; 20];
    file.read_exact(&mut peer_id)
        .expect("not enough data in urandom");
    peer_id
}
