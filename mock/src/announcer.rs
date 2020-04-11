use crate::tracker_updates::TrackerUpdates;
use crate::Client;
use rand;
use std::time::{SystemTime, UNIX_EPOCH};
use torrent::Torrent;

/// `Announcer` handles announcing of individual
/// torrent.
pub struct Announcer<'a> {
    /// The client to use in all requests
    client: &'a Client,
    /// Announce url to which the torrent is announced
    pub(crate) announce_url: String,
    /// Hash of Info field in torrent file to identify the torrent
    pub(crate) info_hash: String,
    /// Number of bytes uploaded so far
    pub(crate) uploaded: usize,
    /// The name of the torrent
    name: String,
    /// Last received information about torrent, `None` if no request succeded yet.
    tracker_info: Option<TrackerUpdates>,
    /// Time of last announce
    last_announce: u64,
    /// Number of consecutive failed attemtps
    failed: u8,
}

impl<'a> Announcer<'a> {
    /// Creates a new `Announcer` for given `Torrent` and `Client`
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

    /// Attempt to trigger an announce.
    ///
    /// Announce will only be executed if the interval is appropriate
    /// or no interval info is present yet (not yet announced).
    ///
    /// If upload is to be announced a random upload sum is generated
    /// using the provided arguments.
    ///
    /// # Arguments
    ///
    /// * `min_speed` - the minimum speed used for seeding
    /// * `max_speed` - the maximum speed used for seeding
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

    /// Selects the appropriate announce function based on internal state
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

    /// Update the trackers  in this struct and logs information
    /// that was received, also sets the last_announce value to current time.
    fn update_trackers(&mut self, info: TrackerUpdates) {
        println!(
            "Announced {} with {} seeders, {} leechers, {}s interval",
            self.name, info.seeders, info.leechers, info.interval
        );
        self.last_announce = current_time_seconds();
        self.tracker_info = Some(info);
    }

    /// Calculates the upload since last announce.
    ///
    /// Uses the provided values to generate a random amount.
    /// The value is randomly chosen for each second that the torrent
    /// has been seeding.
    ///
    /// # Arguments
    ///
    /// * `min_speed` - min speed to generate
    /// * `max_speed` - max speed to generate
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

    /// Returns number of consecutive failed attempts
    /// to announce.
    pub fn fail_count(&self) -> u8 {
        self.failed
    }
}

/// Drop implementation to send stop announcement
/// when announcer is being dropped.
impl Drop for Announcer<'_> {
    fn drop(&mut self) {
        self.client.send_stop(self);
    }
}

/// Returns the current time in seconds passed since unix epoch.
fn current_time_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time changed")
        .as_secs()
}
