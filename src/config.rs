//! Configuration management for KLP
//! 
//! Handles loading, saving, and managing KLP configuration from `~/.klp/config.toml`

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use tracing::{info, warn};

use crate::{config_path, ensure_dirs, klp_dir};

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// KLP version
    pub version: String,
    
    /// User settings
    pub user: UserConfig,
    
    /// API configurations
    pub apis: ApiConfig,
    
    /// Code generation settings
    pub codegen: CodegenConfig,
    
    /// TUI settings
    pub tui: TuiConfig,
    
    /// Security settings
    pub security: SecurityConfig,
    
    /// Subagent settings
    pub subagents: SubagentConfig,
    
    /// Skill settings
    pub skills: SkillsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    pub name: String,
    pub email: Option<String>,
    pub default_mode: String, // "cli" or "tui"
    pub language: String,     // "en" or "pt"
    pub confirm_actions: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub grok: Option<ApiKey>,
    pub groq: Option<ApiKey>,
    pub openai: Option<ApiKey>,
    pub default_api: String,
    pub timeout_seconds: u64,
    pub max_retries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub key: String,
    pub endpoint: Option<String>,
    pub model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodegenConfig {
    pub default_api: String,
    pub local_model_path: Option<PathBuf>,
    pub use_local_fallback: bool,
    pub max_tokens: u32,
    pub temperature: f32,
    pub top_p: f32,
    pub save_history: bool,
    pub history_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiConfig {
    pub theme: String,           // "cyberpunk", "dark", "light"
    pub animation_enabled: bool,
    pub dragon_frames_per_second: u8,
    pub show_line_numbers: bool,
    pub tab_size: usize,
    pub font_size: u8,
    pub cyberpunk_colors: CyberpunkColors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CyberpunkColors {
    pub primary: String,      // e.g., "#00ff00"
    pub secondary: String,    // e.g., "#ff00ff"
    pub accent: String,       // e.g., "#00ffff"
    pub background: String,   // e.g., "#0a0a0a"
    pub foreground: String,   // e.g., "#ffffff"
    pub dragon_fire: String,  // e.g., "#ff4500"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub ethical_mode: bool,
    pub require_confirmation: bool,
    pub encrypt_logs: bool,
    pub log_encryption_key: Option<String>,
    pub stealth_mode: bool,
    pub self_destruct_enabled: bool,
    pub password_hash: Option<String>,
    pub voice_confirmation_enabled: bool,
    pub voice_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubagentConfig {
    pub cronos_enabled: bool,
    pub monitor_enabled: bool,
    pub stealth_enabled: bool,
    pub self_enabled: bool,
    pub schedule_file: PathBuf,
    pub check_interval_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillsConfig {
    pub skills_dir: PathBuf,
    pub auto_reload: bool,
    pub built_in_enabled: bool,
    pub custom_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        let klp_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join(".klp");
        
        Self {
            version: "0.1.0".to_string(),
            user: UserConfig {
                name: whoami::username(),
                email: None,
                default_mode: "cli".to_string(),
                language: "en".to_string(),
                confirm_actions: true,
            },
            apis: ApiConfig {
                grok: None,
                groq: None,
                openai: None,
                default_api: "grok".to_string(),
                timeout_seconds: 30,
                max_retries: 3,
            },
            codegen: CodegenConfig {
                default_api: "grok".to_string(),
                local_model_path: None,
                use_local_fallback: true,
                max_tokens: 4096,
                temperature: 0.7,
                top_p: 0.9,
                save_history: true,
                history_path: klp_dir.join("history.json"),
            },
            tui: TuiConfig {
                theme: "cyberpunk".to_string(),
                animation_enabled: true,
                dragon_frames_per_second: 12,
                show_line_numbers: true,
                tab_size: 4,
                font_size: 12,
                cyberpunk_colors: CyberpunkColors {
                    primary: "#00ff00".to_string(),
                    secondary: "#ff00ff".to_string(),
                    accent: "#00ffff".to_string(),
                    background: "#0a0a0a".to_string(),
                    foreground: "#ffffff".to_string(),
                    dragon_fire: "#ff4500".to_string(),
                },
            },
            security: SecurityConfig {
                ethical_mode: true,
                require_confirmation: true,
                encrypt_logs: true,
                log_encryption_key: None,
                stealth_mode: false,
                self_destruct_enabled: true,
                password_hash: None,
                voice_confirmation_enabled: false,
                voice_hash: None,
            },
            subagents: SubagentConfig {
                cronos_enabled: true,
                monitor_enabled: true,
                stealth_enabled: true,
                self_enabled: true,
                schedule_file: klp_dir.join("schedule.yaml"),
                check_interval_seconds: 60,
            },
            skills: SkillsConfig {
                skills_dir: klp_dir.join("skills"),
                auto_reload: true,
                built_in_enabled: true,
                custom_enabled: true,
            },
        }
    }
}

impl Config {
    /// Load configuration from file or create default
    pub async fn load() -> anyhow::Result<Self> {
        let config_path = config_path()?;
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path).await?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            let config = Config::default();
            config.save().await?;
            Ok(config)
        }
    }
    
    /// Save configuration to file
    pub async fn save(&self) -> anyhow::Result<()> {
        ensure_dirs().await?;
        let config_path = config_path()?;
        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content).await?;
        Ok(())
    }
    
    /// Get configuration file path
    pub fn path() -> anyhow::Result<PathBuf> {
        config_path()
    }
    
    /// Initialize configuration directory
    pub async fn initialize(force: bool) -> anyhow::Result<()> {
        let config_path = config_path()?;
        
        if config_path.exists() && !force {
            println!("KLP is already initialized.");
            println!("Use --force to reinitialize.");
            return Ok(());
        }
        
        ensure_dirs().await?;
        
        // Create default config
        let config = Config::default();
        config.save().await?;
        
        // Create default schedule file
        let schedule_path = crate::schedule_path()?;
        let default_schedule = r#"# KLP Schedule Configuration
# Define scheduled tasks here

# Example: Check IP every 30 minutes
# - name: ip-watch
#   interval: 30m
#   command: klp osint ip-check

# Example: Clean browser history every 30 minutes
# - name: clean-browser
#   interval: 30m
#   command: klp chrome clear-history
"#;
        fs::write(&schedule_path, default_schedule).await?;
        
        // Create .gitignore
        let gitignore_path = klp_dir()?.join(".gitignore");
        let gitignore_content = r#"# KLP Secrets and Logs
config.toml
*.key
*.pem
log.db
history.json
.voice_cache/
"#;
        fs::write(&gitignore_path, gitignore_content).await?;
        
        println!("✓ KLP initialized successfully!");
        println!("  Config: ~/.klp/config.toml");
        println!("  Skills: ~/.klp/skills/");
        println!("  Schedule: ~/.klp/schedule.yaml");
        println!("\nNext steps:");
        println!("  1. Edit ~/.klp/config.toml to add your API keys");
        println!("  2. Run 'klp code' to start the TUI");
        
        Ok(())
    }
    
    /// Edit configuration file
    pub async fn edit() -> anyhow::Result<()> {
        let config_path = config_path()?;
        
        let editor = std::env::var("EDITOR")
            .unwrap_or_else(|_| "nano".to_string());
        
        let status = tokio::process::Command::new(&editor)
            .arg(&config_path)
            .status()
            .await?;
        
        if !status.success() {
            return Err(anyhow::anyhow!("Editor exited with error"));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.version, "0.1.0");
        assert!(config.security.ethical_mode);
    }
}
