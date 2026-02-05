//! Sys Skill - System control and monitoring

use async_trait::async_trait;
use std::process::Command;
use sysinfo::{System, RefreshKind};
use tracing::info;

use crate::config::Config;
use crate::skills::Skill;

pub struct SysSkill {
    config: Config,
}

impl SysSkill {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        Ok(Self { config: config.clone() })
    }
}

#[async_trait]
impl Skill for SysSkill {
    fn name(&self) -> &str {
        "sys"
    }
    
    fn description(&self) -> &str {
        "System control, process management, and monitoring"
    }
    
    async fn execute(&self, args: &[String]) -> anyhow::Result<()> {
        if args.is_empty() {
            println!("Sys Skill - Available commands:");
            println!("  top              - Show system processes");
            println!("  kill <pid>       - Kill process by PID");
            println!("  info             - System information");
            println!("  disk             - Disk usage");
            println!("  net              - Network interfaces");
            return Ok(());
        }
        
        match args[0].as_str() {
            "top" => {
                let mut s = System::new_all();
                s.refresh_all();
                
                println!("{:<10} {:<40} {:<10} {:<10}", "PID", "Name", "CPU%", "Memory(MB)");
                for (pid, process) in s.processes() {
                    let name = process.name();
                    let cpu = process.cpu_usage();
                    let mem = process.memory() as f64 / 1024.0 / 1024.0;
                    println!("{:<10} {:<40} {:<10.1} {:<10.1}",
                        pid,
                        name,
                        cpu,
                        mem
                    );
                }
            }
            "kill" => {
                if args.len() < 2 {
                    println!("Usage: klp skill sys kill <pid>");
                    return Ok(());
                }
                let pid: i32 = args[1].parse()?;
                Command::new("kill")
                    .arg(pid.to_string())
                    .spawn()?
                    .wait()?;
                println!("Killed process {}", pid);
            }
            "info" => {
                let mut s = System::new_all();
                s.refresh_all();
                
                println!("System Information:");
                println!("  OS: {} {}", System::name().unwrap_or_default(), System::os_version().unwrap_or_default());
                println!("  Kernel: {}", System::kernel_version().unwrap_or_default());
                println!("  CPUs: {}", s.cpus().len());
                println!("  Memory: {:.1} GB / {:.1} GB",
                    s.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0,
                    s.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0
                );
            }
            "disk" => {
                let output = Command::new("df").args(&["-h"]).output()?;
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            "net" => {
                // Use ifconfig for cross-platform compatibility, fallback to ip
                let output = if cfg!(target_os = "macos") {
                    Command::new("ifconfig").output()?
                } else {
                    Command::new("ip").args(&["addr"]).output()?
                };
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            _ => {
                println!("Unknown sys command: {}", args[0]);
            }
        }
        
        Ok(())
    }
}
