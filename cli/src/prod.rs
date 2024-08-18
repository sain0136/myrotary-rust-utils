use crate::utils;
use std::io::Write;
use std::time::Duration;
use std::{fs, io, thread};
#[derive(Debug)]
pub struct Production {}

pub fn prod() {
    println!("Tool Initialized");
    let os_mode = utils::terminal::load_env_var("OSMODE");
    let path_to_toml = utils::terminal::load_env_var("PATH_TO_TOML");
    println!("OS Mode: {}", os_mode);
    match fs::metadata(&path_to_toml) {
        Ok(_) => {
            println!("Configuration TOML exists: {}", path_to_toml);
        }
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    println!("File does not exist: {}", path_to_toml);
                }
                io::ErrorKind::PermissionDenied => {
                    println!(
                        "Permission denied when accessing the file: {}",
                        path_to_toml
                    );
                }
                _ => {
                    println!(
                        "Failed to access the file: {} with error: {:?}",
                        path_to_toml, e
                    );
                }
            }
        }
    }
    let handle = thread::spawn(move || {
        print!("Loading");

        for _ in 0..3 {
            print!("...");
            io::stdout().flush().unwrap(); // Flush the output to ensure it appears immediately
            thread::sleep(Duration::from_secs(1)); // Sleep for 1 second
        }
        println!(""); // To move to the next line after loading
    });
    thread::sleep(Duration::from_secs(4));
    handle.join().unwrap();
    println!("Production Test Complete");
    thread::sleep(Duration::from_secs(1));
}
