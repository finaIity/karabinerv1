use std::fs;
use std::io;
use crate::userkey::decrypt;

pub fn read_decrypt(filepath: &str, key: &[u8]) -> io::Result<String> {
    let encrypted_data = fs::read(filepath)?;
    let decrypted_data = decrypt(&encrypted_data, key);
    Ok(decrypted_data)
}