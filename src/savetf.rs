use std::{fs, io};
use std::path::Path;
use chrono::Local;

pub fn savetf(filename: &str, password: &str) -> io::Result<()> {
    // Get the current date
    let date = Local::now().format("%Y-%m-%d").to_string();
    let dir_path = format!("passwords/{}", date);

    // Create the directory if it doesn't exist
    fs::create_dir_all(&dir_path)?;

    // Create the full file path
    let file_path = Path::new(&dir_path).join(filename);

    // Write the password to the file
    fs::write(file_path, password)?;

    Ok(())
}