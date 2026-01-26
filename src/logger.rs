use tracing::{info, Level};
use tracing_subscriber::{fmt, EnvFilter};

pub fn init_logging() {
    let env_filter = EnvFilter::builder()
        .with_default_directive(Level::INFO.into())
        .from_env_lossy();

    fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .with_level(true)
        .init();

    info!("Logging initialized");
}
