//! Cronos - Scheduler subagent

use crate::config::Config;
use tracing::info;

pub async fn start(config: &Config) -> anyhow::Result<()> {
    info!("Cronos scheduler started");
    Ok(())
}
