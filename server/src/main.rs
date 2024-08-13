use inotify::{Inotify, WatchMask};
use std::env;
use std::error::Error;
use std::fs;
use std::io::stdin;
use std::thread;

fn main() {
    let mut inotify = Inotify::init().expect("Failed to initialize inotify");

    let env_path: String = match load_env("LOG_FILEPATH") {
        Ok(path) => path,
        Err(e) => {
            println!("Failed to load LOG_FILEPATH env with error {:?}", e);
            println!("Exiting...");
            panic!();
        }
    };

    let watch_descriptor = inotify
        .add_watch(&env_path, WatchMask::MODIFY)
        .expect("Failed to add file watch");

    let mut buffer = [0; 1024];

    loop {
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Error reading events");

        for event in events {
            if event.mask.contains(inotify::EventMask::MODIFY) {
                println!("Log File Modified");
                let metadata = fs::metadata(env_path)?;
                let file_size = metadata.len();
                println!("Log Files Size: {}", file_size);
                thread::spawn(move || {
                    let mut input_line = String::new();
                    println!("Would you like to roll over logs? (Y/y)");
                    match stdin().read_line(&mut input_line) {
                        Ok(_) => {
                            if input_line.trim() == "y" || input_line.trim() == "Y" {
                                println!("Rolling Logs!")
                            } else {
                                println!("Watching log file!")
                            }
                        }
                        Err(_) => {
                            eprintln!("Failed to read line");
                        }
                    }
                });
            }
        }
    }
}

fn load_env(key: &str) -> Result<String, Box<dyn Error>> {
    dotenvy::dotenv()?;
    let target = env::var(key)?;
    Ok(target)
}
