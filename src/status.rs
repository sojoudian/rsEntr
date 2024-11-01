// src/status.rs

// Import necessary standard library modules
use std::fs;

/// Function to display status, equivalent to a function in `status.c`
pub fn display_status() {
    match fs::read_dir(".") {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("Found file: {:?}", entry.file_name());
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading directory: {:?}", e);
        }
    }
}
