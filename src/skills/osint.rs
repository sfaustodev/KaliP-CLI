//! OSINT Skill - Open Source Intelligence gathering

use async_trait::async_trait;
use std::process::Command;
use tracing::{info, debug};

use crate::config::Config;
use crate::skills::Skill;

pub struct OsintSkill {
    config: Config,
}

impl OsintSkill {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        Ok(Self { config: config.clone() })
    }
}

#[async_trait]
impl Skill for OsintSkill {
    fn name(&self) -> &str {
        "osint"
    }
    
    fn description(&self) -> &str {
        "Open Source Intelligence gathering tools"
    }
    
    async fn execute(&self, args: &[String]) -> anyhow::Result<()> {
        if args.is_empty() {
            println!("OSINT Skill - Available commands:");
            println!("  ip-check        - Check public IP address");
            println!("  whois <domain>  - WHOIS lookup");
            println!("  dns <domain>    - DNS enumeration");
            println!("  portscan <host> - Quick port scan");
            return Ok(());
        }
        
        match args[0].as_str() {
            "ip-check" => {
                let output = Command::new("curl")
                    .args(&["-s", "ifconfig.me"])
                    .output()?;
                println!("Public IP: {}", String::from_utf8_lossy(&output.stdout));
            }
            "whois" => {
                if args.len() < 2 {
                    println!("Usage: klp skill osint whois <domain>");
                    return Ok(());
                }
                let output = Command::new("whois")
                    .arg(&args[1])
                    .output()?;
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            "dns" => {
                if args.len() < 2 {
                    println!("Usage: klp skill osint dns <domain>");
                    return Ok(());
                }
                let output = Command::new("dig")
                    .args(&[&args[1], "+short"])
                    .output()?;
                println!("DNS Records:\n{}", String::from_utf8_lossy(&output.stdout));
            }
            "portscan" => {
                if args.len() < 2 {
                    println!("Usage: klp skill osint portscan <host>");
                    return Ok(());
                }
                println!("Scanning {}...", args[1]);
                let output = Command::new("nmap")
                    .args(&["-F", &args[1]])
                    .output()?;
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            _ => {
                println!("Unknown OSINT command: {}", args[0]);
            }
        }
        
        Ok(())
    }
}
