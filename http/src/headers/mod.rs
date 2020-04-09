#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::io::Result;

enum State {
    Status,
    HeaderNext,
    Header,
    Body,
}

pub(crate) struct HttpHeaders {
    state: State,
    carriage_return: bool,
    protocol: String,
    pub(super) headers: HashMap<String, String>,
}

impl HttpHeaders {
    pub(crate) fn new() -> HttpHeaders {
        HttpHeaders {
            state: State::Status,
            carriage_return: false,
            protocol: String::new(),
            headers: HashMap::new(),
        }
    }

    fn handle_next_http(&mut self, buffer: &mut String, byte: u8) {
        if self.carriage_return && byte == 10 {
            self.carriage_return = false;
            self.state = match self.state {
                State::Status => State::HeaderNext,
                State::Header => State::HeaderNext,
                State::HeaderNext => State::Body,
                State::Body => return,
            };
            return;
        }
        if byte == 13 {
            self.carriage_return = true;
            return;
        }
        buffer.push(byte as char);
        if matches!(self.state, State::HeaderNext) {
            self.state = State::Header
        }
        self.carriage_return = false;
    }

    fn add_header(&mut self, header_line: String) {
        let split_index = header_line.find(": ");

        match split_index {
            Some(index) => {
                let (name, value) = header_line.split_at(index);
                self.headers
                    .insert(name.to_lowercase().to_string(), value[2..].to_string());
            }
            None => {
                self.headers.insert(header_line, String::new());
            }
        }
    }
}

pub(crate) fn handle_http_headers<T>(bytes: &mut T) -> Result<HttpHeaders>
where
    T: Iterator<Item = Result<u8>>,
{
    let mut headers = HttpHeaders::new();

    let mut buffer = String::new();
    for byte in bytes {
        let byte = byte?;

        headers.handle_next_http(&mut buffer, byte);
        if matches!(headers.state, State::HeaderNext) && !headers.carriage_return {
            if headers.protocol.is_empty() {
                headers.protocol = buffer;
            } else {
                headers.add_header(buffer);
            }
            buffer = String::new();
        }
        if matches!(headers.state, State::Body) {
            break;
        }
    }
    Ok(headers)
}
