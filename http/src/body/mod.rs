#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::io::{Error, ErrorKind, Result};

const CONTENT_LENGTH: &str = "content-length";
const TRANSFER_ENCODING: &str = "transfer-encoding";
const ENCODING_CHUNKED: &str = "chunked";

/// State represents the current internal state of body
/// reader.
struct State {
    /// The body that has been read so far
    result: Vec<u8>,
    /// Buffer for chunked encoding length
    count: String,
    /// Whether or not the last character was \r
    carriage_return: bool,
    /// Whether or not we're reading actual body bytes
    reading_body: bool,
}

impl State {
    /// Creates new instance with default values
    fn new() -> State {
        State {
            result: vec![],
            count: String::new(),
            carriage_return: false,
            reading_body: false,
        }
    }

    /// Sets flags for when \r was encountered
    fn read_carriage_return(&mut self) {
        self.carriage_return = true;
        self.reading_body = !self.reading_body;
    }

    /// Should be called after reading \n
    ///
    /// Checks if \r has been seen before and reads the body.
    /// Returns Ok(true) if last chunk was read, Ok(false) if
    /// more data is required and Err if an error ocurred while
    /// reading.
    fn read_newline<T>(&mut self, bytes: &mut T) -> Result<bool>
    where
        T: Iterator<Item = Result<u8>>,
    {
        if !self.carriage_return {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid format: no carriage return before \\n",
            ));
        }
        if !self.reading_body {
            return Ok(false);
        }
        if read_chunk(&self.count, &mut self.result, bytes)? {
            return Ok(true);
        }
        self.count = String::new();
        self.carriage_return = false;
        Ok(false)
    }
}

/// Attempts to read the body of a http request.
///
/// Supports plain and chunked encodings.
/// Returns a `Vec<u8>`, as the data may not be UTF-8 valid.
pub(crate) fn handle_http_body<T>(
    headers: &HashMap<String, String>,
    bytes: &mut T,
) -> Result<Vec<u8>>
where
    T: Iterator<Item = Result<u8>>,
{
    if let Some(value) = headers.get(CONTENT_LENGTH) {
        let max = match value.parse() {
            Ok(value) => value,
            Err(error) => return Err(Error::new(ErrorKind::InvalidData, format!("{}", error))),
        };
        return Ok(bytes.take(max).filter_map(|x| x.ok()).collect());
    }
    if let Some(value) = headers.get(TRANSFER_ENCODING) {
        if value == ENCODING_CHUNKED {
            return handle_http_chunked_body(bytes);
        }
    }
    Ok(bytes.filter_map(|x| x.ok()).collect())
}

/// Reads chunked http body.
///
/// Attempts to read chunk length and content if applicable.
fn handle_http_chunked_body<T>(bytes: &mut T) -> Result<Vec<u8>>
where
    T: Iterator<Item = Result<u8>>,
{
    let mut state = State::new();

    while let Some(byte) = bytes.next() {
        let byte = byte?;

        match byte {
            b'0'..=b'9' | b'A'..=b'F' => state.count.push(byte as char),
            13 => {
                state.read_carriage_return();
            }
            10 => {
                if state.read_newline(bytes)? {
                    return Ok(state.result);
                }
            }
            value => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Unexpected value: {}", value),
                ));
            }
        }
    }

    Err(Error::new(
        ErrorKind::UnexpectedEof,
        "Stream finished before fully read",
    ))
}

/// Reads individual chunk.
///
/// Tries to read `count` bytes, appending them to `result` if possible.
/// If chunk of length 0 is encountered returns Ok(true).
fn read_chunk<T>(count: &str, result: &mut Vec<u8>, bytes: &mut T) -> Result<bool>
where
    T: Iterator<Item = Result<u8>>,
{
    match usize::from_str_radix(count, 16) {
        Ok(value) => {
            if value == 0 {
                return Ok(true);
            }
            result.extend(bytes.take(value).flat_map(|x| x.ok()));
            Ok(false)
        }
        Err(error) => Err(Error::new(ErrorKind::InvalidData, format!("{}", error))),
    }
}
