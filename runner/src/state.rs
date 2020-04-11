use mock::{Announcer, Client};
use std::collections::HashMap;
use torrent::{Decodable, Torrent};

/// Contains the state of announcers
pub struct State<'a> {
    /// The minimum speed to upload with
    min_speed: usize,
    /// The maximum speed to upload with
    max_speed: usize,
    /// Contains the names and hashes for currently loadd files
    hashes: HashMap<String, String>,
    /// Contains the announcer for given torrent files
    announcers: HashMap<String, Announcer<'a>>,
    /// The client that the announcers use
    client: &'a Client,
}

impl<'a> State<'a> {
    /// Creates a new state using the client and speeds.
    pub fn new(client: &'a Client, min_speed: usize, max_speed: usize) -> State<'a> {
        State {
            min_speed,
            max_speed,
            hashes: HashMap::new(),
            announcers: HashMap::new(),
            client,
        }
    }

    /// Create and add a new announcer for the given key and torrent.
    pub fn add_announcer(&mut self, key: String, torrent: Torrent) {
        let info_hash = torrent.get_info_hash();
        let announcer = Announcer::new(torrent, self.client);
        self.announcers.insert(info_hash.to_string(), announcer);
        self.hashes.insert(key, info_hash);
    }

    /// Load a new file and store the announcer.
    pub fn load_new(&mut self, file: String) {
        if let Ok(torrent) = Torrent::from_file(&file) {
            self.add_announcer(file, torrent);
        }
    }

    /// Remove the announcer for the file specified.
    pub fn remove(&mut self, file: String) {
        if let Some(hash) = self.hashes.remove(&file) {
            self.announcers.remove(&hash);
        }
    }

    /// Reloads the modified tile.
    pub fn modified(&mut self, file: String) {
        self.remove(file.to_string());
        self.load_new(file);
    }

    /// Change the name of the file.
    pub fn move_file(&mut self, from: String, to: String) {
        if let Some(value) = self.hashes.remove(&from) {
            self.hashes.insert(to, value);
        }
    }

    /// Announce all clients and remove all that failed 5 or more times.
    pub fn announce_all(&mut self) {
        for value in self.announcers.values_mut() {
            value.announce(self.min_speed, self.max_speed);
        }
        self.announcers.retain(|_, v| v.fail_count() < 5);
    }
}
