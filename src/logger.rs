use std::fs::File;
use std::io::prelude::*;
use log::{Log, Record, Level, Metadata};

// simple logger that outputs to a file
struct FileLogger {
    file: File
}

impl Log for FileLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.file.write(bytes!(
                format!("{} - {}", record.level(), record.args())
            ));
        }
    }
}

impl FileLogger {
    pub fn new(file_name: String) -> Self {
        Self {
            file: File::open(file_name)
                .expect("File does not exist")
        }
    }
}
