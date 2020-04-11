use super::BencodeValue;
use std::collections::HashMap;
use std::iter;

/// Internal macro used as a shorthand to implement `Encodable` for
/// all integer types.
macro_rules! impl_encodable {
    ($($x:ty),*) => {
        $(
            impl Encodable for $x {

                /// Implements encoding to `BencodeValue` for this type.
                /// Encoding integer values never fails.
                fn to_bencode(&self) -> Option<BencodeValue> {
                    Some(BencodeValue::Integer(*self as i64))
                }
            }
        )*
    };
}

impl_encodable!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);

pub trait Encodable {
    /// Used as a shorthand to encode value to `Vec<u8>`.
    ///
    /// The resulting vector may not be a valid UTF-8 string.
    fn encode(&self) -> Option<Vec<u8>> {
        match self.to_bencode() {
            Some(value) => Some(encode(&value)),
            None => None,
        }
    }

    /// If None is returned from here the value will not be encoded.
    fn to_bencode(&self) -> Option<BencodeValue>;
}

impl Encodable for String {
    /// Default implementation of `Encodable` for `String` as it has a special format.
    fn encode(&self) -> Option<Vec<u8>> {
        Some(format!("{}:{}", self.len(), self.to_string()).into_bytes())
    }

    /// Copies self and creates `BencodeValue::String`.
    fn to_bencode(&self) -> Option<BencodeValue> {
        Some(BencodeValue::String(self.to_string()))
    }
}

impl<T: Encodable> Encodable for Option<T> {
    /// Encodable implementation for `Option<T>`. If the value being
    /// encoded is `None` the result will be `None` as well, otherwise
    /// the encoding is delegated to T.to_bencode()
    fn to_bencode(&self) -> Option<BencodeValue> {
        match self {
            Some(value) => value.to_bencode(),
            None => None,
        }
    }
}

impl<T: Encodable> Encodable for Vec<T> {
    /// Encodable implementation for `Vec<T>`.
    ///
    /// Individual items will be encoded and potentially missing
    /// values will be removed.
    fn to_bencode(&self) -> Option<BencodeValue> {
        Some(BencodeValue::List(
            self.iter()
                .map(|element| element.to_bencode())
                .filter(|element| element.is_some())
                .map(|element| element.unwrap())
                .collect(),
        ))
    }
}

/// Public function to decode any `BencodeValue` to `Vec<u8>`.
///
/// # Arguments
///
/// * `value` - the value to encode
///
/// # Example
///
/// ```
/// use crate::bencode::{BencodeValue, Encodable};
/// use crate::bencode;
///
/// let value = BencodeValue::Integer(108);
/// let result = bencode::encode(&value);
///
/// assert_eq!(b"i108e".to_vec(), result);
/// ```
pub fn encode(value: &BencodeValue) -> Vec<u8> {
    match value {
        BencodeValue::ByteString(value) => encode_byte_string(value),
        BencodeValue::Dictionary(map) => encode_map(map),
        BencodeValue::Integer(value) => encode_integer(*value),
        BencodeValue::List(list) => encode_list(list),
        BencodeValue::String(value) => value.encode().unwrap(), // We never return None
    }
}

/// Encode array of bytes to a string.
///
/// Value is encoded into `Vec<u8>` since it may not be a valid UTF-8 string.
fn encode_byte_string(value: &[u8]) -> Vec<u8> {
    let mut output = format!("{}:", value.len()).into_bytes();
    output.extend(value.iter().copied());
    output
}

/// Encode hashmap to a dictionary.
///
/// Keys are encoded as strings, in alphabetic order as per BEP0003,
/// while values are encoded based on their type.
fn encode_map(map: &HashMap<String, BencodeValue>) -> Vec<u8> {
    let mut keys: Vec<_> = map.keys().collect();
    keys.sort();

    let mut output = Vec::new();
    for key in keys {
        output.extend(&format!("{}:{}", key.len(), key).into_bytes());
        output.extend(encode(&map[key]));
    }

    iter::once(b'd')
        .chain(output.into_iter())
        .chain(iter::once(b'e'))
        .collect()
}

/// Encodes an integer, only values that fit in `i64` can be encoded.
fn encode_integer(value: i64) -> Vec<u8> {
    format!("i{}e", value).into_bytes()
}

/// Encode a slice of values.
fn encode_list(list: &[BencodeValue]) -> Vec<u8> {
    let items = list.iter().map(|element| encode(element)).flatten();

    iter::once(b'l')
        .chain(items)
        .chain(iter::once(b'e'))
        .collect()
}
