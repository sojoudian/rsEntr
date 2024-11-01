// src/status.rs

use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::path::Path;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;

/// Function to watch for file changes in a directory
pub fn monitor_files(path: &str) {
    // Create a channel to receive the events
    let (tx, rx): (Sender<Result<Event>>, _) = channel();

    // Create a recommended watcher object
    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event>| match res {
            Ok(event) => {
                if let Some(path) = event.paths.get(0) {
                    match event.kind {
                        EventKind::Create(_) => println!("File created: {:?}", path),
                        EventKind::Modify(_) => println!("File modified: {:?}", path),
                        EventKind::Remove(_) => println!("File removed: {:?}", path),
                        _ => (),
                    }
                }
            }
            Err(err) => eprintln!("Watch error: {:?}", err),
        },
        Config::default().with_poll_interval(Duration::from_secs(2)),
    )
    .expect("Failed to create watcher");

    // Add the path to be watched (with recursive mode)
    watcher
        .watch(Path::new(path), RecursiveMode::Recursive)
        .expect("Failed to watch path");

    println!("Monitoring file changes in directory: {}", path);

    // Keep the main thread alive to let the watcher work
    loop {
        thread::park();
    }
}

