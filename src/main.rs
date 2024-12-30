use rand::prelude::*;
use std::io::{self, Write};
use std::process::Command;
use crate::savetf::savetf;
use crate::userkey::{encrypt, decrypt};

mod savetf;
mod userkey;

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

fn clipboard_copy(text: &str) -> io::Result<()> {
    let mut cmd = Command::new("pbcopy")
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
        eprintln!("Failed to execute pbcopy: {}", output.status);
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to execute pbcopy"));
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

fn main() -> io::Result<()> {
    println!("Welcome to Karabiner!");

    let mut length = String::new();
    let mut personal_key = String::new();

    print!("Enter your personal key: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut personal_key).expect("Failed to read input");
    let personal_key = personal_key.trim().as_bytes();

    loop {
        print!("Enter the desired password length (7-25): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut length).expect("Failed to read input");

        if let Ok(len) = length.trim().parse::<usize>() {
            if len >= 7 && len <= 25 {
                let password = gen_pass(len);
                println!("Password generated.");

                let mut save_password = String::new();
                print!("Do you want to save the password to a file? Y/N: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut save_password).expect("Failed to read input");

                if save_password.trim().to_uppercase() == "Y" {
                    let filename = gen_filename();
                    let encrypted_password = encrypt(&password, personal_key);
                    let _ = savetf(&filename, &encrypted_password);
                    println!("Password saved to file {}", filename);
                }

                let mut copy_to_clipboard = String::new();
                print!("Do you want to copy the password to clipboard? Y/N: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut copy_to_clipboard).expect("Failed to read input");

                if copy_to_clipboard.trim().to_uppercase() == "Y" {
                    let encrypted_password = encrypt(&password, personal_key);
                    let decrypted_password = decrypt(&encrypted_password, personal_key);
                    if clipboard_copy(&decrypted_password).is_ok() {
                        println!("Password copied to clipboard.");
                    } else {
                        eprintln!("Error: Couldn't copy password to clipboard.");
                    }
                }
                break;
            } else {
                println!("Please enter a length between 7 and 25.");
            }
        } else {
            println!("Invalid input. Please enter a number.");
        }
        length.clear();
    }
    Ok(())
}