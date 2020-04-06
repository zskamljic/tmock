use std::fs::File;
use std::io::prelude::*;

pub fn bytes(into: &mut [u8]) {
    let mut file = File::open("/dev/urandom").expect("urandom not found on the filesystem");

    file.read_exact(into).expect("not enough data in urandom");
}

pub fn random_usize(min: usize, max: usize) -> usize {
    let mut data = [0u8; 8];
    bytes(&mut data[..]);

    let value = usize::from_be_bytes(data);

    value % (max - min) + min
}
