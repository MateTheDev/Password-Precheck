#![allow(unused)]
use sha2::{Sha256, Digest};
use hex;

pub fn hash(input_str: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input_str);
    let result = hasher.finalize();
    let result_str: String = hex::encode(result);
    return result_str
}

pub fn comparison(hash1: &str, hash2: &str) -> bool {
    let mut matching: bool = false;
    if hash1 == hash2 {
        matching = true;
    } else {
        matching = false;
    }
    return matching
}