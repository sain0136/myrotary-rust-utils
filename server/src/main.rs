use chrono::Local;
use inotify::{Inotify, WatchMask};
use std::error::Error;
use std::fs;
use std::io::stdin;
use std::path::Path;
use std::thread;
use std::{env, process};
pub mod http_server;
use std::sync::{Arc, Mutex};
// use std::time::{Duration, Instant};

fn main() {
    clear_screen();
    thread::spawn(|| {
        // Start the HTTP server
        match http_server::run() {
            Ok(_) => println!("HTTP server started successfully"),
            Err(e) => panic!("Failed to start HTTP server: {}", e),
        }
    });

    let mut inotify = Inotify::init().expect("Failed to initialize inotify");

    let env_path: String = match load_env("LOG_FILEPATH") {
        Ok(path) => path,
        Err(e) => {
            println!("Failed to load LOG_FILEPATH env with error {:?}", e);
            println!("Exiting...");
            panic!();
        }
    };

    let file_watch = inotify
        .add_watch(&env_path, WatchMask::MODIFY)
        .expect("Failed to add file watch");
    println!("Begin watching log file: {}", env_path);

    let folder_watch = inotify
        .add_watch(
            "/tmp/rust",
            WatchMask::CREATE | WatchMask::DELETE | WatchMask::MODIFY,
        )
        .expect("Failed to add folder watch");
    println!("Begin watching folder: /tmp/rust");

    let mut buffer = [0; 1024];
    let restart = Arc::new(Mutex::new(false));
    loop {
        let restart_status = restart.lock().unwrap();
        if *restart_status {
            break;
        }
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Error reading events");

        let restart = Arc::new(Mutex::new(false));
        // let mut last_modified = Instant::now();
        // let mut first = true;
        for (index, event) in events.enumerate() {
            println!("\nProcessing event number {}", index);
            // if index > 0 {
            //     println!("Skipping event number {}", index - 1);
            //     continue;
            // }
            if event.wd == file_watch && event.mask.contains(inotify::EventMask::MODIFY) {
                clear_screen();

                // let now = Instant::now();
                // println!("\n");cd
                // if !first && now - last_modified < Duration::from_millis(1000) {
                //     println!("Skipping log file modification");
                //     continue;
                // } else if !first && now - last_modified >= Duration::from_millis(1000) {
                //     println!("Log File Modified (First Event)");
                //     last_modified = now;
                // }

                // first = false;

                let metadata = match fs::metadata(&env_path) {
                    Ok(metadata) => metadata,
                    Err(e) => {
                        eprintln!("Failed to get metadata: {}", e);
                        continue;
                    }
                };
                let file_size = metadata.len();
                println!("Log File Modified. Log Files Size: {} bytes", file_size);

                thread::spawn({
                    let env_path_clone = env_path.clone();
                    let restart_clone = Arc::clone(&restart);
                    // Create a clone of env_path to share with the thread otherwise it will be moved and cannot be used
                    // Create a new thread to handle the log rotation
                    move || {
                        let mut input_line = String::new();
                        println!("Would you like to roll over logs? (Y/y or N/n)");
                        match stdin().read_line(&mut input_line) {
                            Ok(_) => {
                                if input_line.trim() == "y" || input_line.trim() == "Y" {
                                    println!("Rolling Logs!");
                                    rotate_logs(&env_path_clone);
                                    let mut restart = restart_clone.lock().unwrap();
                                    *restart = true;
                                    println!("Rolling Logs Complete!");
                                    process::exit(0);
                                } else {
                                    println!(
                                        "Not Rolling Logs! Continuing to watch logs and folders..."
                                    );
                                }
                            }
                            Err(_) => {
                                eprintln!("Failed to read line");
                            }
                        }
                    }
                });
            } else if event.wd == folder_watch {
                if event.mask.contains(inotify::EventMask::CREATE) {
                    println!("File created in /tmp/rust: {:?}", event.name);
                } else if event.mask.contains(inotify::EventMask::DELETE) {
                    println!("File deleted from /tmp/rust: {:?}", event.name);
                } else if event.mask.contains(inotify::EventMask::MODIFY) {
                    println!("File modified in /tmp/rust: {:?}", event.name);
                }
            }
            // println!("Here")
        }
    }
    main();
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
        let timestamp = Local::now().format("%Y_%m_%d_%H%M%S").to_string();
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

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
