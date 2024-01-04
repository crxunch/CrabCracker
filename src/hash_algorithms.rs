use sha2::{Digest, Sha256, Sha512};
use md5;

pub fn sha256_hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    let hex_string = result.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();

    return hex_string
}

pub fn sha512_hash(input: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    let hex_string = result.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();

    return hex_string
}

pub fn md5_hash(input: &str) -> String {
    let mut hasher = md5::Md5::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    let hex_string = result.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();

    hex_string
}
