use std::path::PathBuf;

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};

#[derive(Parser, Debug, Clone)]
pub struct Opts {
    /// Path to the data
    #[clap(long, default_value = "/opt/ledger.log")]
    pub data_path: PathBuf,
    /// Seconds between saving the ledger to disk
    #[clap(long, default_value = "30")]
    pub save_frequency: u64,
    /// Verbosity of the logging
    #[command(flatten)]
    pub verbosity: Verbosity<InfoLevel>,
    /// HTTP port to use.
    #[clap(long, default_value = "8000")]
    pub http_port: u16,
    /// HTTP port to use.
    #[clap(long, default_value = "8001")]
    pub ws_port: u16,
}
