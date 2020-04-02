use rand;

const TRANSMISSION_CHAR_POOL: [u8; 36] = *b"0123456789abcdefghijklmnopqrstuvwxyz";

pub fn generate_transmission_294_id() -> String {
    // 20 - "-TR2940-".len() - 1 (checksum)
    let mut random_suffix = [0u8; 11];
    rand::bytes(&mut random_suffix);
    let base = TRANSMISSION_CHAR_POOL.len() as u8;

    let mut result = "-TR2940-".to_string();
    let mut sum = 0u8;
    for i in 0..random_suffix.len() {
        random_suffix[i] %= base;
        sum = sum.wrapping_add(random_suffix[i]);
        result.push(TRANSMISSION_CHAR_POOL[random_suffix[i] as usize] as char)
    }

    let checksum = base - sum % base;
    result.push(
        TRANSMISSION_CHAR_POOL[(checksum % TRANSMISSION_CHAR_POOL.len() as u8) as usize] as char,
    );

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
