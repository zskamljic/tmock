#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn load_bencoded() {
        from_file("../torrents/archlinux-2020.02.01-x86_64.iso.torrent").unwrap();
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
