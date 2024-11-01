// src/status.rs

use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::thread;

/// Function to watch for file changes in a directory
pub fn monitor_files(path: &str) {
    // Create a channel to receive the events
    let (tx, rx) = channel();

    // Create a watcher object that sends events to the channel
    let mut watcher = watcher(tx, Duration::from_secs(2)).expect("Failed to create watcher");

    // Add the path to be watched (with recursive mode)
    watcher
        .watch(path, RecursiveMode::Recursive)
        .expect("Failed to watch path");

    println!("Monitoring file changes in directory: {}", path);

    // Start a thread to handle events asynchronously
    thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(event) => match event {
                    DebouncedEvent::Create(path) => println!("File created: {:?}", path),
                    DebouncedEvent::Write(path) => println!("File modified: {:?}", path),
                    DebouncedEvent::Remove(path) => println!("File removed: {:?}", path),
                    DebouncedEvent::Rename(src, dest) => println!("File renamed from {:?} to {:?}", src, dest),
                    DebouncedEvent::Error(err, path) => eprintln!("Error: {:?} at {:?}", err, path),
                    _ => (),
                },
                Err(err) => eprintln!("Watch error: {:?}", err),
            }
        }
    });
}

