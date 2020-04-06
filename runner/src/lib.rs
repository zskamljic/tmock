use std::collections::HashMap;
use std::fs;
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
