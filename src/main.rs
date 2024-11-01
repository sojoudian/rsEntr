// src/main.rs

// Include the modules for the application
mod data;
mod status;

fn main() {
    // Entry point of the program
    println!("Starting the 'entr' application rewrite in Rust...");

    // Example function call from the `status` module
    status::display_status();
}
