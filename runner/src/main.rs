mod config;
mod state;

use config::Config;
use directories;
use mock::Client;
use state::State;
use std::fmt::Display;
use std::process;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;
use watcher::ObservationEvent;
use watcher::Watcher;

fn main() {
    directories::must_exist("torrents").unwrap_or_else(log_exit);

    // We made sure that the directory exists above
    let existing = runner::load_existing_entries().unwrap();

    println!("Loaded files:");
    for key in existing.keys() {
        println!("\t{}", key);
    }

    let config = match Config::new() {
        Ok(value) => value,
        Err(error) => {
            log_exit(error);
            return;
        }
    };
    let client = Client::new();
    let mut state = State::new(&client, config.min, config.max);

    for (key, torrent) in existing {
        state.add_announcer(key, torrent);
    }

    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || observe(sender));

    loop {
        while let Ok(value) = receiver.try_recv() {
            match value {
                ObservationEvent::Created(file) => state.load_new(file),
                ObservationEvent::Deleted(file) => state.remove(file),
                ObservationEvent::Modified(file) => state.modified(file),
                ObservationEvent::Move { from, to } => state.move_file(from, to),
            }
        }

        state.announce_all();

        thread::sleep(Duration::from_secs(1));
    }
}

fn observe(sender: Sender<ObservationEvent>) {
    let mut watcher = Watcher::new("torrents").unwrap_or_else(|err| {
        log_exit(err);
        panic!("Did not exit");
    });

    let receiver = watcher.start_observation().expect("Double observation");
    while let Ok(value) = receiver.recv() {
        if let Err(error) = sender.send(value) {
            eprintln!("Error sending value: {}", error);
        }
    }
}

fn log_exit<E: Display>(error: E) {
    eprintln!("Unrecoverable error: {}", error);
    process::exit(1);
}
