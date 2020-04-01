use super::*;
use std::io::Result;
use torrent::Decodable;

#[test]
fn report_start_update_end() -> Result<()> {
    let torrent = Torrent::from_file("../torrents/archlinux-2020.02.01-x86_64.iso.torrent")?;

    let announcer = Announcer::new(torrent);
    let client = Client::new();

    client.send_start(&announcer);

    Ok(())
}
