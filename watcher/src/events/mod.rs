#[cfg(unix)]
use crate::unix;
use std::io::Error;
use std::io::Result as IoResult;
use std::mem;
use std::str;

const MAX_EVENTS: usize = 1024;
const MAX_NAME_LENGTH: usize = 1024;
const EVENT_SIZE: usize = mem::size_of::<unix::InotifyEvent>();
const BUFFER_LEN: usize = MAX_EVENTS * (EVENT_SIZE + MAX_NAME_LENGTH);

/// Internal file event
#[derive(Debug)]
pub(crate) struct FileEvent {
    /// The name of file for which the even occurred
    pub(crate) file_name: String,
    /// the type of the event
    pub(crate) event_type: EventType,
    /// Cookie that remains same if an event has multiple parts
    pub(crate) cookie: u32,
}

/// Supported events
#[derive(Debug)]
pub(crate) enum EventType {
    /// File was modified
    MODIFY,
    /// File was moved (has multiple parts)
    MOVED,
    /// File was created
    CREATED,
    /// File was deleted
    DELETED,
    /// An unmapped event occurred
    UNKNOWN,
}

/// Reads the event from given file descriptor
pub(crate) fn read(file_descriptor: i32) -> IoResult<Vec<FileEvent>> {
    let buffer = [0u8; BUFFER_LEN];
    let read_size = unsafe { unix::read(file_descriptor, buffer.as_ptr() as *mut u8, BUFFER_LEN) };
    if read_size < 0 {
        return Err(Error::last_os_error());
    }

    let mut events: Vec<FileEvent> = Vec::new();

    let mut buffer = &buffer[..read_size as usize];
    while buffer.len() >= unix::STRUCT_SIZE {
        let (consumed, event) = consume(buffer);
        buffer = &buffer[consumed..];

        events.push(event);
    }

    Ok(events)
}

/// Consumes non-static data from the event
fn consume(buffer: &[u8]) -> (usize, FileEvent) {
    #[allow(clippy::cast_ptr_alignment)]
    let event = buffer.as_ptr() as *const unix::InotifyEvent;
    let event = unsafe { &*event };

    let name_end = unix::STRUCT_SIZE + event.length as usize;

    let name = &buffer[unix::STRUCT_SIZE..name_end];
    let name = str::from_utf8(name)
        .expect("Expected UTF8 string")
        .split('\0')
        .collect::<Vec<&str>>()[0];

    let event_type = if event.mask & unix::IN_MODIFY != 0 {
        EventType::MODIFY
    } else if event.mask & unix::IN_MOVED != 0 {
        EventType::MOVED
    } else if event.mask & unix::IN_CREATE != 0 {
        EventType::CREATED
    } else if event.mask & unix::IN_DELETE != 0 {
        EventType::DELETED
    } else {
        EventType::UNKNOWN
    };

    let event = FileEvent {
        file_name: String::from(name),
        event_type,
        cookie: event.cookie,
    };

    (name_end, event)
}
