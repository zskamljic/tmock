use bencode::BencodeValue;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::hash::BuildHasher;
use std::io::Result;
use std::path::PathBuf;

fn is_torrent(path: &PathBuf) -> bool {
    if let Some(extension) = path.extension() {
        return extension
            .to_str()
            .map(|string| string.ends_with("torrent"))
            .unwrap_or(false);
    }
    false
}

fn unwrap_path_content(
    (path, value): (String, Result<BencodeValue>),
) -> Option<(String, BencodeValue)> {
    if let Ok(value) = value {
        Some((path, value))
    } else {
        None
    }
}

pub fn load_existing_entries() -> Result<HashMap<String, BencodeValue>> {
    Ok(fs::read_dir("torrents")?
        .map(|file| file.map(|entry| entry.path()))
        .filter_map(Result::ok)
        .filter(is_torrent)
        .map(|path| path.to_str().map(String::from))
        .flatten()
        .map(|path| (String::from(&path), bencode::load(&path)))
        .filter_map(unwrap_path_content)
        .collect())
}

pub fn add_file_entry<S: BuildHasher>(map: &mut HashMap<String, BencodeValue, S>, file: &str) {
    if let Ok(value) = bencode::load(file) {
        map.insert(file.to_string(), value);
    }
    // TODO: report file added
}

pub fn remove_file_entry<S: BuildHasher>(map: &mut HashMap<String, BencodeValue, S>, file: &str) {
    if let Entry::Occupied(entry) = map.entry(file.to_string()) {
        entry.remove();
    }
    // TODO: report file removed
}

pub fn move_file_entry<S: BuildHasher>(
    map: &mut HashMap<String, BencodeValue, S>,
    from: String,
    to: String,
) {
    if let Entry::Occupied(value) = map.entry(from) {
        let value = value.remove();
        map.insert(to, value);
    }
}

// TODO: add tests
