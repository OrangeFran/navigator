use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

// simple logger that outputs to a file
pub struct FileLogger {
    file: Option<File>,
}

impl FileLogger {
    pub fn empty() -> Self {
        Self { file: None }
    }

    pub fn set_logfile<S: ToString>(&self, file_name: S) -> Self {
        // only create new files
        // so you do not accidentally overwrite important files
        Self {
            file: Some(
                OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(file_name.to_string())
                    .expect("Failed to open the file")
            )
        }
    }

    pub fn log<D: Display>(&mut self, msg: D) {
        if let Some(f) = self.file.as_mut() {
            // get the current time
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            // write the msg with the current time to the file
            f.write_all(format!("{:?} \t- {}\n", now, msg).as_bytes())
                .expect("Failed to write to the logfile");
        }
    }
}
