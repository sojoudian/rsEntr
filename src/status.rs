use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::ffi::OsString;
use nix::sys::signal::{self, Signal};
use nix::unistd::{self, Pid};

const DEFAULT_SCRIPT: &str = r#"# http://eradman.com/entrproject/status-filters.html
/^signal/ { print $3, "terminated by signal", $2; }
/^exit/ { print $3, "returned exit code", $2; }
"#;

pub struct StatusFilter {
    child: Option<Child>,
    stdin_pipe: Option<File>,
}

impl StatusFilter {
    pub fn new(safe_mode: u8) -> io::Result<Self> {
        let mut status_filter = StatusFilter {
            child: None,
            stdin_pipe: None,
        };
        status_filter.start_log_filter(safe_mode)?;
        Ok(status_filter)
    }

    fn start_log_filter(&mut self, safe: u8) -> io::Result<()> {
        // Get awk script path from environment or use default location
        let awk_script_path = match env::var("ENTR_STATUS_SCRIPT") {
            Ok(path) if !path.is_empty() => PathBuf::from(path),
            _ => {
                let home = env::var("HOME").map_err(|e| {
                    io::Error::new(io::ErrorKind::NotFound, format!("HOME not set: {}", e))
                })?;
                PathBuf::from(home).join(".entr").join("status.awk")
            }
        };

        // Create directory and install default script if needed
        if let Some(parent) = awk_script_path.parent() {
            fs::create_dir_all(parent)?;
        }

        if !awk_script_path.exists() {
            println!("entr: created '{}'", awk_script_path.display());
            let mut file = File::create(&awk_script_path)?;
            file.write_all(DEFAULT_SCRIPT.as_bytes())?;
        }

        // Setup awk command
        let mut cmd = Command::new("awk");
        cmd.arg("-F").arg("|")
            .arg("-f").arg(&awk_script_path);

        // Handle safe mode flag
        if safe != 2 {
            #[cfg(target_os = "linux")]
            cmd.arg("-S");
            #[cfg(not(target_os = "linux"))]
            cmd.arg("-safe");
        }

        // Create pipe for stdin
        let (read_fd, write_fd) = unistd::pipe()?;
        
        // Setup child process
        let child = cmd
            .stdin(unsafe { Stdio::from_raw_fd(read_fd) })
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;

        // Store write end of pipe and child process
        self.stdin_pipe = Some(unsafe { File::from_raw_fd(write_fd) });
        self.child = Some(child);

        Ok(())
    }

    pub fn write(&mut self, input: &str) -> io::Result<()> {
        if let Some(pipe) = self.stdin_pipe.as_mut() {
            pipe.write_all(input.as_bytes())?;
            pipe.flush()?;
        }
        Ok(())
    }

    pub fn end(&mut self) -> io::Result<()> {
        // Close stdin pipe
        self.stdin_pipe.take();

        // Kill child process if it exists
        if let Some(child) = self.child.as_mut() {
            let pid = Pid::from_raw(child.id() as i32);
            let _ = signal::kill(pid, Signal::SIGKILL);
        }

        Ok(())
    }
}

// Helper functions
pub fn create_dir(dir: &Path) -> io::Result<()> {
    if !dir.exists() {
        fs::create_dir_all(dir)?;
    }
    Ok(())
}

pub fn install_file(dst: &Path, content: &str) -> io::Result<()> {
    if !dst.exists() {
        println!("entr: created '{}'", dst.display());
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .mode(0o640)
            .open(dst)?;
        file.write_all(content.as_bytes())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_status_filter_creation() {
        let tmp_dir = TempDir::new().unwrap();
        env::set_var("HOME", tmp_dir.path());
        
        let mut filter = StatusFilter::new(1).unwrap();
        
        // Verify that the default script was created
        let script_path = tmp_dir.path().join(".entr").join("status.awk");
        assert!(script_path.exists());
        
        // Clean up
        filter.end().unwrap();
    }

    #[test]
    fn test_write_log() {
        let tmp_dir = TempDir::new().unwrap();
        env::set_var("HOME", tmp_dir.path());
        
        let mut filter = StatusFilter::new(1).unwrap();
        
        // Write test data
        filter.write("exit|0|test\n").unwrap();
        
        // Clean up
        filter.end().unwrap();
    }

    #[test]
    fn test_create_dir() {
        let tmp_dir = TempDir::new().unwrap();
        let test_dir = tmp_dir.path().join("test_dir");
        
        create_dir(&test_dir).unwrap();
        assert!(test_dir.exists());
    }

    #[test]
    fn test_install_file() {
        let tmp_dir = TempDir::new().unwrap();
        let test_file = tmp_dir.path().join("test.txt");
        
        install_file(&test_file, "test content").unwrap();
        assert!(test_file.exists());
        
        let content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(content, "test content");
    }
}
