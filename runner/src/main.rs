use directories;
use std::fmt::Display;
use std::process;
use watcher::ObservationEvent;
use watcher::Watcher;

fn main() {
    directories::must_exist("torrents").unwrap_or_else(log_exit);

    let mut watcher = Watcher::new("torrents").unwrap_or_else(|err| {
        log_exit(err);
        panic!("Did not exit");
    });

    let receiver = watcher.start_observation().expect("Double observation");

    while let Ok(value) = receiver.recv() {
        match value {
            ObservationEvent::Created(file) => println!("Created file {}", file),
            ObservationEvent::Deleted(file) => println!("Deleted file {}", file),
            ObservationEvent::Modified(file) => println!("Modified file {}", file),
            ObservationEvent::Move { from, to } => println!("Moved from {} to {}", from, to),
        }
    }
}

fn log_exit<E: Display>(error: E) {
    eprintln!("Unrecoverable error: {}", error);
    process::exit(1);
}
