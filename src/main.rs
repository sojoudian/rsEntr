mod data;
mod status;

fn main() {
    println!("Starting the 'entr' application rewrite in Rust...");

    // Path to monitor (you can adjust this to a specific directory)
    let path_to_watch = "."; // Current directory

    // Start monitoring for file changes
    status::monitor_files(path_to_watch);

    // Prevent the main thread from exiting
    loop {
        std::thread::park();
    }
}
