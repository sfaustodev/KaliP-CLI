//! Monitor Skill - System monitoring

use async_trait::async_trait;
use tracing::info;

use crate::config::Config;
use crate::skills::Skill;

pub struct MonitorSkill {
    config: Config,
}

impl MonitorSkill {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        Ok(Self { config: config.clone() })
    }
}

#[async_trait]
impl Skill for MonitorSkill {
    fn name(&self) -> &str {
        "monitor"
    }
    
    fn description(&self) -> &str {
        "Monitor system resources and network"
    }
    
    async fn execute(&self, args: &[String]) -> anyhow::Result<()> {
        println!("Monitor Skill - Watching system...");
        Ok(())
    }
}
