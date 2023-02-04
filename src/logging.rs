//From: https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/log.html#log-messages-with-a-custom-logger, slightly modified

use colored::{ColoredString, Colorize};
use log::{Level, Metadata, Record};

pub const CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

pub struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("[{}]\t{}", level_to_color(&record.level()), record.args());
        }
    }

    fn flush(&self) {}
}

fn level_to_color(level: &Level) -> ColoredString {
    match level {
        Level::Error => { Level::Error.to_string().red() }
        Level::Warn => { Level::Warn.to_string().yellow() }
        Level::Info => { Level::Info.to_string().bright_blue() }
        Level::Debug => { Level::Debug.to_string().green() }
        Level::Trace => { Level::Error.to_string().normal() }
    }
}