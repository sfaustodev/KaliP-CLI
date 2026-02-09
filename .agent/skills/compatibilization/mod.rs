//! Compatibilization Skill - Discover and manage skills from multiple sources
//! 
//! This skill provides unified skill discovery across:
//! - Anthropic's official skills repository
//! - OpenAI's official skills repository  
//! - Local skill directories
//! - npm packages (@tech-leads-club/agent-skills)

use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{info, debug, warn};

use crate::config::Config;
use crate::skills::Skill;

/// Skill source types
#[derive(Debug, Clone)]
pub enum SkillSource {
    Anthropic,
    OpenAI,
    Local,
    Npm,
    BuiltIn,
}

/// Skill metadata for discovery
#[derive(Debug, Clone)]
pub struct SkillMetadata {
    pub name: String,
    pub description: String,
    pub source: SkillSource,
    pub path: PathBuf,
    pub compatible_agents: Vec<String>,
}

/// Compatibilization skill for skill discovery and management
pub struct CompatibilizationSkill {
    config: Config,
    skill_registry: HashMap<String, SkillMetadata>,
}

impl CompatibilizationSkill {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        let mut skill = Self {
            config: config.clone(),
            skill_registry: HashMap::new(),
        };
        
        // Initialize skill registry
        skill.discover_local_skills().await?;
        skill.discover_external_skills().await?;
        
        Ok(skill)
    }
    
    /// Discover skills in local directories
    async fn discover_local_skills(&mut self) -> anyhow::Result<()> {
        let local_paths = vec![
            PathBuf::from(".agent/skills"),
            PathBuf::from("src/skills"),
            dirs::home_dir()
                .map(|h| h.join(".config/opencode/skills"))
                .unwrap_or_default(),
        ];
        
        for path in local_paths {
            if path.exists() {
                debug!("Scanning for skills in: {}", path.display());
                self.scan_skill_directory(&path, SkillSource::Local).await?;
            }
        }
        
        Ok(())
    }
    
    /// Discover external skills (Anthropic, OpenAI, npm)
    async fn discover_external_skills(&mut self) -> anyhow::Result<()> {
        // Check for Anthropic skills
        if let Ok(anthropic_path) = std::env::var("ANTHROPIC_SKILLS_PATH") {
            let path = PathBuf::from(anthropic_path);
            if path.exists() {
                info!("Found Anthropic skills at: {}", path.display());
                self.scan_skill_directory(&path, SkillSource::Anthropic).await?;
            }
        }
        
        // Check for OpenAI skills
        if let Ok(openai_path) = std::env::var("OPENAI_SKILLS_PATH") {
            let path = PathBuf::from(openai_path);
            if path.exists() {
                info!("Found OpenAI skills at: {}", path.display());
                self.scan_skill_directory(&path, SkillSource::OpenAI).await?;
            }
        }
        
        // Check npm packages
        if let Some(npm_path) = dirs::home_dir().map(|h| h.join(".config/opencode/skills")) {
            if npm_path.exists() {
                self.scan_skill_directory(&npm_path, SkillSource::Npm).await?;
            }
        }
        
        Ok(())
    }
    
    /// Scan a directory for skills
    async fn scan_skill_directory(
        &mut self,
        path: &PathBuf,
        source: SkillSource,
    ) -> anyhow::Result<()> {
        use tokio::fs;
        
        let mut entries = fs::read_dir(path).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let entry_path = entry.path();
            
            if entry_path.is_dir() {
                let skill_name = entry_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                
                // Check for SKILL.md
                let skill_md = entry_path.join("SKILL.md");
                if skill_md.exists() {
                    let metadata = SkillMetadata {
                        name: skill_name.clone(),
                        description: format!("{:?} skill: {}", source, skill_name),
                        source: source.clone(),
                        path: entry_path.clone(),
                        compatible_agents: vec![
                            "opencode".to_string(),
                            "claude".to_string(),
                            "codex".to_string(),
                            "klp".to_string(),
                        ],
                    };
                    
                    self.skill_registry.insert(skill_name, metadata);
                }
            }
        }
        
        Ok(())
    }
    
    /// Search for skills by keyword
    pub fn search_skills(&self, keyword: &str) -> Vec<&SkillMetadata> {
        self.skill_registry
            .values()
            .filter(|skill| {
                skill.name.contains(keyword) || 
                skill.description.contains(keyword)
            })
            .collect()
    }
    
    /// Get skill by name
    pub fn get_skill(&self, name: &str) -> Option<&SkillMetadata> {
        self.skill_registry.get(name)
    }
    
    /// List all registered skills
    pub fn list_skills(&self) -> Vec<&SkillMetadata> {
        self.skill_registry.values().collect()
    }
    
    /// Check if skill is compatible with an agent
    pub fn is_compatible(&self, skill_name: &str, agent: &str) -> bool {
        if let Some(skill) = self.skill_registry.get(skill_name) {
            skill.compatible_agents.iter().any(|a| a == agent)
        } else {
            false
        }
    }
    
    /// Import skill from remote repository
    pub async fn import_skill(
        &mut self,
        source: &str,
        skill_name: &str,
    ) -> anyhow::Result<()> {
        info!("Importing skill '{}' from '{}'", skill_name, source);
        
        match source {
            "anthropic" => {
                self.clone_anthropic_skill(skill_name).await?;
            }
            "openai" => {
                self.clone_openai_skill(skill_name).await?;
            }
            _ => {
                warn!("Unknown skill source: {}", source);
            }
        }
        
        Ok(())
    }
    
    /// Clone skill from Anthropic repository
    async fn clone_anthropic_skill(&self, skill_name: &str) -> anyhow::Result<()> {
        let repo_url = std::env::var("ANTHROPIC_SKILLS_URL")
            .unwrap_or_else(|_| "https://github.com/anthropics/skills".to_string());
        
        info!("Cloning Anthropic skill '{}' from {}", skill_name, repo_url);
        
        // Implementation would clone specific skill subdirectory
        // For now, this is a placeholder for the actual git operations
        
        Ok(())
    }
    
    /// Clone skill from OpenAI repository
    async fn clone_openai_skill(&self, skill_name: &str) -> anyhow::Result<()> {
        let repo_url = std::env::var("OPENAI_SKILLS_URL")
            .unwrap_or_else(|_| "https://github.com/openai/skills".to_string());
        
        info!("Cloning OpenAI skill '{}' from {}", skill_name, repo_url);
        
        Ok(())
    }
}

#[async_trait]
impl Skill for CompatibilizationSkill {
    fn name(&self) -> &str {
        "compatibilization"
    }
    
    fn description(&self) -> &str {
        "Discover and manage AI skills from Anthropic, OpenAI, and local sources"
    }
    
    async fn execute(&self, args: &[String]) -> anyhow::Result<()> {
        if args.is_empty() {
            println!("Compatibilization Skill - Cross-agent skill discovery");
            println!("\nUsage:");
            println!("  klp skill search <keyword>     - Search for skills");
            println!("  klp skill list                 - List all skills");
            println!("  klp skill import <source>      - Import from Anthropic/OpenAI");
            println!("  klp skill compat <name>        - Check compatibility");
            return Ok(());
        }
        
        match args[0].as_str() {
            "search" => {
                if args.len() > 1 {
                    let keyword = &args[1];
                    let results = self.search_skills(keyword);
                    println!("Found {} skills matching '{}'", results.len(), keyword);
                    for skill in results {
                        println!("  - {} ({:?})", skill.name, skill.source);
                    }
                }
            }
            "list" => {
                let skills = self.list_skills();
                println!("Registered skills: {}", skills.len());
                for skill in skills {
                    println!("  - {} [{:?}]", skill.name, skill.source);
                }
            }
            "compat" => {
                if args.len() > 1 {
                    let skill_name = &args[1];
                    let agents = vec!["opencode", "claude", "codex", "klp"];
                    println!("Compatibility for '{}':", skill_name);
                    for agent in agents {
                        let compatible = self.is_compatible(skill_name, agent);
                        println!("  {} {}", 
                            if compatible { "✅" } else { "❌" },
                            agent
                        );
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
