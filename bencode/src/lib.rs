mod decode;

pub use decode::decode;
pub use decode::Decodable;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

#[derive(PartialEq)]
pub enum BencodeValue {
    Integer(i64),
    String(String),
    List(Vec<BencodeValue>),
    Dictionary(HashMap<String, BencodeValue>),
}

impl Display for BencodeValue {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            BencodeValue::Integer(value) => write!(formatter, "{}", value),
            BencodeValue::String(value) => write!(formatter, "{}", value),
            BencodeValue::List(value) => {
                let values = value
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>();
                write!(formatter, "[{}]", values.join(", "))?;
                Ok(())
            }
            BencodeValue::Dictionary(value) => {
                let values = value
                    .iter()
                    .map(|x| format!("\t{}: {}", x.0, x.1))
                    .collect::<Vec<String>>();
                write!(formatter, "{{\n{}\n}}", values.join(",\n"))
            }
        }
    }
}

pub fn load(file_name: &str) -> Result<BencodeValue> {
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

    let mut buffer = [0u8; 1];
    let mut resulting_string = String::new();

    for _ in 0..string_length {
        reader.read_exact(&mut buffer)?;

        resulting_string.push(buffer[0] as char);
    }

    Ok(BencodeValue::String(resulting_string))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_bencoded() {
        load("../torrents/archlinux-2020.02.01-x86_64.iso.torrent").unwrap();
    }

    #[test]
    fn prints_formatted() -> Result<()> {
        let mut input = "d3:bar4:spam3:fooi42e3:keyl1:a1:b1:c1:dee".as_bytes();
        let value = read(&mut input)?;

        println!("{}", value);

        Ok(())
    }

    #[test]
    fn read_integer_fails_non_numeric() {
        let mut input = "a".as_bytes();

        assert!(read_integer(&mut input).is_err());
    }

    #[test]
    fn read_integer_fails_without_terminator() {
        let mut input = "108".as_bytes();

        assert!(read_integer(&mut input).is_err());
    }

    #[test]
    fn read_integer_reads_integer() -> Result<()> {
        let mut input = "108e".as_bytes();

        let result = read_integer(&mut input)?;
        if let BencodeValue::Integer(value) = result {
            assert_eq!(108, value);
        } else {
            panic!("Value read was not an integer");
        }
        Ok(())
    }
    #[test]
    fn read_integer_reads_negative_integer() -> Result<()> {
        let mut input = "-108e".as_bytes();

        let result = read_integer(&mut input)?;
        if let BencodeValue::Integer(value) = result {
            assert_eq!(-108, value);
        } else {
            panic!("Value read was not an integer");
        }
        Ok(())
    }

    #[test]
    fn read_string_fails_without_colon() {
        let mut input = "abc".as_bytes();

        assert!(read_string(&mut input, b'3').is_err());
    }

    #[test]
    fn read_string_fails_with_eof() {
        let mut input = ":a".as_bytes();

        assert!(read_string(&mut input, b'3').is_err());
    }

    #[test]
    fn read_string_suceeds_double_digit() -> Result<()> {
        let mut input = "6:0123456789ABCDEF".as_bytes();

        let result = read_string(&mut input, b'1')?;
        if let BencodeValue::String(value) = result {
            assert_eq!("0123456789ABCDEF", value);
        } else {
            panic!("Value read was not a string");
        }
        Ok(())
    }

    #[test]
    fn read_string_succeeds_single_digit() -> Result<()> {
        let mut input = ":abc".as_bytes();

        let result = read_string(&mut input, b'3')?;
        if let BencodeValue::String(value) = result {
            assert_eq!("abc", value);
        } else {
            panic!("Value read was not a string");
        }
        Ok(())
    }

    #[test]
    fn read_list_loads_list() -> Result<()> {
        let mut input = "4:spami42ee".as_bytes();

        let result = read_list(&mut input)?;
        if let BencodeValue::List(mut value) = result {
            assert_eq!(2, value.len());
            if let BencodeValue::String(string) = value.remove(0) {
                assert_eq!("spam", string);
            } else {
                panic!("Value was not a string");
            }

            if let BencodeValue::Integer(integer) = value.remove(0) {
                assert_eq!(42, integer);
            } else {
                panic!("Value was not an integer");
            }
        } else {
            panic!("Value read was not a list");
        }
        Ok(())
    }

    #[test]
    fn read_list_loads_empty() -> Result<()> {
        let mut input = "e".as_bytes();

        let result = read_list(&mut input)?;
        if let BencodeValue::List(value) = result {
            assert_eq!(0, value.len());
        } else {
            panic!("Value was not a list");
        }

        Ok(())
    }

    #[test]
    fn read_list_fails_on_unknown() {
        let mut input = "f".as_bytes();

        assert!(read_list(&mut input).is_err());
    }

    #[test]
    fn read_dictionary_suceeds_with_valid() -> Result<()> {
        let mut input = "3:bar4:spam3:fooi42ee".as_bytes();

        let result = read_dictionary(&mut input)?;
        if let BencodeValue::Dictionary(map) = result {
            assert_eq!(2, map.len());

            let value = &map["bar"];
            if let BencodeValue::String(value) = value {
                assert_eq!("spam", value);
            } else {
                panic!("Value for bar was not a string");
            }

            let value = &map["foo"];
            if let BencodeValue::Integer(value) = value {
                assert_eq!(&42, value);
            } else {
                panic!("Value for foo was not an integer");
            }
        } else {
            panic!("Value not a dictionary");
        }

        Ok(())
    }

    #[test]
    fn read_dictionary_fails_with_non_string_keys() {
        let mut input = "di5e:spam3:fooi42ee".as_bytes();

        assert!(read_dictionary(&mut input).is_err());
    }
}
