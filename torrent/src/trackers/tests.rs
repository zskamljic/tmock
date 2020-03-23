use super::*;
use bencode::Decodable;
use std::io::Result;

#[test]
fn get_host_and_path_splits_correct() -> Result<()> {
    let (host, path) = get_host_and_path("http://tracker.archlinux.org:6969/announce")?;

    assert_eq!("tracker.archlinux.org:6969", host);
    assert_eq!("/announce", path);
    Ok(())
}

#[test]
fn fetch_trackers() -> Result<()> {
    request_trackers(
        &Torrent::from_file("../torrents/archlinux-2020.02.01-x86_64.iso.torrent")?,
        &Client {
            peer_id: [
                0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 9u8, b'A', b'B', b'C', b'D', b'E', b'F',
                b'G', b'H', b'I', b'J', b'K',
            ],
            port: 6881,
        },
    )?;

    Ok(())
}

#[test]
fn create_parameters_creates_query_url() {
    let info = Info {
        name: "file".to_string(),
        piece_length: 20,
        pieces: "asdf".to_string(),
        length: Some(5),
        files: None,
    };

    let parameter_str = create_parameters(
        &Client {
            peer_id: [
                0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 9u8, b'A', b'B', b'C', b'D', b'E', b'F',
                b'G', b'H', b'I', b'J', b'K',
            ],
            port: 6881,
        },
        &info,
    );

    assert_eq!(
        format!(
            "{}{}{}{}{}{}",
            "?info_hash=%A7%5E%11%C1%8E%17%12%07%F5%E0%21%84z%3EwFH%7D%B6%E0",
            "&peer_id=%00%01%02%03%04%05%06%07%09ABCDEFGHIJK",
            "&port=6881",
            "&uploaded=0",
            "&downloaded=0",
            "&left=5"
        ),
        parameter_str
    );
}

#[test]
fn url_encode_encodes_correct() {
    let input = [b'a', b'A', 0, b'('];
    let output = url_encode(&input);

    assert_eq!("aA%00%28", output);
}
