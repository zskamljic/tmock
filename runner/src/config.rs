use std::collections::HashMap;
use std::fs;

const MIN_VALUE: &str = "MIN_UPLOAD";
const MAX_VALUE: &str = "MAX_UPLOAD";

pub struct Config {
    pub min: usize,
    pub max: usize,
}

impl Config {
    pub fn new() -> Result<Config, String> {
        let map = read_to_map()?;

        let min = load_and_parse(&map, MIN_VALUE)?;
        let max = load_and_parse(&map, MAX_VALUE)?;

        Ok(Config { min, max })
    }
}

fn read_to_map() -> Result<HashMap<String, String>, String> {
    if let Ok(value) = fs::read_to_string("config.txt") {
        Ok(parse_to_map(value))
    } else {
        Err("config.txt missing or not readable".to_string())
    }
}

fn parse_to_map(value: String) -> HashMap<String, String> {
    let mut result = HashMap::new();

    for line in value.lines() {
        let mut parts = line.split('=');
        let key = match parts.next() {
            Some(value) => value,
            None => continue,
        };
        let value = match parts.next() {
            Some(value) => value,
            None => continue,
        };

        result.insert(key.to_string(), value.to_string());
    }

    result
}

fn load_and_parse(map: &HashMap<String, String>, key: &str) -> Result<usize, String> {
    if let Some(value) = map.get(key) {
        if let Ok(value) = value.parse() {
            Ok(value)
        } else {
            Err(format!("Value for {} must be a positive integer.", key))
        }
    } else {
        Err(format!("Value for {} was not present.", key))
    }
}
