use super::BencodeValue;
use std::collections::HashMap;
use std::iter;

macro_rules! impl_encodable {
    ($($x:ty),*) => {
        $(
            impl Encodable for $x {
                fn to_bencode(&self) -> Option<BencodeValue> {
                    Some(BencodeValue::Integer(*self as i64))
                }
            }
        )*
    };
}

impl_encodable!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);

pub trait Encodable {
    fn encode(&self) -> Option<Vec<u8>> {
        match self.to_bencode() {
            Some(value) => Some(encode(&value)),
            None => None,
        }
    }
    fn to_bencode(&self) -> Option<BencodeValue>;
}

impl Encodable for String {
    fn encode(&self) -> Option<Vec<u8>> {
        Some(format!("{}:{}", self.len(), self.to_string()).into_bytes())
    }

    fn to_bencode(&self) -> Option<BencodeValue> {
        Some(BencodeValue::String(self.to_string()))
    }
}

impl<T: Encodable> Encodable for Option<T> {
    fn to_bencode(&self) -> Option<BencodeValue> {
        match self {
            Some(value) => value.to_bencode(),
            None => None,
        }
    }
}

impl<T: Encodable> Encodable for Vec<T> {
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

pub fn encode(value: &BencodeValue) -> Vec<u8> {
    match value {
        BencodeValue::ByteString(value) => encode_byte_string(value),
        BencodeValue::Dictionary(map) => encode_map(map),
        BencodeValue::Integer(value) => encode_integer(*value),
        BencodeValue::List(list) => encode_list(list),
        BencodeValue::String(value) => value.encode().unwrap(), // We never return None
    }
}

fn encode_byte_string(value: &[u8]) -> Vec<u8> {
    let mut output = format!("{}:", value.len()).into_bytes();
    output.extend(value.iter().copied());
    output
}

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

fn encode_integer(value: i64) -> Vec<u8> {
    format!("i{}e", value).into_bytes()
}

fn encode_list(list: &[BencodeValue]) -> Vec<u8> {
    let items = list.iter().map(|element| encode(element)).flatten();

    iter::once(b'l')
        .chain(items)
        .chain(iter::once(b'e'))
        .collect()
}
