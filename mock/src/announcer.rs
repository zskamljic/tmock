use crate::tracker_updates::TrackerUpdates;
use crate::Client;
use rand;
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
    failed: u8,
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
            failed: 0,
        }
    }

    pub fn announce(&mut self, min_speed: usize, max_speed: usize) {
        if let Some(info) = &self.tracker_info {
            let now = current_time_seconds();
            if now - self.last_announce < info.interval {
                return;
            }
        }

        if let Some(value) = self.select_announce_function(min_speed, max_speed) {
            self.failed = 0;
            self.update_trackers(value);
        } else {
            println!("Failed to announce {}", self.name);
            self.failed += 1;
        }
    }

    fn select_announce_function(
        &mut self,
        min_speed: usize,
        max_speed: usize,
    ) -> Option<TrackerUpdates> {
        if self.tracker_info.is_none() {
            self.client.send_start(self)
        } else {
            self.calculate_upload(min_speed, max_speed);
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

    fn calculate_upload(&mut self, min_speed: usize, max_speed: usize) {
        if let Some(value) = &self.tracker_info {
            if value.leechers == 0 {
                return;
            }

            let now = current_time_seconds();
            let delay = now - self.last_announce;
            self.uploaded += (0..=delay)
                .map(|_| rand::random_usize(min_speed, max_speed))
                .sum::<usize>();
        }
    }

    pub fn fail_count(&self) -> u8 {
        self.failed
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
