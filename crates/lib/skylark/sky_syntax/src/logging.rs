use anyhow::Result;
use tracing::Level;
use tracing_subscriber::fmt::Subscriber;

/// Initializes the logging system with a default configuration.
pub fn init_logging(log_level: Level) -> Result<()> {
    let subscriber = Subscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_ansi(true)
        .with_max_level(log_level)
        .without_time()
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}
