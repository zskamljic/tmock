//! # rand
//!
//! Simple and minimalistic OS-dependent random generator.
//!
//! This implementation only supports unix based systems, where
//! /dev/urandom is present on the filesystem.
use std::fs::File;
use std::io::prelude::*;

/// Fills the passed slice with random bytes.
pub fn bytes(into: &mut [u8]) {
    let mut file = File::open("/dev/urandom").expect("urandom not found on the filesystem");

    file.read_exact(into).expect("not enough data in urandom");
}

/// Generates a random value between min and max.
pub fn random_usize(min: usize, max: usize) -> usize {
    let mut data = [0u8; 8];
    bytes(&mut data[..]);

    let value = usize::from_be_bytes(data);

    value % (max - min) + min
}
