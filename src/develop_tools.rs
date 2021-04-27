use log::Level;

pub fn execute_query(query: &str) {
    log::debug!("Executing query: {}", query);
}

pub fn execute_query_err(_query: &str) -> Result<(), &'static str> {
    Err("I'm afraid I can't do that")
}

use log::{Metadata, Record};
pub struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata:&Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record:&Record) {
        if self.enabled(record.metadata()) {
            println!("Rust says: {}: {:?}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

use syslog::{Facility, Error};

pub fn log_to_syslog() -> Result<(), Error>{
    syslog::init(Facility::LOG_USER,
    log::LevelFilter::Debug,
    Some("My app name"))?;

    log::debug!("this is a debug {}", "message");
    log::error!("this is an error");

    Ok(())
}