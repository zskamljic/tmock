use super::{from_file, read, BencodeValue};
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/// Utility macro to implement Decodable for integer types.
/// Function will only return Err if the input value was not an integer.
macro_rules! impl_decodable {
    ($($x:ty),*) => {
        $(
            impl Decodable for $x {
                type Output = $x;

                /// Provided implementation for decoding this type.
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

    /// Decode should return `Ok` with the object being decoded in case of success,
    /// `std::io::Error` otherwise.
    ///
    /// # Arguments
    ///
    /// * `value` the BencodeValue to be mapped to Self::Output type.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::bencode::{BencodeValue, Decodable};
    ///
    /// let value = BencodeValue::Integer(108);
    /// let int_value = i32::decode(&value).unwrap();
    ///
    /// assert_eq!(108, int_value);
    /// ```
    fn decode(value: &BencodeValue) -> Result<Self::Output>;

    /// Default implementation to try and decode an optional `BencodeValue`.
    ///
    /// Delegates to decode if the value is present, returns NotFound error otherwise.
    ///
    /// # Arguments
    ///
    /// * `value` optional `BencodeValue` to try and map to `Self::Output`
    ///
    /// # Example
    ///
    /// ```
    /// use crate::bencode::{BencodeValue, Decodable};
    ///
    /// let value = Some(&BencodeValue::Integer(108));
    /// let int_value = i32::decode_option(value).unwrap();
    ///
    /// assert_eq!(108, int_value);
    /// ```
    fn decode_option(value: Option<&BencodeValue>) -> Result<Self::Output> {
        match value {
            Some(value) => Self::decode(value),
            None => Err(Error::new(ErrorKind::NotFound, "Value was not present")),
        }
    }

    /// Utility function to help with reading an array of bytes.
    ///
    /// # Arguments
    ///
    /// * `bytes` a slice of u8 to decode to implementing type.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::bencode::{BencodeValue, Decodable};
    ///
    /// let input = b"i108e";
    /// let int_value = i32::read_bytes(&input[..]).unwrap();
    ///
    /// assert_eq!(108, int_value);
    /// ```
    fn read_bytes(bytes: &[u8]) -> Result<Self::Output> {
        let mut reader = BufReader::new(bytes);
        Self::read(&mut reader)
    }

    /// Default implementation for reading the value from `BufRead`.
    ///
    /// # Arguments
    ///
    /// * `reader` - the reader from which to read the `Decodable`
    ///
    /// # Example
    ///
    /// ```
    /// use crate::bencode::{BencodeValue, Decodable};
    /// use std::io::BufRead;
    ///
    /// let mut input = "i108e".as_bytes();
    /// let int_value = i32::read(&mut input).unwrap();
    ///
    /// assert_eq!(108, int_value);
    /// ```
    fn read<T: BufRead>(reader: &mut T) -> Result<Self::Output> {
        Self::decode(&read(reader)?)
    }

    /// Utility function for reading the Decodable from a file.
    ///
    /// # Arguments
    ///
    /// * `file_name` - the file from which to read the Decodable.
    fn from_file(file_name: &str) -> Result<Self::Output> {
        Self::decode(&from_file(file_name)?)
    }
}

impl<T: Decodable> Decodable for Vec<T> {
    type Output = Vec<T::Output>;

    /// Default implementation for decoding a vector of Decodables.
    ///
    /// # Argument
    ///
    /// * `value` - the value to turn into vector
    ///
    /// # Example
    ///
    /// ```
    /// use crate::bencode::{BencodeValue, Decodable};
    ///
    /// let input = BencodeValue::List(vec![BencodeValue::Integer(107), BencodeValue::Integer(108), BencodeValue::Integer(109)]);
    /// let decoded = Vec::<i32>::decode(&input).unwrap();
    ///
    /// assert_eq!(vec![107, 108, 109], decoded);
    /// ```
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

    /// Default implementation to allow decoding `Option` of `Decodable`.
    /// If decoding fails None is returned.
    ///
    /// # Arguments
    ///
    /// * `value` - the value to decode into `Option`
    ///
    /// # Example
    ///
    /// ```
    /// use crate::bencode::{BencodeValue, Decodable};
    ///
    /// let input = BencodeValue::Integer(108);
    /// let result = Option::<i32>::decode(&input).unwrap();
    ///
    /// assert_eq!(Some(108), result);
    /// ```
    fn decode(value: &BencodeValue) -> Result<Self::Output> {
        match T::decode(value) {
            Ok(value) => Ok(Some(value)),
            Err(_) => Ok(None),
        }
    }

    /// Modified decoding of `Option` value, unlike default implementation this
    /// will return a `None` instead of fail decoding.
    ///
    /// # Arguments
    ///
    /// * `value` - the optional value to decode
    ///
    /// # Example
    ///
    /// ```
    /// use crate::bencode::{BencodeValue, Decodable};
    ///
    /// let input = Some(&BencodeValue::Integer(108));
    /// let result = Option::<i32>::decode_option(input).unwrap();
    ///
    /// assert_eq!(Some(108), result);
    /// ```
    fn decode_option(value: Option<&BencodeValue>) -> Result<Self::Output> {
        match value {
            Some(value) => Self::decode(value),
            None => Ok(None),
        }
    }
}

impl Decodable for String {
    type Output = String;

    /// Default implementation for decoding a `String`.
    ///
    /// # Arguments
    ///
    /// * `value` - BencodeValue to decode to `String`
    ///
    /// # Example
    ///
    /// ```
    /// use crate::bencode::{BencodeValue, Decodable};
    ///
    /// let input = BencodeValue::String(String::from("test"));
    /// let result = String::decode(&input).unwrap();
    ///
    /// assert_eq!("test", result);
    /// ```
    fn decode(value: &BencodeValue) -> Result<String> {
        if let BencodeValue::String(string) = value {
            Ok(string.to_string())
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Not a valid string"))
        }
    }
}

/// Used to decode a BencodeValue to a struct that implements Decodable.
/// The argument must be a `BencodeValue::Dictionary`.
///
/// # Arguments
///
/// * `value` - value to decode from, must be a Dictionary
/// * `name` - name of the field being decoded
pub fn decode<T: Decodable>(value: &BencodeValue, name: &str) -> Result<T::Output> {
    match value {
        BencodeValue::Dictionary(map) => T::decode_option(map.get(name)),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "The structure didn't match",
        )),
    }
}
