use rand::prelude::*;
use std::fs;
use std::io::{self, Write};
use std::process::Command;
use crate::savetf::savetf;
use crate::hash::hash_key;
use crate::userkey::encrypt;
use crate::file_access::read_decrypt;
use sha2::{Sha256, Digest};
use rpassword::read_password;

mod file_access;
mod savetf;
mod userkey;
mod hash;

const USER_KEY_FILE: &str = "safe/user_key.txt";

fn save_user_key(key: &str) -> io::Result<()> {
    let hashed_key = hash_key(key);
    fs::create_dir_all("safe")?;
    fs::write(USER_KEY_FILE, hashed_key)
}

fn load_user_key() -> io::Result<String> {
    let key = fs::read_to_string(USER_KEY_FILE)?;
    Ok(key.trim().to_string())
}

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

    let personal_key = if fs::metadata(USER_KEY_FILE).is_ok() {
        print!("Enter your personal key: ");
        io::stdout().flush().unwrap();
        let key = rpassword::read_password().expect("Failed to read input");
        let key = key.trim().to_string();

        //verify key with stored key
        let stored_key = load_user_key()?;
        if hash_key(&key) == stored_key {
            key
        } else {
            eprintln!("Invalid personal key.");
            return Ok(());
        }
    } else {
        print!("Set your personal key: ");
        io::stdout().flush().unwrap();
        let key = rpassword::read_password().expect("Failed to read input");
        let key = key.trim().to_string();
        save_user_key(&key)?;
        key
    };

    loop {
        println!("Choose an option:");
        println!("1. Generate a new password");
        println!("2. Decrypt a password from a file");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => {
                let mut length = String::new();
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
                            let encrypted_password = encrypt(&password, personal_key.as_bytes());
                            let _ = savetf(&filename, &encrypted_password);
                            println!("Password saved to file {}", filename);
                        }

                        let mut copy_to_clipboard = String::new();
                        print!("Do you want to copy the password to clipboard? Y/N: ");
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut copy_to_clipboard).expect("Failed to read input");

                        if copy_to_clipboard.trim().to_uppercase() == "Y" {
                            if clipboard_copy(&password).is_ok() {
                                println!("Password copied to clipboard.");
                            } else {
                                    eprintln!("Error: Couldn't copy password to clipboard.");
                                }
                            }
                        } else {
                            println!("Please enter a length between 7 and 25.");
                        }
                    } else {
                        println!("Invalid input. Please enter a number.");
                    }
                }
                "2" => {
                    let mut filepath = String::new();
                    print!("Enter the file path to decrypt (passwords/date/filename): ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut filepath).expect("Failed to read input");
                    let filepath = filepath.trim();

                    match read_decrypt(filepath, personal_key.as_bytes()) {
                        Ok(decrypted_content) => {
                            if clipboard_copy(&decrypted_content).is_ok() {
                                println!("Password copied to clipboard.");
                            } else {
                                eprintln!("Error: Couldn't copy password to clipboard.");
                            }
                        }
                    Err(e) => eprintln!("Failed to read and decrypt file: {}", e),
                }
            }
            "3" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please enter 1, 2 or 3."),
        }
    }
    Ok(())
}