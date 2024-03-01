
use log::{debug, SetLoggerError};

use log4rs::{
    append::console::{ConsoleAppender, Target},
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
    Handle
};

use chrono::Local;

use crate::cli;

/// Get name of cargo package from Cargo.toml
const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");

/// Setup logging within the application
/// Takes in cli args to help configure logging behavior
pub fn setup_logging(args: cli::Args) -> Result<Handle, SetLoggerError> {

    let timestamp: &str = &Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    
    // Generate log filename
    let log_filename: &str = &format!(
        "{}/{}.{}.log",
        args.logging_location,
        CARGO_PKG_NAME,
        timestamp
    ).to_string();

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();    

    // Create RootBuilder
    let mut rootbuilder = Root::builder();

    // add logfile appender
    if args.save_log {
        rootbuilder = rootbuilder.appender("logfile");     
    }

    // add stderr appender
    rootbuilder = rootbuilder.appender("stderr");
        

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let mut config = Config::builder();
        
    // Add logfile logging
    if args.save_log {
        // Generate FileAppender
        // Logging to log file. (with rolling)
        let logfile = log4rs::append::file::FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S %Z)(utc)} {l} - {m}\n")))
        .build(log_filename)
        .unwrap();

        config = config.appender(Appender::builder().build("logfile", Box::new(logfile)));
    }
    
    // Add console logging
    config = config.appender(
        Appender::builder()
            .filter(Box::new(ThresholdFilter::new(args.logging_level)))
            .build("stderr", Box::new(stderr)),
    );


    let handler: Result<log4rs::Handle, log::SetLoggerError> = log4rs::init_config(
        config.build(rootbuilder.build(args.logging_level)).unwrap()
    );
    
    debug!("Logging Enabled");

    handler
}