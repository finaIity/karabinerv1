use std::io::Write;
use rand::prelude::*;
use std::io::{self};
use std::process::Command;
use crate::hash::hash_pw;
use crate::savetf::savetf;
mod savetf;
mod hash;


fn gen_pass(length: usize) -> String {
    let charset= "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                         abcdefghijklmnopqrstuvwxyz\
                         0123456789!@#$%^&*()_+-=";
   let mut rng = rand::thread_rng();
   (0..length).map(|_| {
        let idx = rng.gen_range(0..charset.len());
        charset.chars().nth(idx).unwrap()
        }).collect()
}

fn clipboard_copy(text: &str) -> io::Result <()> {
    let mut cmd = Command::new("xclip")
    .arg("-selection")
    .arg("clipboard")
    .stdin(std::process::Stdio::piped())
    .spawn()?;

    if let Some(mut stdin) = cmd.stdin.take() {
        stdin.write_all(text.as_bytes())?;
    } else {
        eprintln!("Failed to open stdin");
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to open stdin"));
    }

    let output = cmd.wait_with_output()?;
    if !output.status.success() {
        eprintln!("Failed to execute xclip: {}", output.status);
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to execute xclip"));
    }
    Ok(())
}

fn gen_filename() -> String {
    let mut filename = String::new();
    print!("Enter filename to save password in (This will be hashed): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut filename).unwrap();

    filename.trim().to_string()
}

fn main() ->io::Result<()> {
    println!("Welcome to Karabiner !");
    let password = gen_pass(16);
    println!("Password generated.");

    let mut save_password = String::new();
    print!("Do you want to save the password to a file ? Y/N: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut save_password)
        .expect("Failed to read input");

    if save_password.trim().to_uppercase() == "Y" {
        let filename = gen_filename();
        let hashed = hash_pw(&password);
        savetf(&filename, &hashed);
        println!("Password saved to file {}", filename)
    }

    if clipboard_copy(&password).is_ok() {
        println!("Password copied to clipboard !");
    } else {
        eprintln!("Error: Couldn't copy password to clipboard.")
    }
    Ok(())
}