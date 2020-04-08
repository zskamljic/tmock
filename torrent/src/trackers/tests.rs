use super::*;
use bencode::ByteString;
use bencode::Decodable;
use std::fs;
use std::io::Result;

#[test]
fn fetch_trackers() -> Result<()> {
    request_trackers(
        &Torrent::from_file("../torrents/archlinux-2020.02.01-x86_64.iso.torrent")?,
        &[
            0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 9u8, b'A', b'B', b'C', b'D', b'E', b'F', b'G',
            b'H', b'I', b'J', b'K',
        ],
        6881,
    )?;

    Ok(())
}

#[test]
fn create_parameters_creates_query_url() {
    let info = Info {
        name: "file".to_string(),
        piece_length: 20,
        pieces: ByteString::new(vec![b'a', b'b', b'c', b'd']),
        length: Some(5),
        files: None,
        private: None,
        source: None,
    };

    let parameter_str = create_parameters(
        &[
            0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 9u8, b'A', b'B', b'C', b'D', b'E', b'F', b'G',
            b'H', b'I', b'J', b'K',
        ],
        6881,
        &info,
    );

    assert_eq!(
        format!(
            "{}{}{}{}{}{}",
            "?downloaded=0",
            "&info_hash=%7B%BB%B1%F8%A7%A9R%E3%B0I%F6k%F7%98%7D%2C%DB%8AL%85",
            "&left=5",
            "&peer_id=%00%01%02%03%04%05%06%07%09ABCDEFGHIJK",
            "&port=6881",
            "&uploaded=0"
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

#[test]
fn process_response_succeeds() -> Result<()> {
    let request = fs::read("trackers_response.txt")?;
    let response = process_response(&request)?;

    assert_eq!(900, response.interval);
    assert_eq!(50, response.peers.len());
    Ok(())
}
