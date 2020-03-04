use crate::events::{self, EventType, FileEvent};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};

pub enum ObservationEvent {
    Created(String),
    Deleted(String),
    Modified(String),
    Move { from: String, to: String },
}

pub fn observe(receiver: Receiver<()>, sender: Sender<ObservationEvent>, file_descriptor: i32) {
    let mut cookie_map = HashMap::new();

    loop {
        match events::read(file_descriptor) {
            Ok(events) => process_events(&mut cookie_map, events, &sender),
            Err(error) => println!("Error when reading event: {}", error),
        }
        if receiver.try_recv().is_ok() {
            break;
        }
    }
}

fn process_events(
    cookie_map: &mut HashMap<u32, String>,
    events: Vec<FileEvent>,
    sender: &Sender<ObservationEvent>,
) {
    for event in events {
        process_event(cookie_map, event, sender);
    }
}

fn process_event(
    cookie_map: &mut HashMap<u32, String>,
    event: FileEvent,
    sender: &Sender<ObservationEvent>,
) {
    let event = match event.event_type {
        EventType::CREATED => Some(ObservationEvent::Created(event.file_name)),
        EventType::DELETED => Some(ObservationEvent::Deleted(event.file_name)),
        EventType::MODIFY => Some(ObservationEvent::Modified(event.file_name)),
        EventType::MOVED => process_move_event(cookie_map, event),
        _ => None,
    };

    if let Some(event) = event {
        sender.send(event).unwrap(); // If receiver is closed we should fail anyway
    }
}

fn process_move_event(
    cookie_map: &mut HashMap<u32, String>,
    event: FileEvent,
) -> Option<ObservationEvent> {
    match cookie_map.entry(event.cookie) {
        Entry::Occupied(entry) => Some(ObservationEvent::Move {
            from: entry.remove(),
            to: event.file_name,
        }),
        Entry::Vacant(entry) => {
            entry.insert(event.file_name);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_move_event_inserts_new() {
        let mut cookies = HashMap::new();
        let event = FileEvent {
            cookie: 0,
            event_type: EventType::MODIFY,
            file_name: String::from("file"),
        };

        match process_move_event(&mut cookies, event) {
            Some(_) => panic!("Value should not be present"),
            _ => assert_eq!(1, cookies.len()),
        }
    }

    #[test]
    fn process_move_event_returns_move_event() {
        let mut cookies = HashMap::new();
        cookies.insert(0, String::from("orig"));

        let event = FileEvent {
            cookie: 0,
            event_type: EventType::MODIFY,
            file_name: String::from("target"),
        };

        match process_move_event(&mut cookies, event) {
            Some(ObservationEvent::Move { from, to }) => {
                assert_eq!("orig", from);
                assert_eq!("target", to);
            }
            _ => panic!("Must return an event"),
        }

        assert_eq!(0, cookies.len());
    }
}
