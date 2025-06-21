use std::{fs, io};
use std::path::Path;
use chrono::Local;

pub fn savetf(filename: &str, password: &[u8]) -> io::Result<()> {
    // Get the current date
    let date = Local::now().format("%Y-%m-%d").to_string();
    let dir_path = format!("passwords/{}", date);

    // creates dir if it doesn't exist
    fs::create_dir_all(&dir_path)?;

    // full file path
    let file_path = Path::new(&dir_path).join(filename);

    // writes passw to file
    fs::write(file_path, password)?;

    Ok(())
}