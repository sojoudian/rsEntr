// src/main.rs
use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, exit};
use std::io;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() -> io::Result<()> {
    // Initialize environment and configuration
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: entr_app <command>");
        exit(1);
    }

    let command = &args[1];

    // Setting up a file watcher
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(2)).expect("Failed to initialize watcher");

    // Define the path to watch
    let path_to_watch = Path::new(".");
    watcher.watch(path_to_watch, RecursiveMode::Recursive)
        .expect("Failed to watch directory");

    println!("Watching directory for changes...");

    loop {
        match rx.recv() {
            Ok(DebouncedEvent::Write(path)) => {
                println!("File changed: {:?}", path);
                if let Err(e) = Command::new(command).status() {
                    eprintln!("Failed to execute command: {:?}", e);
                }
            }
            Err(e) => {
                eprintln!("watch error: {:?}", e);
                break;
            }
            _ => {}
        }
    }
    Ok(())
}
