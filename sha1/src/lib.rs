#[cfg(test)]
mod tests;

/// Utility macro to help with adding with overflows.
macro_rules! add_with_mask {
    ($x:expr) => {
        $x
    };
    ($x:expr, $y:expr) => {
        (($x as u64 + $y as u64) & 0xFFFFFFFF) as u32
    };
    ($x:expr, $y:expr, $($rest:expr),+) => {
        add_with_mask!(add_with_mask!($x, $y), $($rest),*)
    }
}

/// Execute sha1 on a `&str` and returns a `String` of hex encoded data.
pub fn sha1_str_as_str(string: &str) -> String {
    let mut sha = Sha1::new();
    sha.update_str(string);
    sha.hex_digest()
}

/// Execute sha1 on a `&str` and returns an array of 20 bytes.
pub fn sha1_str_as_bytes(string: &str) -> [u8; 20] {
    let mut sha = Sha1::new();
    sha.update_str(string);
    sha.digest()
}

/// Execute sha1 on array of bytes and returns an array of 20 bytes.
pub fn sha1_bytes_as_bytes(bytes: &[u8]) -> [u8; 20] {
    let mut sha = Sha1::new();
    sha.update(bytes);
    sha.digest()
}

/// Sha1 state
#[derive(Default)]
pub struct Sha1 {
    /// the h1...h5 values
    h: [u32; 5],
    /// The total length of the message
    message_length: u32,
    /// Undigested bytes
    pending: Vec<u8>,
}

impl Sha1 {
    /// Creates a new state with default vector
    pub fn new() -> Sha1 {
        Sha1 {
            h: [
                0x6745_2301,
                0xEFCD_AB89,
                0x98BA_DCFE,
                0x1032_5476,
                0xC3D2_E1F0,
            ],
            message_length: 0,
            pending: Vec::new(),
        }
    }

    /// Update the state with string.
    pub fn update_str(&mut self, data: &str) {
        self.update(data.as_bytes())
    }

    /// Update the state with byte array.
    pub fn update(&mut self, data: &[u8]) {
        self.pending.extend(data.iter().cloned());
        self.consume_pending();
    }

    /// Processes a chunk of 64 bytes.
    // The names follow specification, changing them would hinder readability
    #[allow(clippy::many_single_char_names)]
    fn process_chunk(&mut self, chunk: Vec<u8>) {
        let mut words = [0u32; 80];
        fill_start(&mut words, chunk);
        extend(&mut words);

        let mut a = self.h[0];
        let mut b = self.h[1];
        let mut c = self.h[2];
        let mut d = self.h[3];
        let mut e = self.h[4];

        for (i, item) in words.iter().enumerate() {
            let (f, k) = if i <= 19 {
                ((b & c) | ((!b) & d), 0x5A82_7999)
            } else if i <= 39 {
                (b ^ c ^ d, 0x6ED9_EBA1)
            } else if i <= 59 {
                ((b & c) | (b & d) | (c & d), 0x8F1B_BCDC_u32)
            } else {
                (b ^ c ^ d, 0xCA62_C1D6_u32)
            };

            let temp = add_with_mask!(a.rotate_left(5), f, e, k, *item);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        self.h[0] = add_with_mask!(self.h[0], a);
        self.h[1] = add_with_mask!(self.h[1], b);
        self.h[2] = add_with_mask!(self.h[2], c);
        self.h[3] = add_with_mask!(self.h[3], d);
        self.h[4] = add_with_mask!(self.h[4], e);
    }

    /// Consume the pending bytes if their length is at least 64.
    fn consume_pending(&mut self) {
        while self.pending.len() >= 64 {
            let chunk = self.pending.drain(0..64).collect();
            self.process_chunk(chunk);
            self.message_length += 64;
        }
    }

    /// Finalize the hash digest.
    fn digest(&mut self) -> [u8; 20] {
        let message_byte_len = self.message_length as usize + self.pending.len();
        self.pending.push(0x80);

        while self.pending.len() % 64 != 56 {
            self.pending.push(0x00);
        }

        let message_bit_len = message_byte_len as u64 * 8u64;
        self.pending.extend(message_bit_len.to_be_bytes().iter());
        self.consume_pending();

        let mut result = [0u8; 20];
        for (h_index, value) in self.h.iter().enumerate() {
            let bytes = value.to_be_bytes();
            for (byte_index, byte) in bytes.iter().enumerate() {
                result[h_index * bytes.len() + byte_index] = *byte;
            }
        }
        result
    }

    /// Perform the digest and return it as a hex encoded `String`.
    pub fn hex_digest(&mut self) -> String {
        self.digest()
            .iter()
            .map(|value| format!("{:02x}", value))
            .collect::<Vec<String>>()
            .join("")
    }
}

/// Fill the start of the array with chunk bytes.
fn fill_start(words: &mut [u32; 80], chunk: Vec<u8>) {
    for i in 0..=15 {
        let bytes = [
            chunk[i * 4],
            chunk[i * 4 + 1],
            chunk[i * 4 + 2],
            chunk[i * 4 + 3],
        ];
        words[i] = u32::from_be_bytes(bytes);
    }
}

/// Fill the rest of the array with the data from the start
fn extend(words: &mut [u32; 80]) {
    for i in 16..=79 {
        words[i] = (words[i - 3] ^ words[i - 8] ^ words[i - 14] ^ words[i - 16]).rotate_left(1);
    }
}
