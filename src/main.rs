use rand::prelude::*;
use std::io::{self, Write};
use std::process::{Command, Stdio};

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
fn main() ->io::Result<()> {
    println!("Welcome to Karabiner !");
    let password = gen_pass(16);
    let mut child = if cfg!(target_os = "macos") {
        Command::new("pbcopy")
    } else if cfg!(target_os = "linux") {
        let mut cmd = Command::new("xclip");
        cmd.arg("-selection").arg("clipboard");
        cmd
    } else {
        panic!("Unsupported OS");
    }.stdin(Stdio::piped())
    .spawn()?;
    child.stdin.as_mut().unwrap().write_all(password.as_bytes())?;
    println!("Generated password & copied to clipboard !");

    Ok(())
}