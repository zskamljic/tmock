use directories;
use std::fmt::Display;
use std::process;
use watcher::ObservationEvent;
use watcher::Watcher;

fn main() {
    directories::must_exist("torrents").unwrap_or_else(log_exit);

    // We made sure that the directory exists above
    let mut entries = runner::load_existing_entries().unwrap();

    let mut watcher = Watcher::new("torrents").unwrap_or_else(|err| {
        log_exit(err);
        panic!("Did not exit");
    });

    let receiver = watcher.start_observation().expect("Double observation");

    while let Ok(value) = receiver.recv() {
        match value {
            ObservationEvent::Created(file) => runner::add_file_entry(&mut entries, &file),
            ObservationEvent::Deleted(file) => runner::remove_file_entry(&mut entries, &file),
            ObservationEvent::Modified(file) => {
                runner::remove_file_entry(&mut entries, &file);
                runner::add_file_entry(&mut entries, &file);
            }
            ObservationEvent::Move { from, to } => runner::move_file_entry(&mut entries, from, to),
        }
    }
}

fn log_exit<E: Display>(error: E) {
    eprintln!("Unrecoverable error: {}", error);
    process::exit(1);
}
