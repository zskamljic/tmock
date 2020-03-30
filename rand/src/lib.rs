use std::fs::File;
use std::io::prelude::*;

pub fn bytes(into: &mut [u8]) {
    let mut file = File::open("/dev/urandom").expect("urandom not found on the filesystem");

    file.read_exact(into).expect("not enough data in urandom");
}
