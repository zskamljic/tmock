use super::BencodeValue;
use std::collections::HashMap;

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
    fn encode(&self) -> Option<String> {
        match self.to_bencode() {
            Some(value) => Some(encode(&value)),
            None => None,
        }
    }
    fn to_bencode(&self) -> Option<BencodeValue>;
}

impl Encodable for String {
    fn encode(&self) -> Option<String> {
        Some(format!("{}:{}", self.len(), self.to_string()))
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

pub fn encode(value: &BencodeValue) -> String {
    match value {
        BencodeValue::Dictionary(map) => encode_map(map),
        BencodeValue::Integer(value) => encode_integer(*value),
        BencodeValue::List(list) => encode_list(list),
        BencodeValue::String(value) => value.encode().unwrap(), // We never return None
    }
}

fn encode_map(map: &HashMap<String, BencodeValue>) -> String {
    let mut output = String::from("d");

    for (key, value) in map {
        output.push_str(&format!("{}:{}{}", key.len(), key, encode(value)));
    }

    output.push_str("e");
    output
}

fn encode_integer(value: i64) -> String {
    format!("i{}e", value)
}

fn encode_list(list: &[BencodeValue]) -> String {
    let elements: Vec<String> = list.iter().map(|element| encode(element)).collect();
    format!("l{}e", elements.join(""))
}
