use super::{BencodeValue, Decodable, Encodable};
use std::io::{Error, ErrorKind, Result};

#[derive(Debug, PartialEq)]
pub struct ByteString(Vec<u8>);

impl ByteString {
    pub fn new(value: Vec<u8>) -> ByteString {
        ByteString(value)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Decodable for ByteString {
    type Output = ByteString;

    fn decode(value: &BencodeValue) -> Result<ByteString> {
        match value {
            BencodeValue::ByteString(value) => read_byte_string(value),
            BencodeValue::String(value) => read_byte_string(&value.to_string().into_bytes()),
            _ => Err(Error::new(ErrorKind::InvalidData, "expected string")),
        }
    }
}

impl Encodable for ByteString {
    fn to_bencode(&self) -> Option<BencodeValue> {
        Some(BencodeValue::ByteString(self.0.iter().copied().collect()))
    }
}

fn read_byte_string(value: &Vec<u8>) -> Result<ByteString> {
    Ok(ByteString(value.iter().copied().collect()))
}
