use super::*;
use std::io::Result;

#[test]
fn decode_file_succeds() -> Result<()> {
    let mut input = "d6:lengthi5e4:pathl1:a1:bee".as_bytes();
    let file = File::read(&mut input)?;

    assert_eq!(5, file.length);
    assert_eq!(2, file.path.len());
    assert_eq!(vec!["a".to_string(), "b".to_string()], file.path);

    Ok(())
}

#[test]
fn decode_fn_succeeds_string() -> Result<()> {
    let value: String = String::decode(&BencodeValue::String("asdf".to_string()))?;

    assert_eq!("asdf", value);
    Ok(())
}

#[test]
fn decode_info_succeeds() -> Result<()> {
    let mut input = "d4:name4:name12:piece lengthi1e6:pieces1:56:lengthi11ee".as_bytes();
    let info = Info::read(&mut input)?;

    assert_eq!("name", info.name);
    assert_eq!(1, info.piece_length);
    assert_eq!(1, info.pieces.len());
    assert_eq!("5", info.pieces);
    assert!(info.length.is_some());
    assert_eq!(Some(11), info.length);

    Ok(())
}

#[test]
fn decode_torrent_succeeds() -> Result<()> {
    Torrent::from_file("../torrents/archlinux-2020.02.01-x86_64.iso.torrent")?;

    Ok(())
}

#[test]
fn encode_info_succeeds() {
    let info = Info {
        name: "name".to_string(),
        piece_length: 5,
        pieces: "pieces".to_string(),
        length: Some(10),
        files: None,
    };

    let encoded = info.encode().unwrap();
    assert_eq!(
        "d4:name4:name12:piece lengthi5e6:pieces6:pieces6:lengthi10ee",
        encoded
    );
}
