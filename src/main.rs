use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    let (tx, rx) = channel();

    // Watch the current directory for file changes
    let mut watcher = watcher(tx, Duration::from_secs(2)).expect("Failed to create file watcher");
    watcher
        .watch(".", RecursiveMode::Recursive)
        .expect("Failed to start watching directory");

    println!("Watching for changes...");

    loop {
        match rx.recv() {
            Ok(DebouncedEvent::Write(path)) => {
                println!("File changed: {:?}", path);

                // Replace with the command you want to execute on file change
                let mut child = Command::new("echo")
                    .arg("File changed!")
                    .spawn()
                    .expect("Failed to execute command");

                child.wait().expect("Command wasn't running");
            }
            Err(e) => println!("Watch error: {:?}", e),
            _ => (),
        }
    }
}
