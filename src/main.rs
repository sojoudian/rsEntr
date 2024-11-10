use clap::Parser;
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::process::Command;
use std::sync::mpsc::channel;

/// Simple program to watch files and execute a command on change
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Command to execute on file change
    #[arg(short, long)]
    command: String,

    /// Clear the screen before executing the command
    #[arg(short, long, default_value_t = false)]
    clear: bool,

    /// Postpone the first execution until a file is modified
    #[arg(short, long, default_value_t = false)]
    postpone: bool,

    /// Directories to watch
    #[arg(short, long, default_value = ".")]
    directories: Vec<String>,
}

fn main() -> notify::Result<()> {
    let args = Args::parse();

    // Set up a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, configured with default settings.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Watch the specified directories.
    for dir in &args.directories {
        watcher.watch(Path::new(dir), RecursiveMode::Recursive)?;
    }

    println!("Watching for changes...");

    // If not postponing, execute the command once at the start.
    if !args.postpone {
        execute_command(&args.command, args.clear);
    }

    // Loop to process events.
    for res in rx {
        match res {
            Ok(Event {
                kind: EventKind::Modify(..),
                paths,
                ..
            }) => {
                println!("File changed: {:?}", paths);
                execute_command(&args.command, args.clear);
            }
            Err(e) => println!("Watch error: {:?}", e),
            _ => (), // Handle other events if necessary
        }
    }

    Ok(())
}

fn execute_command(command: &str, clear: bool) {
    if clear {
        // Clear the screen before executing the command
        // This is a simple way to clear the terminal screen
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    // Execute the specified command
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("Failed to execute command");

    child.wait().expect("Command wasn't running");
}