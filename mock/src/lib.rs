mod id_generator;
mod key_generator;

#[derive(Default)]
pub struct Client {
    key: String,
    peer_id: String,
}

impl Client {
    pub fn new() -> Client {
        Client {
            key: key_generator::generate_i32_hex_key(),
            peer_id: id_generator::generate_transmission_294_id(),
        }
        // TODO: send started event
    }

    // TODO: implement report

    // TODO: implement upload checks
}

impl Drop for Client {
    fn drop(&mut self) {
        todo!("Call announce stop")
    }
}
