use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::process::Command;

fn main() -> notify::Result<()> {
    // Set up a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, configured with default settings.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Start watching the current directory.
    watcher.watch(".", RecursiveMode::Recursive)?;

    println!("Watching for changes...");

    // Loop to process events.
    for res in rx {
        match res {
            Ok(Event { kind: EventKind::Modify(..), paths, .. }) => {
                println!("File changed: {:?}", paths);

                // Run a command on file change.
                let mut child = Command::new("echo")
                    .arg("File changed!")
                    .spawn()
                    .expect("Failed to execute command");

                child.wait().expect("Command wasn't running");
            }
            Err(e) => println!("Watch error: {:?}", e),
        }
    }

    Ok(())
}
