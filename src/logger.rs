use tracing::info;

pub fn init_logging() {
    tracing_subscriber::fmt::init();
    info!("Logging initialized");
}
