use super::*;

#[test]
fn get_host_and_path_splits_correct() -> Result<()> {
    let (host, path) = get_host_and_path("http://tracker.archlinux.org:6969/announce")?;

    assert_eq!("tracker.archlinux.org:6969", host);
    assert_eq!("/announce", path);
    Ok(())
}
