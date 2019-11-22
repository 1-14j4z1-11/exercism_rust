extern crate rand;
extern crate regex;

use rand::Rng;
use regex::Regex;

pub fn encode(key: &str, s: &str) -> Option<String> {
    if !is_valid_key(key) {
        return None;
    }

    let mut encoded = String::new();

    for i in 0..s.len() {
        let c = s.chars().nth(i).unwrap();
        let k = key.chars().nth(i % key.len()).unwrap();
        encoded.push(rotate_char(c, k));
    }

    Some(encoded)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if !is_valid_key(key) {
        return None;
    }

    let mut decoded = String::new();

    for i in 0..s.len() {
        let c = s.chars().nth(i).unwrap();
        let k = key.chars().nth(i % key.len()).unwrap();
        decoded.push(reverse_rotate_char(c, k));
    }

    Some(decoded)
}

pub fn encode_random(s: &str) -> (String, String) {
    let key = create_random_key(100);
    let encoded = encode(&key, s).unwrap();
    (key, encoded)
}

fn is_valid_key(key: &str) -> bool {
    let key_regex = Regex::new(r"^[a-z]+$").unwrap();
    key_regex.is_match(key)
}

fn create_random_key(len: usize) -> String {
    let mut key = String::new();
    let mut rng = rand::thread_rng();

    for _ in 0..len {
        let c = rng.gen_range(0x61u8, 0x7Bu8) as char;
        key.push(c);
    }

    key
}

const LOWER_A_CODE: u8 = 0x61;
const ALPHABET_COUNT: u8 = 26;

fn rotate_char(c: char, key: char) -> char {
    let c_index = c as u8 - LOWER_A_CODE;
    let key_index = key as u8 - LOWER_A_CODE;

    (((c_index + key_index) % ALPHABET_COUNT) + LOWER_A_CODE) as char
}

fn reverse_rotate_char(c: char, key: char) -> char {
    let c_index = c as u8 - LOWER_A_CODE;
    let key_index = key as u8 - LOWER_A_CODE;

    (((c_index + ALPHABET_COUNT - key_index) % ALPHABET_COUNT) + LOWER_A_CODE) as char
}
