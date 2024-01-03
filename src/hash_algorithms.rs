use sha2::{Digest, Sha256};

pub fn sha256_hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    let hex_string = result.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();

    return hex_string
}
