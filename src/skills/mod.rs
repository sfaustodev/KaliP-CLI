//! Skills system for KLP
//! 
//! Manages installable skills stored in ~/.klp/skills/

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{debug, error, info, warn};

use crate::config::Config;
use crate::skills_dir;

pub mod chrome;
pub mod connect;
pub mod monitor;
pub mod osint;
pub mod pentest;
pub mod report;
pub mod selfdestruct;
pub mod skill_creator;
pub mod sys;

/// Skill manager for loading and executing skills
pub struct SkillManager {
    config: Config,
    skills: HashMap<String, Box<dyn Skill>>,
}

/// Trait for all skills
#[async_trait::async_trait]
pub trait Skill: Send + Sync {
    /// Skill name
    fn name(&self) -> &str;
    
    /// Skill description
    fn description(&self) -> &str;
    
    /// Execute skill with arguments
    async fn execute(&self, args: &[String]) -> anyhow::Result<()>;
}

impl SkillManager {
    /// Create new skill manager and load all skills
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        let mut manager = Self {
            config: config.clone(),
            skills: HashMap::new(),
        };
        
        manager.load_built_in_skills().await?;
        manager.load_custom_skills().await?;
        
        Ok(manager)
    }
    
    /// Load built-in skills
    async fn load_built_in_skills(&mut self) -> anyhow::Result<()> {
        if !self.config.skills.built_in_enabled {
            return Ok(());
        }
        
        // Register built-in skills
        self.register_skill(Box::new(osint::OsintSkill::new(&self.config).await?));
        self.register_skill(Box::new(pentest::PentestSkill::new(&self.config).await?));
        self.register_skill(Box::new(sys::SysSkill::new(&self.config).await?));
        self.register_skill(Box::new(chrome::ChromeSkill::new(&self.config).await?));
        self.register_skill(Box::new(report::ReportSkill::new(&self.config).await?));
        self.register_skill(Box::new(monitor::MonitorSkill::new(&self.config).await?));
        self.register_skill(Box::new(selfdestruct::SelfSkill::new(&self.config).await?));
        self.register_skill(Box::new(connect::ConnectSkill::new(&self.config).await?));
        self.register_skill(Box::new(skill_creator::SkillCreatorSkill::new(&self.config).await?));
        self.register_skill(Box::new(git::GitSkill::new(&self.config).await?));
        
        info!("Loaded {} built-in skills", self.skills.len());
        Ok(())
    }
    
    /// Load custom skills from ~/.klp/skills/
    async fn load_custom_skills(&mut self) -> anyhow::Result<()> {
        if !self.config.skills.custom_enabled {
            return Ok(());
        }
        
        let skills_dir = skills_dir()?;
        
        if !skills_dir.exists() {
            return Ok(());
        }
        
        let mut entries = fs::read_dir(&skills_dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            
            if path.extension().map(|e| e == "rs").unwrap_or(false) {
                let name = path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                
                // Custom skills are loaded dynamically
                // In a full implementation, these would be compiled and loaded as plugins
                info!("Found custom skill: {}", name);
            }
        }
        
        Ok(())
    }
    
    /// Register a skill
    fn register_skill(&mut self, skill: Box<dyn Skill>) {
        let name = skill.name().to_string();
        self.skills.insert(name, skill);
    }
    
    /// Add a new skill from file
    pub async fn add_skill(&self, file: &Path) -> anyhow::Result<()> {
        if !file.exists() {
            return Err(anyhow::anyhow!("Skill file does not exist: {}", file.display()));
        }
        
        if file.extension().map(|e| e != "rs").unwrap_or(true) {
            return Err(anyhow::anyhow!("Skill file must be a Rust (.rs) file"));
        }
        
        let skills_dir = skills_dir()?;
        let dest = skills_dir.join(file.file_name().unwrap());
        
        fs::copy(file, &dest).await?;
        
        info!("Skill added: {}", dest.display());
        Ok(())
    }
    
    /// List all installed skills
    pub async fn list_skills(&self) -> anyhow::Result<Vec<String>> {
        let mut names: Vec<String> = self.skills.keys().cloned().collect();
        names.sort();
        Ok(names)
    }
    
    /// Remove a skill
    pub async fn remove_skill(&self, name: &str) -> anyhow::Result<()> {
        let skills_dir = skills_dir()?;
        let skill_file = skills_dir.join(format!("{}.rs", name));
        
        if skill_file.exists() {
            fs::remove_file(&skill_file).await?;
            info!("Skill removed: {}", name);
        } else {
            warn!("Skill not found: {}", name);
        }
        
        Ok(())
    }
    
    /// Run a skill
    pub async fn run_skill(&self, name: &str, args: &[String]) -> anyhow::Result<()> {
        if let Some(skill) = self.skills.get(name) {
            info!("Executing skill: {} with args: {:?}", name, args);
            skill.execute(args).await?;
        } else {
            return Err(anyhow::anyhow!("Skill not found: {}", name));
        }
        
        Ok(())
    }
}

/// Macro for registering skills
#[macro_export]
macro_rules! register_skill {
    ($manager:expr, $skill:expr) => {
        $manager.register_skill(Box::new($skill));
    };
}
