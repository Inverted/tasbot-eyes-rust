//From: , slightly modified

use colored::{ColoredString, Colorize};
use log::{Level, Metadata, Record};

/// Global reference to the `ConsoleLogger`
pub const CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

/// A console logger
pub struct ConsoleLogger;

/// The console logger which prints log entries to the standard output
///
/// # Credits
/// Taken from the [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/) from chapter [9.1.1 Log Messages](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/log.html#log-messages-with-a-custom-logger), slightly modified
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

/// Color the various log level
///
/// # Input
/// The log `Level`
///
/// # Output
/// A `ColoredString` that can be printed
fn level_to_color(level: &Level) -> ColoredString {
    match level {
        Level::Error => { Level::Error.to_string().red() }
        Level::Warn => { Level::Warn.to_string().yellow() }
        Level::Info => { Level::Info.to_string().bright_blue() }
        Level::Debug => { Level::Debug.to_string().green() }
        Level::Trace => { Level::Error.to_string().normal() }
    }
}