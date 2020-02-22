use directories;
use std::process;

fn main() {
    directories::must_exist("torrents").unwrap_or_else(log_exit);
}

fn log_exit(error: std::io::Error) {
    eprintln!("Unrecoverable error: {}", error);
    process::exit(1);
}
