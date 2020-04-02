use crate::tracker_updates::TrackerUpdates;
use crate::Client;
use std::time::{SystemTime, UNIX_EPOCH};
use torrent::Torrent;

pub struct Announcer<'a> {
    client: &'a Client,
    pub(crate) announce_url: String,
    pub(crate) info_hash: String,
    pub(crate) uploaded: usize,
    name: String,
    tracker_info: Option<TrackerUpdates>,
    last_announce: u64,
}

impl<'a> Announcer<'a> {
    pub fn new(torrent: Torrent, client: &'a Client) -> Announcer<'a> {
        let hash = torrent.get_info_hash();
        Announcer {
            client,
            announce_url: torrent.announce,
            info_hash: hash,
            uploaded: 0,
            name: torrent.info.name,
            tracker_info: None,
            last_announce: 0,
        }
    }

    pub fn announce(&mut self) {
        if let Some(info) = &self.tracker_info {
            let now = current_time_seconds();
            if now - self.last_announce < info.interval {
                return;
            }
        }

        if let Some(value) = self.select_announce_function() {
            self.update_trackers(value);
        } else {
            println!("Failed to announce {}", self.name);
        }
    }

    fn select_announce_function(&mut self) -> Option<TrackerUpdates> {
        if self.tracker_info.is_none() {
            self.client.send_start(self)
        } else {
            self.client.send_update(self)
        }
    }

    fn update_trackers(&mut self, info: TrackerUpdates) {
        println!(
            "Announced {} with {} seeders, {} leechers, {}s interval",
            self.name, info.seeders, info.leechers, info.interval
        );
        self.last_announce = current_time_seconds();
        self.tracker_info = Some(info);
    }
}

impl Drop for Announcer<'_> {
    fn drop(&mut self) {
        self.client.send_stop(self);
    }
}

fn current_time_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time changed")
        .as_secs()
}
