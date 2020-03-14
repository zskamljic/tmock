use bencode::BencodeValue;
use std::io::Error;
use std::io::ErrorKind;

struct Torrent {
    announce: String,
    info: Info,
}

struct Info {
    name: String,
    piece_length: usize,
    pieces: Vec<u8>,
    length: Option<usize>,
    files: Option<Vec<File>>,
}

struct File {
    length: usize,
    path: Vec<String>,
}

impl Torrent {
    fn from_bencode(value: &BencodeValue) -> Result<Torrent, Error> {
        if let BencodeValue::Dictionary(dictionary) = value {
            let announce = match dictionary.get("announce") {
                Some(BencodeValue::String(url)) => url,
                _ => return Err(Error::new(ErrorKind::InvalidData, "Announce URL missing")),
            };
            let info = match dictionary.get("info") {
                Some(value) => value,
                None => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Info dictionary missing",
                    ))
                }
            };
            Ok(Torrent {
                announce: announce.to_string(),
                info: Info::from_bencode(info)?,
            })
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "Torrent file root was not a dictionary",
            ))
        }
    }
}

impl Info {
    fn from_bencode(value: &BencodeValue) -> Result<Info, Error> {
        if let BencodeValue::Dictionary(dictionary) = value {
            let name = match dictionary.get("name") {
                Some(BencodeValue::String(value)) => value,
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Info name was not present or not a string",
                    ))
                }
            };
            let piece_length = match dictionary.get("piece_length") {
                Some(BencodeValue::Integer(value)) => value,
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Info piece length was not an integer",
                    ))
                }
            };
            let pieces = match dictionary.get("pieces") {
                Some(BencodeValue::String(value)) => value,
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Info pieces was not a string",
                    ))
                }
            };
            let length = match dictionary.get("length") {
                Some(BencodeValue::Integer(value)) => Some(*value as usize),
                _ => None,
            };
            let files = match dictionary.get("files") {
                Some(BencodeValue::List(value)) => Some(as_files(value)?),
                _ => None,
            };
            Ok(Info {
                name: name.to_string(),
                piece_length: *piece_length as usize,
                pieces: pieces.to_string().into_bytes(),
                length,
                files,
            })
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "Info field was not a dictionary",
            ))
        }
    }
}

fn as_files(list: &Vec<BencodeValue>) -> Result<Vec<File>, Error> {
    let mut files = Vec::new();

    for value in list {
        files.push(File::from_bencode(value)?)
    }

    Ok(files)
}

impl File {
    fn from_bencode(value: &BencodeValue) -> Result<File, Error> {
        if let BencodeValue::Dictionary(dictionary) = value {
            let length = match dictionary.get("length") {
                Some(BencodeValue::Integer(value)) => value,
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Length of file was not an integer",
                    ))
                }
            };
            let path = match dictionary.get("path") {
                Some(BencodeValue::List(values)) => as_string_vector(values)?,
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Path of file was not a vector",
                    ))
                }
            };
            Ok(File {
                length: *length as usize,
                path,
            })
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "Files must be dictionaries",
            ))
        }
    }
}

fn as_string_vector(list: &Vec<BencodeValue>) -> Result<Vec<String>, Error> {
    let mut result = Vec::new();

    for item in list {
        match item {
            BencodeValue::String(value) => result.push(value.to_string()),
            _ => return Err(Error::new(ErrorKind::InvalidData, "Expected string vector")),
        }
    }

    Ok(result)
}
