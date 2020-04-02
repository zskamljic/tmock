use crate::announcer::Announcer;
use crate::id_generator;
use crate::key_generator;
use crate::tracker_updates::TrackerUpdates;
use http;
use std::net::TcpListener;

const TRANSMISSION_HEADERS: &str = "User-Agent: Transmission/2.94
Accept: */*
Accept-Encoding: gzip;q=1.0, deflate, identity";

#[derive(Debug, PartialEq)]
enum Event {
    STARTED,
    STOPPED,
    EMPTY,
}

#[derive(Default)]
pub struct Client {
    key: String,
    peer_id: String,
    port: u16,
}

impl Client {
    pub fn new() -> Client {
        Client {
            key: key_generator::generate_i32_hex_key(),
            peer_id: id_generator::generate_transmission_294_id(),
            port: find_available_port(),
        }
    }

    pub fn send_start(&self, announcer: &Announcer) -> Option<TrackerUpdates> {
        self.send_event(
            &announcer.announce_url,
            Event::STARTED,
            &announcer.info_hash,
            0,
        )
    }

    pub fn send_update(&self, announcer: &Announcer) -> Option<TrackerUpdates> {
        self.send_event(
            &announcer.announce_url,
            Event::EMPTY,
            &announcer.info_hash,
            announcer.uploaded,
        )
    }

    pub fn send_stop(&self, announcer: &Announcer) -> Option<TrackerUpdates> {
        self.send_event(
            &announcer.announce_url,
            Event::STOPPED,
            &announcer.info_hash,
            announcer.uploaded,
        )
    }

    fn send_event(
        &self,
        url: &str,
        event: Event,
        info_hash: &str,
        uploaded: usize,
    ) -> Option<TrackerUpdates> {
        let peer_count = if event == Event::STOPPED { 0 } else { 80 };
        let mut parameters = self.create_parameters(info_hash, uploaded, peer_count);
        match event {
            Event::STARTED => parameters.push_str("&event=started"),
            Event::STOPPED => parameters.push_str("&event=stopped"),
            Event::EMPTY => (),
        }

        let result = http::http_get(url, &parameters, Some(TRANSMISSION_HEADERS));
        match result {
            Ok(value) => TrackerUpdates::decode(&value),
            Err(error) => {
                eprintln!("Error sending event {:?}: {}", event, error);
                None
            }
        }
    }

    fn create_parameters(&self, info_hash: &str, uploaded: usize, peer_num: u32) -> String {
        format!("?info_hash={}&peer_id={}&port={}&uploaded={}&downloaded=0&left=0&numwant={}&key={}&compact=1&supportcrypto=1",
    info_hash, self.peer_id, self.port, uploaded,peer_num, self.key)
    }
}

fn find_available_port() -> u16 {
    for port in 40_000..=50_000 {
        match TcpListener::bind(format!("127.0.0.1:{}", port)) {
            Err(_) => continue,
            Ok(_) => return port,
        }
    }
    panic!("Port range 40,000-50,000 is taken.");
}
