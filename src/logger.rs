use std::fs::File;
use std::fmt::Display;
use std::io::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

// simple logger that outputs to a file
struct FileLogger {
    file: Option<File>
}

impl FileLogger {
    pub const fn empty() -> Self {
        Self {
            file: None
        }
    }

    pub fn set_logfile(&mut self, file_name: String) {
        self.file = Some(File::open(file_name)
            .expect("Failed to open the file"));
    }

    pub fn log<D: Display>(&mut self, msg: D) {
        if let Some(f) = self.file.as_mut() {
            // get the current time
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            // write the msg with the current time to the file
            f.write_all(
                format!("{:?} - {}", now, msg).as_bytes()
            ).expect("Failed to write to the logfile");
        }
    }
}
