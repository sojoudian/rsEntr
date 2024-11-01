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
        if self.changed {
            println!("{}: {}", FILE_CHANGE_EVENT, self.name);
        }
    }
}
