
use clap::Parser;

/// Arguments
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// filename to write output to
    #[arg(short, long)]
    pub filename: String,

    /// flag for saving log to a file
    #[arg(short, long, default_value_t = false)]
    pub save_log: bool,

    /// argument for specifying where to store log files
    #[arg(short, long, default_value_t = String::from("log"))]
    pub logging_location: String,

    /// argument for specifying logging level
    /// Available options are:
    /// - Off
    /// - Error
    /// - Warn
    /// - Info
    /// - Debug
    /// - Trace"
    #[arg(long, default_value_t = log::LevelFilter::Info)]
    pub logging_level: log::LevelFilter,
}