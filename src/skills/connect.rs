//! Connect Skill - SSH, RDP, VNC connectivity

use async_trait::async_trait;
use tracing::info;

use crate::config::Config;
use crate::skills::Skill;

pub struct ConnectSkill {
    config: Config,
}

impl ConnectSkill {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        Ok(Self { config: config.clone() })
    }
}

#[async_trait]
impl Skill for ConnectSkill {
    fn name(&self) -> &str {
        "connect"
    }
    
    fn description(&self) -> &str {
        "SSH, RDP, VNC connectivity"
    }
    
    async fn execute(&self, args: &[String]) -> anyhow::Result<()> {
        println!("Connect Skill - Remote connectivity");
        Ok(())
    }
}
