use inotify::{Inotify, WatchMask};
use std::fs::{self, File};
use std::io::{stdin, Read};
use std::os::windows::fs::MetadataExt;

fn main() {
    let mut inotify = Inotify::init().expect("Failed to initialize inotify");

    const ENV_PATH: String = match load_env("LOG_FILEPATH") {
        Ok(path) => path,
        Err(e) => {
            println!("Failed to load LOG_FILEPATH env with error {:?}", e);
            println!("Exiting...");
            panic!();
        }
    };

    let watch_descriptor = inotify
        .add_watch(ENV_PATH, WatchMask::MODIFY)
        .expect("Failed to add file watch");

    let mut buffer = [0; 1024];

    loop {
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Error reading events");

        for event in events {
            if event.mask.contains(inotify::EventMask::MODIFY) {
                println!("Log File Modified");
                let metadata = fs::metadata(ENV_PATH)?;
                let file_size = metadata.file_size();
                println!("Log Files Size: {}", file_size);
                thread::spawn(move || {
                    let mut input_line = String::new();
                    println!("Would you like to roll over logs? (Y/y)");
                    match stdin().read_line(&mut input_line) {
                        Ok(_) => {
                            if input.trim() == "y" || input.trim() == "Y" {
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

    inotify
        .rm_watch(watch_descriptor)
        .expect("Failed to remove watch");
}

fn load_env(key: &str) -> Result<String, Box<dyn Error>> {
    dotenvy::dotenv()?;
    let target = env::var(key)?;
    Ok(target)
}
