use super::*;

#[test]
fn get_host_and_path_splits_correct() -> Result<()> {
    let (host, path) = get_host_and_path("http://tracker.archlinux.org:6969/announce")?;

    assert_eq!("tracker.archlinux.org:6969", host);
    assert_eq!("/announce", path);
    Ok(())
}

#[test]
fn get_content_returns_correct() {
    let input = b"one\r\ntwo\r\nthree\r\n\r\nfour
five
six\r\nseven\r\n\r\neight";
    let content = get_content(&input[..]);
    assert_eq!(
        b"four
five
six\r\nseven\r\n\r\neight"
            .to_vec(),
        content
    );
}
