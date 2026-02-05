//! Stealth - Stealth mode subagent

use crate::config::Config;
use tracing::info;

pub async fn start(config: &Config) -> anyhow::Result<()> {
    info!("Stealth agent started");
    Ok(())
}
