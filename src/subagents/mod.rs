//! Subagents module for KLP
//! 
//! Manages always-running subagents: cronos, monitor, stealth, self

use std::collections::HashMap;
use tokio::sync::mpsc;
use tracing::{error, info, warn};

use crate::config::Config;

pub mod cronos;
pub mod monitor;
pub mod stealth;
pub mod selfdestruct;

/// Manager for all subagents
pub struct SubagentManager {
    config: Config,
    agents: HashMap<String, AgentHandle>,
}

struct AgentHandle {
    tx: mpsc::Sender<AgentCommand>,
}

enum AgentCommand {
    Start,
    Stop,
    Status,
}

impl SubagentManager {
    /// Create new subagent manager
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        Ok(Self {
            config: config.clone(),
            agents: HashMap::new(),
        })
    }
    
    /// Start a subagent
    pub async fn start_agent(&self, name: &str) -> anyhow::Result<()> {
        info!("Starting agent: {}", name);
        
        match name {
            "cronos" => {
                if self.config.subagents.cronos_enabled {
                    cronos::start(&self.config).await?;
                }
            }
            "monitor" => {
                if self.config.subagents.monitor_enabled {
                    monitor::start(&self.config).await?;
                }
            }
            "stealth" => {
                if self.config.subagents.stealth_enabled {
                    stealth::start(&self.config).await?;
                }
            }
            "self" => {
                if self.config.subagents.self_enabled {
                    selfdestruct::start(&self.config).await?;
                }
            }
            _ => {
                return Err(anyhow::anyhow!("Unknown agent: {}", name));
            }
        }
        
        Ok(())
    }
    
    /// Stop a subagent
    pub async fn stop_agent(&self, name: &str) -> anyhow::Result<()> {
        info!("Stopping agent: {}", name);
        Ok(())
    }
    
    /// List all agents
    pub async fn list_agents(&self) -> anyhow::Result<Vec<String>> {
        Ok(vec![
            "cronos".to_string(),
            "monitor".to_string(),
            "stealth".to_string(),
            "self".to_string(),
        ])
    }
    
    /// Get agent status
    pub async fn agent_status(&self, name: &str) -> anyhow::Result<String> {
        Ok(format!("{}: not running", name))
    }
}
