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
pub use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};
use std::str;

#[derive(PartialEq)]
pub enum BencodeValue {
    Integer(i64),
    String(String),
    ByteString(Vec<u8>),
    List(Vec<BencodeValue>),
    Dictionary(HashMap<String, BencodeValue>),
}

pub fn from_file(file_name: &str) -> Result<BencodeValue> {
    let file = File::open(file_name)?;
    let mut reader = BufReader::new(file);

    read(&mut reader)
}

fn read<T: BufRead>(reader: &mut T) -> Result<BencodeValue> {
    let mut type_buffer = [0u8; 1];
    reader.read_exact(&mut type_buffer)?;

    select_next_type(reader, type_buffer[0])
}

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

fn read_string<T: BufRead>(reader: &mut T, first: u8) -> Result<BencodeValue> {
    let string_length = read_string_length(reader, first)?;

    let mut buffer = vec![0u8; string_length];

    reader.read_exact(&mut buffer)?;

    match str::from_utf8(&buffer) {
        Ok(value) => Ok(BencodeValue::String(value.to_string())),
        _ => Ok(BencodeValue::ByteString(buffer)),
    }
}

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
