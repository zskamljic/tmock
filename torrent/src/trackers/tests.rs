use super::*;
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
        "http://tracker.archlinux.org:6969/announce",
        &Info {
            name: "".to_string(),
            piece_length: 1,
            pieces: "".to_string(),
            files: None,
            length: None,
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

    let parameter_str = create_parameters(&info);

    assert_eq!(
        "?info_hash=%A7%5E%11%C1%8E%17%12%07%F5%E0%21%84z%3EwFH%7D%B6%E0",
        parameter_str
    );
}

#[test]
fn url_encode_encodes_correct() {
    let input = [b'a', b'A', 0, b'('];
    let output = url_encode(&input);

    assert_eq!("aA%00%28", output);
}
