//! KLP - Kali Language Processor
//! 
//! Core library for the Kali Language Processor CLI agent.
//! Provides modules for code generation, skills, subagents, security, and more.

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

pub mod codegen;
pub mod config;
pub mod connect;
pub mod chrome;
pub mod nlp;
pub mod report;
pub mod scheduler;
pub mod security;
pub mod skills;
pub mod subagents;
pub mod tui;

use std::path::PathBuf;

/// Get the KLP configuration directory
pub fn klp_dir() -> anyhow::Result<PathBuf> {
    let home = std::env::var("HOME")
        .map(PathBuf::from)
        .map_err(|_| anyhow::anyhow!("HOME environment variable not set"))?;
    Ok(home.join(".klp"))
}

/// Get the skills directory
pub fn skills_dir() -> anyhow::Result<PathBuf> {
    Ok(klp_dir()?.join("skills"))
}

/// Get the logs database path
pub fn logs_db_path() -> anyhow::Result<PathBuf> {
    Ok(klp_dir()?.join("log.db"))
}

/// Get the config file path
pub fn config_path() -> anyhow::Result<PathBuf> {
    Ok(klp_dir()?.join("config.toml"))
}

/// Get the schedule file path
pub fn schedule_path() -> anyhow::Result<PathBuf> {
    Ok(klp_dir()?.join("schedule.yaml"))
}

/// Ensure KLP directories exist
pub async fn ensure_dirs() -> anyhow::Result<()> {
    let klp = klp_dir()?;
    let skills = skills_dir()?;
    
    tokio::fs::create_dir_all(&klp).await?;
    tokio::fs::create_dir_all(&skills).await?;
    
    Ok(())
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Platform information
pub const PLATFORM: &str = "kali-linux-x86_64";

/// Check if running on Kali Linux
pub fn check_platform() -> anyhow::Result<()> {
    use std::fs;
    
    // Check /etc/os-release
    let os_release = fs::read_to_string("/etc/os-release")
        .map_err(|_| anyhow::anyhow!("Cannot read /etc/os-release"))?;
    
    if !os_release.to_lowercase().contains("kali") {
        return Err(anyhow::anyhow!(
            "KLP is designed exclusively for Kali Linux x86_64"
        ));
    }
    
    // Check architecture
    let uname = std::process::Command::new("uname")
        .arg("-m")
        .output()?;
    
    let arch = String::from_utf8_lossy(&uname.stdout);
    if !arch.trim().eq("x86_64") {
        return Err(anyhow::anyhow!(
            "KLP requires x86_64 architecture"
        ));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_klp_dir() {
        let dir = klp_dir().unwrap();
        assert!(dir.to_string_lossy().contains(".klp"));
    }
    
    #[test]
    fn test_version() {
        assert_eq!(VERSION, "0.1.0");
    }
}
