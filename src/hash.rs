use sha2::{Digest, Sha256};

pub fn hash_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key);
    format!("{:x}", hasher.finalize())
}