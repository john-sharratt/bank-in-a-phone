use anyhow::anyhow;
use clap_verbosity_flag::Level;
use tracing::level_filters::LevelFilter;

pub fn init(log_level: Level) -> anyhow::Result<()> {
    match tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(match log_level {
            Level::Error => LevelFilter::ERROR,
            Level::Warn => LevelFilter::WARN,
            Level::Info => LevelFilter::INFO,
            Level::Debug => LevelFilter::DEBUG,
            Level::Trace => LevelFilter::TRACE,
        })
        .try_init()
    {
        Err(err) => Err(anyhow!(err)),
        _ => {
            tracing::info!("logging level: {}", log_level);
            Ok(())
        }
    }
}
