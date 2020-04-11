use super::{BencodeValue, Decodable, Encodable};
use std::io::{Error, ErrorKind, Result};

/// Represents a byte string, something that may not be
/// a valid UTF-8 string. It is backed by `Vec<u8>`.
#[derive(Debug, PartialEq)]
pub struct ByteString(Vec<u8>);

impl ByteString {
    /// Creates a new instance of `ByteString`.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to wrap in a `ByteString`
    pub fn new(value: Vec<u8>) -> ByteString {
        ByteString(value)
    }

    /// Returns the length of the backing Vec&lt;u8&gt;.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the backing `Vec<u8>` is empty, false otherwise.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Decodable for ByteString {
    type Output = ByteString;

    /// Default decode implementation for deserialization.
    ///
    /// The value will be decoded for both `Bencode::ByteString` and `Bencode::String`.
    fn decode(value: &BencodeValue) -> Result<ByteString> {
        match value {
            BencodeValue::ByteString(value) => read_byte_string(value),
            BencodeValue::String(value) => read_byte_string(&value.to_string().into_bytes()),
            _ => Err(Error::new(ErrorKind::InvalidData, "expected string")),
        }
    }
}

impl Encodable for ByteString {
    /// Default encode implementation for serialization.
    ///
    /// Encoding never fails.
    fn to_bencode(&self) -> Option<BencodeValue> {
        Some(BencodeValue::ByteString(self.0.iter().copied().collect()))
    }
}

/// This function will copy the input vector and return a new instance.
///
/// Reading never fails, optional is used for convenience while decoding.
fn read_byte_string(value: &[u8]) -> Result<ByteString> {
    Ok(ByteString(value.iter().copied().collect()))
}
