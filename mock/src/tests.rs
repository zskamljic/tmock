use super::*;
use std::io::Result;
use torrent::{Decodable, Torrent};

#[test]
fn report_start_update_end() -> Result<()> {
    let client = Client::new();
    let torrent = Torrent::from_file("../torrents/archlinux-2020.02.01-x86_64.iso.torrent")?;

    let mut announcer = Announcer::new(torrent, &client);
    announcer.announce(0, 0);

    Ok(())
}
