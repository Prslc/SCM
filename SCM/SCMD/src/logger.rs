use log::{Record, Level, Metadata, LevelFilter, SetLoggerError};
use chrono::Local;

struct Loggers;

impl log::Log for Loggers {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
            println!("[{}] [{}] {}", ts, record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: Loggers = Loggers;

pub fn init_logger() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)?;
    log::set_max_level(LevelFilter::Info);
    Ok(())
}
