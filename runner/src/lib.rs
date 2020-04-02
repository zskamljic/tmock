use mock::Announcer;
use mock::Client;
use std::collections::HashMap;
use std::fs;
use std::hash::BuildHasher;
use std::io::Result;
use std::path::PathBuf;
use torrent::{Decodable, Torrent};

fn is_torrent(path: &PathBuf) -> bool {
    if let Some(extension) = path.extension() {
        return extension
            .to_str()
            .map(|string| string.ends_with("torrent"))
            .unwrap_or(false);
    }
    false
}

fn unwrap_path_content((path, value): (String, Result<Torrent>)) -> Option<(String, Torrent)> {
    match value {
        Ok(value) => Some((path, value)),
        Err(error) => {
            eprintln!("Error when loading {}: {}", path, error);
            None
        }
    }
}

pub fn load_existing_entries() -> Result<HashMap<String, Torrent>> {
    Ok(fs::read_dir("torrents")?
        .map(|file| file.map(|entry| entry.path()))
        .filter_map(Result::ok)
        .filter(is_torrent)
        .map(|path| path.to_str().map(String::from))
        .flatten()
        .map(|path| (String::from(&path), Torrent::from_file(&path)))
        .filter_map(unwrap_path_content)
        .collect())
}

pub fn load_and_store<'a, S: BuildHasher>(
    hashes: &mut HashMap<String, String, S>,
    announcers: &mut HashMap<String, Announcer<'a>, S>,
    file: String,
    client: &'a Client,
) {
    if let Ok(torrent) = Torrent::from_file(&file) {
        store_announcer(hashes, announcers, file, torrent, &client);
    }
}

pub fn remove_torrent<S: BuildHasher>(
    hashes: &mut HashMap<String, String, S>,
    announcers: &mut HashMap<String, Announcer, S>,
    file: &str,
) {
    if let Some(hash) = hashes.remove(file) {
        announcers.remove(&hash);
    }
}

pub fn store_announcer<'a, S: BuildHasher>(
    hashes: &mut HashMap<String, String, S>,
    announcers: &mut HashMap<String, Announcer<'a>, S>,
    key: String,
    torrent: Torrent,
    client: &'a Client,
) {
    let info_hash = torrent.get_info_hash();
    let announcer = Announcer::new(torrent, &client);
    announcers.insert(info_hash.to_string(), announcer);
    hashes.insert(key, info_hash);
}
