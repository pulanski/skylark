use anyhow::Result;
use tracing_subscriber::fmt::Subscriber;

/// Initializes the logging system with a default configuration.
pub fn init_logging() -> Result<()> {
    let subscriber = Subscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_ansi(true)
        .with_max_level(tracing::Level::TRACE)
        .without_time()
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}
