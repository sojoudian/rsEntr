use clap::Parser;
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::process::{Command, Child, Stdio};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

/// Simple program to watch files and execute a command on change
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Command to execute on file change
    #[arg(short = 'c', long)]
    command: String,

    /// Clear the screen before executing the command
    #[arg(short = 'l', long, default_value_t = false)]
    clear: bool,

    /// Postpone the first execution until a file is modified
    #[arg(short = 'p', long, default_value_t = false)]
    postpone: bool,

    /// Restart the command if it's still running on a new file change
    #[arg(short = 'r', long, default_value_t = false)]
    restart: bool,

    /// Directories or files to watch
    #[arg(short = 'd', long, default_value = ".")]
    watch_paths: Vec<String>,
}

fn main() -> notify::Result<()> {
    let args = Args::parse();

    // Set up a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, configured with default settings.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Watch the specified directories or files.
    for path in &args.watch_paths {
        watcher.watch(Path::new(path), RecursiveMode::Recursive)?;
    }

    println!("Watching for changes...");

    // Shared state for the child process
    let child_process = Arc::new(Mutex::new(None));

    // If not postponing, execute the command once at the start.
    if !args.postpone {
        execute_command(&args.command, args.clear, args.restart, Arc::clone(&child_process));
    }

    // Loop to process events.
    for res in rx {
        match res {
            Ok(Event { kind, paths, .. }) => {
                match kind {
                    EventKind::Modify(..)
                    | EventKind::Create(..)
                    | EventKind::Remove(..)
                    | EventKind::Rename(..) => {
                        println!("File event {:?} detected on: {:?}", kind, paths);
                        execute_command(&args.command, args.clear, args.restart, Arc::clone(&child_process));
                    }
                    _ => (),
                }
            }
            Err(e) => println!("Watch error: {:?}", e),
        }
    }

    Ok(())
}

fn execute_command(command: &str, clear: bool, restart: bool, child_process: Arc<Mutex<Option<Child>>>) {
    if clear {
        // Clear the screen before executing the command
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    if restart {
        // Terminate the existing child process if it exists
        if let Some(mut child) = child_process.lock().unwrap().take() {
            child.kill().expect("Failed to kill existing process");
            child.wait().expect("Failed to wait on child");
        }
    }

    // Spawn the new child process
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute command");

    if restart {
        // Store the child process handle
        *child_process.lock().unwrap() = Some(child);
    } else {
        // Wait for the child process to exit
        child.wait().expect("Command wasn't running");
    }
}