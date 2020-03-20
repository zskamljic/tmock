use super::*;

#[test]
fn sha1_returns_example() {
    let mut sha1 = Sha1::new();
    sha1.update_str("The quick brown fox jumps over the lazy dog");
    let result = sha1.hex_digest();

    assert_eq!("2fd4e1c67a2d28fced849ee1bb76e7391b93eb12", result);
}

#[test]
fn sha1_returns_example2() {
    let mut sha1 = Sha1::new();
    sha1.update_str("The quick brown fox jumps over the lazy cog");
    let result = sha1.hex_digest();

    assert_eq!("de9f2c7fd25e1b3afad3e85a0bd17d9b100db4b3", result);
}
#[test]
fn sha1_returns_example3() {
    let mut sha1 = Sha1::new();
    sha1.update_str("");
    let result = sha1.hex_digest();

    assert_eq!("da39a3ee5e6b4b0d3255bfef95601890afd80709", result);
}
