use sha2::{Sha256, Digest};

pub fn encrypt(text: &str, key: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(key);
    let hash = hasher.finalize();

    let mut encrypted = Vec::new();
    for (i, byte) in text.bytes().enumerate() {
        encrypted.push(byte ^ hash[i % hash.len()]);
    }
    encrypted
}

pub fn decrypt(ciphertext: &[u8], key: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key);
    let hash = hasher.finalize();

    let mut decrypted = Vec::new();
    for (i, &byte) in ciphertext.iter().enumerate() {
        decrypted.push(byte ^ hash[i % hash.len()]);
    }
    String::from_utf8(decrypted).unwrap()
}