// src/data.rs

// Define constants equivalent to those in `data.h`
pub const FILE_CHANGE_EVENT: &str = "File change detected";

// Define any structs or functions needed
pub struct FileEvent {
    pub name: String,
    pub changed: bool,
}

impl FileEvent {
    // Method to create a new `FileEvent`
    pub fn new(name: &str, changed: bool) -> Self {
        FileEvent {
            name: name.to_string(),
            changed,
        }
    }

    // Method to display the event details
    pub fn display(&self) {
        if self.changed {
            println!("{}: {}", FILE_CHANGE_EVENT, self.name);
        }
    }
}
