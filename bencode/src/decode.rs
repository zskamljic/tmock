use super::{from_file, read, BencodeValue};
use std::io::{BufRead, Error, ErrorKind, Result};

macro_rules! impl_decodable {
    ($($x:ty),*) => {
        $(
            impl Decodable for $x {
                type Output = $x;

                fn decode(value: &BencodeValue) -> Result<$x> {
                    if let BencodeValue::Integer(number) = value {
                        Ok(*number as $x)
                    } else {
                        Err(Error::new(ErrorKind::InvalidData, "Value was not an integer"))
                    }
                }
            }
        )*
    };
}

impl_decodable!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);

pub trait Decodable {
    type Output;
    fn decode(value: &BencodeValue) -> Result<Self::Output>;

    fn decode_option(value: Option<&BencodeValue>) -> Result<Self::Output> {
        match value {
            Some(value) => Self::decode(value),
            None => Err(Error::new(ErrorKind::NotFound, "Value was not present")),
        }
    }

    fn read<T: BufRead>(reader: &mut T) -> Result<Self::Output> {
        Self::decode(&read(reader)?)
    }

    fn from_file(file_name: &str) -> Result<Self::Output> {
        Self::decode(&from_file(file_name)?)
    }
}

impl<T: Decodable> Decodable for Vec<T> {
    type Output = Vec<T::Output>;

    fn decode(value: &BencodeValue) -> Result<Self::Output> {
        if let BencodeValue::List(list) = value {
            let mut result = Vec::with_capacity(list.len());
            for item in list {
                result.push(T::decode(item)?);
            }
            Ok(result)
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "Value was not an integer",
            ))
        }
    }
}

impl<T: Decodable> Decodable for Option<T> {
    type Output = Option<T::Output>;

    fn decode(value: &BencodeValue) -> Result<Self::Output> {
        match T::decode(value) {
            Ok(value) => Ok(Some(value)),
            Err(_) => Ok(None),
        }
    }

    fn decode_option(value: Option<&BencodeValue>) -> Result<Self::Output> {
        match value {
            Some(value) => Self::decode(value),
            None => Ok(None),
        }
    }
}

impl Decodable for String {
    type Output = String;

    fn decode(value: &BencodeValue) -> Result<String> {
        if let BencodeValue::String(string) = value {
            Ok(string.to_string())
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Not a valid string"))
        }
    }
}

pub fn decode<T: Decodable>(value: &BencodeValue, name: &str) -> Result<T::Output> {
    match value {
        BencodeValue::Dictionary(map) => T::decode_option(map.get(name)),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "The structure didn't match",
        )),
    }
}
