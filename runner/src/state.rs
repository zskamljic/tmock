use mock::{Announcer, Client};
use std::collections::HashMap;
use torrent::{Decodable, Torrent};

pub struct State<'a> {
    min_speed: usize,
    max_speed: usize,
    hashes: HashMap<String, String>,
    announcers: HashMap<String, Announcer<'a>>,
    client: &'a Client,
}

impl<'a> State<'a> {
    pub fn new(client: &'a Client, min_speed: usize, max_speed: usize) -> State<'a> {
        State {
            min_speed,
            max_speed,
            hashes: HashMap::new(),
            announcers: HashMap::new(),
            client,
        }
    }

    pub fn add_announcer(&mut self, key: String, torrent: Torrent) {
        let info_hash = torrent.get_info_hash();
        let announcer = Announcer::new(torrent, self.client);
        self.announcers.insert(info_hash.to_string(), announcer);
        self.hashes.insert(key, info_hash);
    }

    pub fn load_new(&mut self, file: String) {
        if let Ok(torrent) = Torrent::from_file(&file) {
            self.add_announcer(file, torrent);
        }
    }

    pub fn remove(&mut self, file: String) {
        if let Some(hash) = self.hashes.remove(&file) {
            self.announcers.remove(&hash);
        }
    }

    pub fn modified(&mut self, file: String) {
        self.remove(file.to_string());
        self.load_new(file);
    }

    pub fn move_file(&mut self, from: String, to: String) {
        if let Some(value) = self.hashes.remove(&from) {
            self.hashes.insert(to, value);
        }
    }

    pub fn announce_all(&mut self) {
        for value in self.announcers.values_mut() {
            value.announce(self.min_speed, self.max_speed);
        }
    }
}
