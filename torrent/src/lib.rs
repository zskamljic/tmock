use bencode::{BencodeValue, Decodable};
use bencode_derive::Decodable;

#[derive(Decodable)]
pub struct Torrent {
    announce: String,
    info: Info,
}

#[derive(Decodable)]
pub struct Info {
    name: String,
    piece_length: usize,
    pieces: String,
    length: Option<usize>,
    files: Option<Vec<File>>,
}

#[derive(Decodable)]
pub struct File {
    length: usize,
    path: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Result;

    #[test]
    fn decode_file_succeds() -> Result<()> {
        let mut input = "d6:lengthi5e4:pathl1:a1:bee".as_bytes();
        let file = File::read(&mut input)?;

        assert_eq!(5, file.length);
        assert_eq!(2, file.path.len());
        assert_eq!(vec!["a".to_string(), "b".to_string()], file.path);

        Ok(())
    }

    #[test]
    fn decode_fn_succeeds_string() -> Result<()> {
        let value: String = String::decode(&BencodeValue::String("asdf".to_string()))?;

        assert_eq!("asdf", value);
        Ok(())
    }

    #[test]
    fn decode_info_succeeds() -> Result<()> {
        let mut input = "d4:name4:name12:piece_lengthi1e6:pieces1:56:lengthi11ee".as_bytes();
        let info = Info::read(&mut input)?;

        assert_eq!("name", info.name);
        assert_eq!(1, info.piece_length);
        assert_eq!(1, info.pieces.len());
        assert_eq!("5", info.pieces);
        assert!(info.length.is_some());
        assert_eq!(Some(11), info.length);

        Ok(())
    }
}
