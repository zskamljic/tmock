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

struct Sha1 {
    h: [u32; 5],
    message_length: u32,
    pending: Vec<u8>,
}

impl Sha1 {
    fn new() -> Sha1 {
        Sha1 {
            h: [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0],
            message_length: 0,
            pending: Vec::new(),
        }
    }

    fn update_str(&mut self, data: &str) {
        self.update(data.as_bytes())
    }

    fn update(&mut self, data: &[u8]) {
        self.pending.extend(data.iter().cloned());
        self.consume_pending();
    }

    fn process_chunk(&mut self, chunk: Vec<u8>) {
        let mut words = [0u32; 80];
        fill_start(&mut words, chunk);
        extend(&mut words);

        let mut a = self.h[0];
        let mut b = self.h[1];
        let mut c = self.h[2];
        let mut d = self.h[3];
        let mut e = self.h[4];

        for i in 0..=79 {
            let (f, k) = if i <= 19 {
                ((b & c) | ((!b) & d), 0x5A827999)
            } else if i <= 39 {
                (b ^ c ^ d, 0x6ED9EBA1)
            } else if i <= 59 {
                ((b & c) | (b & d) | (c & d), 0x8F1BBCDCu32)
            } else {
                (b ^ c ^ d, 0xCA62C1D6u32)
            };

            let temp = add_with_mask!(left_rotate(a, 5), f, e, k, words[i]);
            //let temp = left_rotate(a, 5) + f + e + k + words[i];
            e = d;
            d = c;
            c = left_rotate(b, 30);
            b = a;
            a = temp;
        }

        self.h[0] = add_with_mask!(self.h[0], a);
        self.h[1] = add_with_mask!(self.h[1], b);
        self.h[2] = add_with_mask!(self.h[2], c);
        self.h[3] = add_with_mask!(self.h[3], d);
        self.h[4] = add_with_mask!(self.h[4], e);
    }

    fn consume_pending(&mut self) {
        while self.pending.len() >= 64 {
            let chunk = self.pending.drain(0..64).collect();
            self.process_chunk(chunk);
            self.message_length += 64;
        }
    }

    fn digest(&mut self) {
        let message_byte_len = self.message_length as usize + self.pending.len();
        self.pending.push(0x80);

        let pad_length = (56 - (message_byte_len + 1) % 64) % 64;
        for _ in 0..pad_length {
            self.pending.push(0x00);
        }

        let message_bit_len = message_byte_len as u64 * 8u64;
        self.pending.extend(message_bit_len.to_be_bytes().iter());
        self.consume_pending();
    }

    fn hex_digest(&mut self) -> String {
        self.digest();

        self.h
            .iter()
            .map(|value| format!("{:08x}", value))
            .collect::<Vec<String>>()
            .join("")
    }
}

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

fn extend(words: &mut [u32; 80]) {
    for i in 16..=79 {
        words[i] = left_rotate(
            words[i - 3] ^ words[i - 8] ^ words[i - 14] ^ words[i - 16],
            1,
        );
    }
}

fn left_rotate(value: u32, bits: usize) -> u32 {
    ((value << bits) | (value >> (32 - bits))) & 0xffffffff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha1_returns_example() {
        let mut sha1 = Sha1::new();
        sha1.update_str("The quick brown fox jumps over the lazy dog");
        let result = sha1.hex_digest();

        assert_eq!("2fd4e1c67a2d28fced849ee1bb76e7391b93eb12", result);
    }

    #[test]
    fn sha1_returns_example2() {
        let mut sha1 = Sha1::new();
        sha1.update_str("The quick brown fox jumps over the lazy cog");
        let result = sha1.hex_digest();

        assert_eq!("de9f2c7fd25e1b3afad3e85a0bd17d9b100db4b3", result);
    }
    #[test]
    fn sha1_returns_example3() {
        let mut sha1 = Sha1::new();
        sha1.update_str("");
        let result = sha1.hex_digest();

        assert_eq!("da39a3ee5e6b4b0d3255bfef95601890afd80709", result);
    }
}
