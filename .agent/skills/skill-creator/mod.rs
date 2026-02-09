//! Skill Creator - Generate new skill templates
//! 
//! This skill creates properly structured skill directories following
//! the skill-creator specification format.

use async_trait::async_trait;
use std::fs;
use std::path::PathBuf;
use tracing::{info, error};

use crate::config::Config;
use crate::skills::Skill;

pub struct SkillCreatorSkill {
    config: Config,
}

impl SkillCreatorSkill {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        Ok(Self { config: config.clone() })
    }
    
    /// Create a new skill with the given name
    async fn create_skill(&self, name: &str, skill_type: &str) -> anyhow::Result<()> {
        let skills_dir = PathBuf::from(".agent/skills").join(name);
        
        // Create directory structure
        fs::create_dir_all(&skills_dir)?;
        fs::create_dir_all(skills_dir.join("scripts"))?;
        fs::create_dir_all(skills_dir.join("references"))?;
        fs::create_dir_all(skills_dir.join("assets"))?;
        
        // Generate SKILL.md
        let skill_md = self.generate_skill_md(name);
        fs::write(skills_dir.join("SKILL.md"), skill_md)?;
        
        // Generate mod.rs if rust type
        if skill_type == "rust" {
            let mod_rs = self.generate_mod_rs(name);
            fs::write(skills_dir.join("mod.rs"), mod_rs)?;
        }
        
        info!("Created skill '{}' at {:?}", name, skills_dir);
        println!("✅ Skill '{}' created successfully!", name);
        println!("📁 Location: .agent/skills/{}/", name);
        println!("🎯 Purpose: Brief description here");
        println!("💡 Edit SKILL.md to customize your skill");
        
        Ok(())
    }
    
    /// Generate SKILL.md template
    fn generate_skill_md(&self, name: &str) -> String {
        format!(r#"---
name: {}
description: Brief description of what this skill does and when to use it. Include trigger phrases.
---

# {}

## Overview

Brief explanation of the skill's purpose and capabilities.

## Quick Start

```bash
# Example command
klp {} <command> [args]
```

## Commands

- `command1` - Description of what this does
- `command2` - Description of what this does

## Examples

### Example 1
```bash
klp {} do-something
```

### Example 2
```bash
klp {} do-something --option
```

## References

- Source code: `src/skills/{}/mod.rs`
"#, name, name, name, name, name, name)
    }
    
    /// Generate mod.rs template
    fn generate_mod_rs(&self, name: &str) -> String {
        let struct_name = name.split('-')
            .map(|s| {
                let mut c = s.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            })
            .collect::<String>() + "Skill";
        
        format!(r#"//! {} Skill
//! 
//! Brief description of this skill's functionality.

use async_trait::async_trait;
use tracing::info;

use crate::config::Config;
use crate::skills::Skill;

pub struct {} {{
    config: Config,
}}

impl {} {{
    pub async fn new(config: &Config) -> anyhow::Result<Self> {{
        Ok(Self {{ config: config.clone() }})
    }}
}}

#[async_trait]
impl Skill for {} {{
    fn name(&self) -> &str {{
        "{}"
    }}
    
    fn description(&self) -> &str {{
        "Brief description of what this skill does"
    }}
    
    async fn execute(&self, args: &[String]) -> anyhow::Result<()> {{
        println!("{} - Skill execution");
        println!("Args: {{:?}}", args);
        Ok(())
    }}
}}
"#, name, struct_name, struct_name, struct_name, name, struct_name)
    }
    
    /// Convert existing file to skill format
    async fn convert_file(&self, file_path: &str, output_name: &str) -> anyhow::Result<()> {
        let source = fs::read_to_string(file_path)?;
        let skills_dir = PathBuf::from(".agent/skills").join(output_name);
        
        fs::create_dir_all(&skills_dir)?;
        fs::create_dir_all(skills_dir.join("references"))?;
        
        // Write original file to references
        let file_name = PathBuf::from(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("source");
        fs::write(skills_dir.join("references").join(file_name), &source)?;
        
        // Create SKILL.md
        let skill_md = format!(r#"---
name: {}
description: Auto-generated skill from {}
---

# {}

## Overview

This skill was auto-generated from `{}`.

## Source Code

See: `references/{}`

## Usage

```bash
klp {} [args]
```
"#, output_name, file_path, output_name, file_path, file_name, output_name);
        
        fs::write(skills_dir.join("SKILL.md"), skill_md)?;
        
        info!("Converted '{}' to skill '{}'", file_path, output_name);
        println!("✅ Converted '{}' to skill '{}'", file_path, output_name);
        
        Ok(())
    }
}

#[async_trait]
impl Skill for SkillCreatorSkill {
    fn name(&self) -> &str {
        "skill-creator"
    }
    
    fn description(&self) -> &str {
        "Generate new skill templates following skill-creator specification"
    }
    
    async fn execute(&self, args: &[String]) -> anyhow::Result<()> {
        if args.is_empty() {
            println!("Skill Creator - Generate new skill templates");
            println!("\nUsage:");
            println!("  klp skill-creator create <name> [--type rust|markdown]");
            println!("  klp skill-creator convert <file> --output <name>");
            return Ok(());
        }
        
        match args[0].as_str() {
            "create" => {
                if args.len() > 1 {
                    let name = &args[1];
                    let skill_type = args.iter()
                        .position(|a| a == "--type")
                        .and_then(|i| args.get(i + 1))
                        .map(|s| s.as_str())
                        .unwrap_or("rust");
                    
                    self.create_skill(name, skill_type).await?;
                } else {
                    error!("Skill name required");
                }
            }
            "convert" => {
                if args.len() > 1 {
                    let file_path = &args[1];
                    let output_name = args.iter()
                        .position(|a| a == "--output")
                        .and_then(|i| args.get(i + 1))
                        .map(|s| s.as_str())
                        .unwrap_or("converted-skill");
                    
                    self.convert_file(file_path, output_name).await?;
                } else {
                    error!("File path required");
                }
            }
            _ => {
                println!("Unknown command: {}", args[0]);
            }
        }
        
        Ok(())
    }
}
