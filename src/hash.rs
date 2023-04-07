use sha2::{Digest, Sha256};

pub fn hash_pw(password: &str) -> String {
    format!("{:x}", Sha256::digest(password.as_bytes()))
}