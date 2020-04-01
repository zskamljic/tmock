use rand;

const TRANSMISSION_CHAR_POOL: [u8; 36] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f',
    b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
    b'w', b'x', b'y', b'z',
];

pub fn generate_transmission_294_id() -> String {
    // 20 - "-TR2940-".len() - 1 (checksum)
    let mut random_suffix = [0u8; 11];
    rand::bytes(&mut random_suffix);
    let base = TRANSMISSION_CHAR_POOL.len() as u8;

    let mut result = "-TR2940-".to_string();
    let mut sum = 0;
    for i in 0..random_suffix.len() {
        random_suffix[i] %= base;
        sum += random_suffix[i];
        result.push(TRANSMISSION_CHAR_POOL[random_suffix[i] as usize] as char)
    }

    let checksum = base - sum % base;
    result.push(TRANSMISSION_CHAR_POOL[checksum as usize] as char);

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_transmission_id() {
        println!("{}", generate_transmission_294_id());
    }
}
