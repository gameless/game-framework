

use log::{LogRecord, LogLevel, LogMetadata, LogLevelFilter};
use log::Log;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use log::SetLoggerError;
use log;
use std::sync::RwLock;

pub struct GameLogger {
    log_file: RwLock<File>,
    log_level: LogLevel,
}

impl GameLogger {
    pub fn new(log_level: LogLevel) -> GameLogger {
        let path = Path::new("log");
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(e) => panic!("ERROR: Could not create log file"),

            Ok(file) => file,
        };

        GameLogger {
            log_file: RwLock::new(file),
            log_level: log_level,
        }
    }
}

impl Log for GameLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.log_level
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let entry = format!("{} {}:{} - {}",
                                record.level(),
                                record.location().file(),
                                record.location().line(),
                                record.args());

            println!("{}", entry);
            self.log_file.write().unwrap().write_all(entry.as_bytes());
        }
    }
}
pub fn init(log_level: LogLevel) -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Trace);
        Box::new(GameLogger::new(log_level))
    })
}
