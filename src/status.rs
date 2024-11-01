// src/data.rs
pub const FILE_CHANGE_EVENT: &str = "File change detected";

pub struct FileEvent {
    pub name: String,
    pub changed: bool,
}

impl FileEvent {
    pub fn new(name: &str, changed: bool) -> Self {
        FileEvent {
            name: name.to_string(),
            changed,
        }
    }

    pub fn display(&self) {
        println!("{}: {}", FILE_CHANGE_EVENT, self.name);
    }
}

// src/status.rs
use crate::data::{FileEvent, FILE_CHANGE_EVENT}; // Add this line to use the items from data.rs
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

/// Function to watch for file changes in a directory
pub fn monitor_files(path: &str) {
    let _ = channel::<Result<Event>>(); // Ignoring the channel as it is unused for now

    // Create a recommended watcher object
    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event>| match res {
            Ok(event) => {
                if let Some(path) = event.paths.get(0) {
                    let file_event = FileEvent::new(path.to_str().unwrap_or("unknown"), true);
                    file_event.display(); // Use the display method

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

