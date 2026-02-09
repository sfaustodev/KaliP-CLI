//! System Operations Skill - Execute system commands and manage OS resources

use async_trait::async_trait;
use std::process::Command;
use tracing::{info, warn};

use crate::config::Config;
use crate::skills::Skill;

pub struct SystemOperationsSkill {
    config: Config,
}

impl SystemOperationsSkill {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        Ok(Self { config: config.clone() })
    }
    
    /// Get system information
    fn get_system_info(&self) -> anyhow::Result<String> {
        let mut info = String::new();
        
        // OS info
        #[cfg(target_os = "macos")]
        info.push_str("OS: macOS\n");
        #[cfg(target_os = "linux")]
        info.push_str("OS: Linux\n");
        #[cfg(target_os = "windows")]
        info.push_str("OS: Windows\n");
        
        // Architecture
        info.push_str(&format!("Architecture: {}\n", std::env::consts::ARCH));
        
        // Try to get more info with system commands
        if let Ok(output) = Command::new("uname").arg("-a").output() {
            if output.status.success() {
                info.push_str(&format!("Kernel: {}\n", 
                    String::from_utf8_lossy(&output.stdout)));
            }
        }
        
        Ok(info)
    }
    
    /// Execute shell command
    async fn execute_command(&self, cmd: &str) -> anyhow::Result<String> {
        info!("Executing command: {}", cmd);
        
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", cmd])
                .output()?
        } else {
            Command::new("sh")
                .args(["-c", cmd])
                .output()?
        };
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        if !output.status.success() {
            warn!("Command failed: {}", stderr);
            return Err(anyhow::anyhow!("Command failed: {}", stderr));
        }
        
        Ok(stdout.to_string())
    }
    
    /// List processes
    fn list_processes(&self) -> anyhow::Result<String> {
        let output = if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
            Command::new("ps").args(["aux"]).output()?
        } else {
            Command::new("tasklist").output()?
        };
        
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
    
    /// Kill process
    fn kill_process(&self, pid: &str) -> anyhow::Result<()> {
        info!("Killing process: {}", pid);
        
        let status = if cfg!(target_os = "windows") {
            Command::new("taskkill")
                .args(["/PID", pid, "/F"])
                .status()?
        } else {
            Command::new("kill")
                .arg(pid)
                .status()?
        };
        
        if !status.success() {
            return Err(anyhow::anyhow!("Failed to kill process {}"));
        }
        
        Ok(())
    }
    
    /// Read file
    fn read_file(&self, path: &str) -> anyhow::Result<String> {
        use std::fs;
        Ok(fs::read_to_string(path)?)
    }
    
    /// Write file
    fn write_file(&self, path: &str, content: &str) -> anyhow::Result<()> {
        use std::fs;
        fs::write(path, content)?;
        Ok(())
    }
    
    /// Delete file
    fn delete_file(&self, path: &str) -> anyhow::Result<()> {
        use std::fs;
        fs::remove_file(path)?;
        Ok(())
    }
}

#[async_trait]
impl Skill for SystemOperationsSkill {
    fn name(&self) -> &str {
        "system-operations"
    }
    
    fn description(&self) -> &str {
        "Execute system commands and manage OS resources"
    }
    
    async fn execute(&self, args: &[String]) -> anyhow::Result<()> {
        if args.is_empty() {
            println!("System Operations Skill");
            println!("\nUsage:");
            println!("  klp sys info                - System information");
            println!("  klp sys exec <command>      - Execute shell command");
            println!("  klp sys ps                  - List processes");
            println!("  klp sys kill <pid>          - Kill process");
            println!("  klp sys file read <path>    - Read file");
            println!("  klp sys file write <path>   - Write file");
            println!("  klp sys file delete <path>  - Delete file");
            return Ok(());
        }
        
        match args[0].as_str() {
            "info" => {
                let info = self.get_system_info()?;
                println!("{}", info);
            }
            "exec" => {
                if args.len() > 1 {
                    let cmd = args[1..].join(" ");
                    let output = self.execute_command(&cmd).await?;
                    println!("{}", output);
                }
            }
            "ps" => {
                let processes = self.list_processes()?;
                println!("{}", processes);
            }
            "kill" => {
                if args.len() > 1 {
                    self.kill_process(&args[1])?;
                    println!("Process {} terminated", args[1]);
                }
            }
            "file" => {
                if args.len() > 2 {
                    match args[1].as_str() {
                        "read" => {
                            let content = self.read_file(&args[2])?;
                            println!("{}", content);
                        }
                        "write" => {
                            if args.len() > 3 {
                                self.write_file(&args[2], &args[3])?;
                                println!("File written: {}", args[2]);
                            }
                        }
                        "delete" => {
                            self.delete_file(&args[2])?;
                            println!("File deleted: {}", args[2]);
                        }
                        _ => println!("Unknown file command: {}", args[1]),
                    }
                }
            }
            _ => {
                println!("Unknown command: {}", args[0]);
            }
        }
        
        Ok(())
    }
}
