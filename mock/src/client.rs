use crate::announcer::Announcer;
use crate::id_generator;
use crate::key_generator;
use crate::tracker_updates::TrackerUpdates;
use http;
use std::net::TcpListener;

const TRANSMISSION_HEADERS: &str = "User-Agent: Transmission/2.94
Accept: */*
Accept-Encoding: gzip;q=1.0, deflate, identity";

/// Event to announce
#[derive(Debug, PartialEq)]
enum Event {
    /// The client has started seeding
    STARTED,
    /// The client stopped seeding
    STOPPED,
    /// The client is sending an update
    EMPTY,
}

/// Client is used to send the updates or announcements
/// to remote, it handles peer ID generation and port
/// selection.
#[derive(Default)]
pub struct Client {
    /// Unique key for this client, in case IP changes
    key: String,
    /// Unique peer ID for this client
    peer_id: String,
    /// The port we supposedly use for P2P
    port: u16,
}

impl Client {
    /// Creates new client.
    pub fn new() -> Client {
        Client {
            key: key_generator::generate_i32_hex_key(),
            peer_id: id_generator::generate_transmission_294_id(),
            port: find_available_port(),
        }
    }

    /// Sends a start event
    pub fn send_start(&self, announcer: &Announcer) -> Option<TrackerUpdates> {
        self.send_event(
            &announcer.announce_url,
            Event::STARTED,
            &announcer.info_hash,
            0,
        )
    }

    /// Sends an update event
    pub fn send_update(&self, announcer: &Announcer) -> Option<TrackerUpdates> {
        self.send_event(
            &announcer.announce_url,
            Event::EMPTY,
            &announcer.info_hash,
            announcer.uploaded,
        )
    }

    /// Sends a stop event
    pub fn send_stop(&self, announcer: &Announcer) -> Option<TrackerUpdates> {
        self.send_event(
            &announcer.announce_url,
            Event::STOPPED,
            &announcer.info_hash,
            announcer.uploaded,
        )
    }

    /// Sends an event as per the parameters
    ///
    /// Automatically selects the extra query parameters, required peers and decodes
    /// the response as bencode value.
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
            Err(_) => None,
        }
    }

    /// Creates a parameter string the same was Transmission 2.94 does.
    fn create_parameters(&self, info_hash: &str, uploaded: usize, peer_num: u32) -> String {
        format!("?info_hash={}&peer_id={}&port={}&uploaded={}&downloaded=0&left=0&numwant={}&key={}&compact=1&supportcrypto=1",
                info_hash, self.peer_id, self.port, uploaded,peer_num, self.key)
    }
}

/// Tries to find the first valid port.
fn find_available_port() -> u16 {
    for port in 40_000..=50_000 {
        match TcpListener::bind(format!("127.0.0.1:{}", port)) {
            Err(_) => continue,
            Ok(_) => return port,
        }
    }
    panic!("Port range 40,000-50,000 is taken.");
}
