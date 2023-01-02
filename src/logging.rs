//From: https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/log.html#log-messages-with-a-custom-logger, slightly modified

use log::{Level, Metadata, Record};

pub const CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

pub struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("[{}]\t{}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}