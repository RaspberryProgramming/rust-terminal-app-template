
use log::{debug, LevelFilter, SetLoggerError};

use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
    Handle
};

use chrono::Local;

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn setup_logging() -> Result<Handle, SetLoggerError> {
    let level = log::LevelFilter::Info;
    let timestamp: &str = &Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let log_path: &str = "log";
    
    let log_filename: &str = &format!(
        "{}/{}.{}.log",
        log_path,
        CARGO_PKG_NAME,
        timestamp
    ).to_string();

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    // Logging to log file. (with rolling)
    let logfile = log4rs::append::file::FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S %Z)(utc)} {l} - {m}\n")))
        .build(log_filename)
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    let handler: Result<log4rs::Handle, log::SetLoggerError> = log4rs::init_config(config);
    
    debug!("Logging Enabled");

    handler
}