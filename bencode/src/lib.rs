//! # bencode
//!
//! A [BEP 0003](https://www.bittorrent.org/beps/bep_0003.html) compliant
//! bencode implementation and ways to encode and decode values. It also
//! exposes functions for reading bencode from file (e.g. torrents), from
//! `BufRead` trait.
//!
//! It also exposes `BencodeValue` enum for manual decoding as well as
//! `Encodable` and `Decodable` traits to implement for encoding and
//! decoding structs. These work well with `bencode_derive` crate.
mod bytestring;
mod decode;
mod encode;
#[cfg(test)]
mod tests;

pub use bytestring::*;
pub use decode::decode;
pub use decode::Decodable;
pub use encode::encode;
pub use encode::Encodable;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};
use std::path::Path;
use std::str;

/// `BencodeValue` is an enum to store different bencode types.
///
/// # Values
///
/// [`Integer`]: enum.BencodeValue.html#variant.Integer
/// [`String`]: enum.BencodeValue.html#variant.String
/// [`ByteString`]: enum.BencodeValue.html#variant.ByteString
/// [`List`]: enum.BencodeValue.html#variant.List
/// [`Dictionary`]: enum.BencodeValue.html#variant.Dictionary
#[derive(PartialEq)]
pub enum BencodeValue {
    /// Can store any integer value that fits in i64
    Integer(i64),
    /// Can store any valid UTF-8 string
    String(String),
    /// Can store any string, regardless of encoding, including invalid UTF-8
    ByteString(Vec<u8>),
    /// Can store a vector of values, which don't need to be of the same type
    List(Vec<BencodeValue>),
    /// Can store a dictionary, keys are always represented with strings, values
    /// can be any valid bencode value.
    Dictionary(HashMap<String, BencodeValue>),
}

/// Load file from disk for the given path.
///
/// # Arguments
///
/// * `file_name` - the path to load the file from
pub fn from_file<P: AsRef<Path>>(file_name: P) -> Result<BencodeValue> {
    let file = File::open(file_name)?;
    let mut reader = BufReader::new(file);

    read(&mut reader)
}

/// Read a value from given reader
///
/// # Arguments
///
/// * `reader` - mutable reader that allows decoding of `BencodeValue`s
///
/// # Example
///
/// ```
/// use crate::bencode::BencodeValue;
/// use std::io::BufRead;
///
/// let mut input = "i108e".as_bytes();
/// let value = bencode::read(&mut input).unwrap();
///
/// if let BencodeValue::Integer(value) = value {
///     assert_eq!(108, value);
/// } else {
///     panic!("Value was not an integer");
/// }
/// ```
pub fn read<T: BufRead>(reader: &mut T) -> Result<BencodeValue> {
    let mut type_buffer = [0u8; 1];
    reader.read_exact(&mut type_buffer)?;

    select_next_type(reader, type_buffer[0])
}

/// Selects the next type when reading, delegating the reading
/// to dedicated functions based on type.
fn select_next_type<T: BufRead>(reader: &mut T, type_token: u8) -> Result<BencodeValue> {
    match type_token {
        b'i' => read_integer(reader),
        value if (b'0'..=b'9').contains(&value) => read_string(reader, value),
        b'l' => read_list(reader),
        b'd' => read_dictionary(reader),
        value => Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Unknown type: {}", value),
        )),
    }
}

/// Reads an integer from the reader or returns an error if encoding
/// is not valid.
fn read_integer<T: BufRead>(reader: &mut T) -> Result<BencodeValue> {
    let mut pending = String::new();

    loop {
        let mut buffer = [0u8; 1];
        reader.read_exact(&mut buffer)?;

        match buffer[0] {
            b'e' => break,
            b'-' if pending.is_empty() => pending.push('-'),
            value if (b'0'..=b'9').contains(&value) => pending.push(value as char),
            value => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Expected integer value but was {}", value),
                ))
            }
        }
    }

    if pending.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Encountered integer with length of 0.",
        ));
    }

    match pending.parse() {
        Ok(value) => Ok(BencodeValue::Integer(value)),
        Err(error) => Err(Error::new(ErrorKind::InvalidData, error)),
    }
}

/// Reads a string from the reader if possible
///
/// # Arguments
///
/// * `reader` - the reader from which to read the string
/// * `first` - the first byte of length of the format
fn read_string<T: BufRead>(reader: &mut T, first: u8) -> Result<BencodeValue> {
    let string_length = read_string_length(reader, first)?;

    let mut buffer = vec![0u8; string_length];

    reader.read_exact(&mut buffer)?;

    match str::from_utf8(&buffer) {
        Ok(value) => Ok(BencodeValue::String(value.to_string())),
        _ => Ok(BencodeValue::ByteString(buffer)),
    }
}

/// Reads the length of the string from reader.
///
/// # Arguments
///
/// * `reader` -  the source from which to read the value
/// * `first` - the first byte of the length
fn read_string_length<T: BufRead>(reader: &mut T, first: u8) -> Result<usize> {
    let mut pending_length = String::new();
    pending_length.push(first as char);

    loop {
        let mut buffer = [0u8; 1];
        reader.read_exact(&mut buffer)?;

        match buffer[0] {
            b':' => break,
            value if (b'0'..=b'9').contains(&value) => pending_length.push(value as char),
            value => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Invalid character in string length: {}", value),
                ))
            }
        }
    }

    match pending_length.parse() {
        Ok(value) => Ok(value),
        Err(error) => Err(Error::new(ErrorKind::InvalidData, error)),
    }
}

/// Reads a list from the input, deserializing items in the process.
fn read_list<T: BufRead>(reader: &mut T) -> Result<BencodeValue> {
    let mut items = vec![];

    let mut buffer = [0u8; 1];
    loop {
        reader.read_exact(&mut buffer)?;
        if buffer[0] == b'e' {
            break;
        }

        let item = select_next_type(reader, buffer[0])?;
        items.push(item);
    }

    Ok(BencodeValue::List(items))
}

/// Reads a dictionary from the input, also validating keys and decoding
/// the values in the process.
fn read_dictionary<T: BufRead>(reader: &mut T) -> Result<BencodeValue> {
    let mut map = HashMap::new();

    let mut buffer = [0u8; 1];
    loop {
        reader.read_exact(&mut buffer)?;
        if buffer[0] == b'e' {
            break;
        }

        let key = read_key(reader, buffer[0])?;
        reader.read_exact(&mut buffer)?;
        let value = select_next_type(reader, buffer[0])?;

        map.insert(key, value);
    }

    Ok(BencodeValue::Dictionary(map))
}

/// Reads a key for a map, ensuring it's a valid string
fn read_key<T: BufRead>(reader: &mut T, type_token: u8) -> Result<String> {
    let value = select_next_type(reader, type_token)?;
    match value {
        BencodeValue::String(value) => Ok(value),
        _ => Err(Error::new(
            ErrorKind::InvalidInput,
            "The key of the dictionary was not a string",
        )),
    }
}
