fn load(file_name: &str) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_bencoded() {
        load("archlinux-2020.02.01-x86_64.iso.torrent");
    }
}
