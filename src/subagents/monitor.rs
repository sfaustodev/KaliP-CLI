//! Monitor - System monitoring subagent

use crate::config::Config;
use tracing::info;

pub async fn start(config: &Config) -> anyhow::Result<()> {
    info!("Monitor agent started");
    Ok(())
}
