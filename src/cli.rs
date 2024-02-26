
use clap::Parser;

/// Arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// filename to write output to
    #[arg(short, long)]
    pub filename: String,

    /// flag for saving log to a file
    #[arg(short, long, default_value_t = false)]
    pub save_log: bool,

    /// argument for specifying where to store log files
    #[arg(short, long, default_value_t = String::from(""))]
    pub logging_location: String,
}