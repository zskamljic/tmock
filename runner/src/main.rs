use directories;
use mock::Client;
use std::collections::HashMap;
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
    let client = Client::new();

    let mut hashes = HashMap::new();
    let mut announcers = HashMap::new();

    for (key, torrent) in existing {
        runner::store_announcer(&mut hashes, &mut announcers, key, torrent, &client);
    }

    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        observe(sender);
    });

    loop {
        while let Ok(value) = receiver.try_recv() {
            match value {
                ObservationEvent::Created(file) => {
                    runner::load_and_store(&mut hashes, &mut announcers, file, &client)
                }
                ObservationEvent::Deleted(file) => {
                    runner::remove_torrent(&mut hashes, &mut announcers, &file)
                }
                ObservationEvent::Modified(file) => {
                    runner::remove_torrent(&mut hashes, &mut announcers, &file);
                    runner::load_and_store(&mut hashes, &mut announcers, file, &client)
                }
                ObservationEvent::Move { from, to } => {
                    if let Some(value) = hashes.remove(&from) {
                        hashes.insert(to, value);
                    }
                }
            }
        }

        for (_, value) in announcers.iter_mut() {
            value.announce();
        }

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
