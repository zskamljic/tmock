use crate::Info;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use std::net::TcpStream;
use std::str;

pub fn request_trackers(url: &str, info: &Info) -> Result<()> {
    let (host, path) = get_host_and_path(url)?;
    let mut stream = TcpStream::connect(host)?;

    stream.write(format!("GET {}\r\n\r\n", path).as_bytes())?;

    let mut buffer = [0u8; 512];
    let read = stream.read(&mut buffer)?;

    println!(
        "Read: {}: {}",
        read,
        str::from_utf8(&buffer).expect("wasn't a valid string")
    );

    Ok(())
}

fn get_host_and_path(url: &str) -> Result<(&str, &str)> {
    if !url.starts_with("http://") {
        return Err(Error::new(
            ErrorKind::AddrNotAvailable,
            format!("Schema for {} is not supported", url),
        ));
    }
    let host = &url[7..];
    let path_start = match host.find('/') {
        Some(value) => value,
        None => host.len(),
    };

    let host = &host[..path_start];
    let path = &url[7 + path_start..];
    Ok((host, path))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Result;

    #[test]
    fn get_host_and_path_splits_correct() -> Result<()> {
        let (host, path) = get_host_and_path("http://tracker.archlinux.org:6969/announce")?;

        assert_eq!("tracker.archlinux.org:6969", host);
        assert_eq!("/announce", path);
        Ok(())
    }

    #[test]
    fn fetch_trackers() -> Result<()> {
        request_trackers(
            "http://tracker.archlinux.org:6969/announce",
            &Info {
                name: "".to_string(),
                piece_length: 1,
                pieces: "".to_string(),
                files: None,
                length: None,
            },
        )?;

        Ok(())
    }
}
