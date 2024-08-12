use inotify::{Inotify, WatchMask};
use std::fs::File;
use std::io::Read;

fn main() {
    let mut inotify = Inotify::init().expect("Failed to initialize inotify");

    let watch_descriptor = inotify
        .add_watch("/tmp/testfile", WatchMask::MODIFY)
        .expect("Failed to add file watch");

    let mut buffer = [0; 1024];

    loop {
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Error reading events");

        for event in events {
            if event.mask.contains(inotify::EventMask::MODIFY) {
                println!("File modified!");
            }
        }
    }

    inotify
        .rm_watch(watch_descriptor)
        .expect("Failed to remove watch");
}
