struct Torrent {
    announce: String,
    info: Info,
}

struct Info {
    name: String,
    piece_length: usize,
    pieces: Vec<u8>,
    length: Option<usize>,
    files: Option<File>,
}

struct File {
    length: usize,
    path: Vec<String>,
}
