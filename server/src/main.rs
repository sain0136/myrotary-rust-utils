use chrono::Local;
use inotify::{Inotify, WatchMask};
use std::env;
use std::error::Error;
use std::fs;
use std::io::stdin;
use std::path::Path;
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

    inotify
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
                let metadata = match fs::metadata(&env_path) {
                    Ok(metadata) => metadata,
                    Err(e) => {
                        eprintln!("Failed to get metadata: {}", e);
                        continue;
                    }
                };
                let file_size = metadata.len();
                println!("Log Files Size: {}", file_size);
                thread::spawn({
                    let env_path_clone = env_path.clone();
                    // Create a clone of env_path to share with the thread otherwise it will be moved and cannot be used
                    // Create a new thread to handle the log rotation
                    move || {
                        let mut input_line = String::new();
                        println!("Would you like to roll over logs? (Y/y)");
                        match stdin().read_line(&mut input_line) {
                            Ok(_) => {
                                if input_line.trim() == "y" || input_line.trim() == "Y" {
                                    println!("Rolling Logs!");
                                    rotate_logs(&env_path_clone);
                                } else {
                                    println!("Watching log file!")
                                }
                            }
                            Err(_) => {
                                eprintln!("Failed to read line");
                            }
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

fn rotate_logs(path: &str) {
    let file_path = Path::new(path);
    if let Some(parent_dir) = file_path.parent() {
        // Create the "archive" folder in the parent directory if it doesn't exist
        let archive_dir = parent_dir.join("archive"); // Add the "archive" directory to the parent directory
        if !archive_dir.exists() {
            fs::create_dir(&archive_dir).expect("Failed to create archive directory");
            println!("Created archive directory: {:?}", archive_dir);
        }
        //Rename the original file with a timestamp and move it to the archive
        let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
        let file_stem = file_path.file_stem().unwrap().to_str().unwrap();
        let extension = file_path.extension().unwrap_or_default().to_str().unwrap();
        let new_file_name = format!("{}-{}.{}", file_stem, timestamp, extension);
        let new_file_path = archive_dir.join(new_file_name);
        match fs::rename(&file_path, &new_file_path) {
            Ok(_) => println!("Moved file to archive: {:?}", new_file_path),
            Err(e) => eprintln!("Failed to move file: {}", e),
        }
        //Create a new file with the same name as the original file
        match fs::File::create(file_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to create new file: {}", e);
                return;
            }
        };
        println!("Created new file: {:?}", file_path);
    } else {
        eprintln!("Could not determine the parent directory for the provided file path.");
    }
}


