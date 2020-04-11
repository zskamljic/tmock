use rand;

/// Generates a random hex key of 4 bytes
pub fn generate_i32_hex_key() -> String {
    let mut key_id = [0u8; 4];
    rand::bytes(&mut key_id);
    let value = i32::from_be_bytes(key_id);
    format!("{:x}", value & 0x7FFF_FFFF)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_i32_key_returns_positive() {
        let result = generate_i32_hex_key();

        assert!(!result.starts_with("-"));
        println!("Result: {}", result);
    }
}
