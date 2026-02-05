//! Connectivity module for KLP
//! 
//! Handles SSH, RDP, VNC connections

use crate::config::Config;
use tracing::info;

/// Protocol for connections
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Protocol {
    Ssh,
    Rdp,
    Vnc,
    Icp,
    Ncp,
}

/// Connection manager
pub struct Connector {
    config: Config,
}

impl Connector {
    /// Create new connector
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        Ok(Self { config: config.clone() })
    }
    
    /// Connect to target
    pub async fn connect(&self, target: &str, protocol: Protocol, port: Option<u16>) -> anyhow::Result<()> {
        info!("Connecting to {} via {:?}", target, protocol);
        
        match protocol {
            Protocol::Ssh => {
                let default_port = port.unwrap_or(22);
                std::process::Command::new("ssh")
                    .args(&["-p", &default_port.to_string(), target])
                    .spawn()?
                    .wait()?;
            }
            Protocol::Rdp => {
                let default_port = port.unwrap_or(3389);
                println!("RDP connection to {}:{} (not implemented)", target, default_port);
            }
            Protocol::Vnc => {
                let default_port = port.unwrap_or(5900);
                println!("VNC connection to {}:{} (not implemented)", target, default_port);
            }
            Protocol::Icp => {
                println!("ICP connection to {} (not implemented)", target);
            }
            Protocol::Ncp => {
                println!("NCP connection to {} (not implemented)", target);
            }
        }
        
        Ok(())
    }
}
