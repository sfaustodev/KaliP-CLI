//! Git Operations Skill - Repository management and version control

use async_trait::async_trait;
use std::process::Command;
use tracing::{info, warn};

use crate::config::Config;
use crate::skills::Skill;

pub struct GitOperationsSkill {
    config: Config,
}

impl GitOperationsSkill {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        Ok(Self { config: config.clone() })
    }
    
    /// Execute git command
    fn git_cmd(&self, args: &[&str]) -> anyhow::Result<String> {
        info!("Executing: git {}", args.join(" "));
        
        let output = Command::new("git")
            .args(args)
            .output()?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        if !output.status.success() {
            return Err(anyhow::anyhow!("Git error: {}", stderr));
        }
        
        Ok(stdout.to_string())
    }
    
    /// Clone repository
    fn clone(&self, url: &str, dir: Option<&str>) -> anyhow::Result<()> {
        let args = if let Some(d) = dir {
            vec!["clone", url, d]
        } else {
            vec!["clone", url]
        };
        
        let output = self.git_cmd(&args)?;
        println!("{}", output);
        Ok(())
    }
    
    /// Get status
    fn status(&self) -> anyhow::Result<()> {
        let output = self.git_cmd(&["status"])?;
        println!("{}", output);
        Ok(())
    }
    
    /// Stage files
    fn add(&self, files: &[String]) -> anyhow::Result<()> {
        let mut args = vec!["add"];
        for file in files {
            args.push(file);
        }
        let output = self.git_cmd(&args)?;
        println!("{}", output);
        Ok(())
    }
    
    /// Commit changes
    fn commit(&self, message: &str) -> anyhow::Result<()> {
        let output = self.git_cmd(&["commit", "-m", message])?;
        println!("{}", output);
        Ok(())
    }
    
    /// Push to remote
    fn push(&self, remote: Option<&str>, branch: Option<&str>) -> anyhow::Result<()> {
        let mut args = vec!["push"];
        if let Some(r) = remote {
            args.push(r);
        }
        if let Some(b) = branch {
            args.push(b);
        }
        
        let output = self.git_cmd(&args)?;
        println!("{}", output);
        Ok(())
    }
    
    /// Pull from remote
    fn pull(&self, remote: Option<&str>, branch: Option<&str>) -> anyhow::Result<()> {
        let mut args = vec!["pull"];
        if let Some(r) = remote {
            args.push(r);
        }
        if let Some(b) = branch {
            args.push(b);
        }
        
        let output = self.git_cmd(&args)?;
        println!("{}", output);
        Ok(())
    }
    
    /// List branches
    fn branch_list(&self) -> anyhow::Result<()> {
        let output = self.git_cmd(&["branch", "-a"])?;
        println!("{}", output);
        Ok(())
    }
    
    /// Create branch
    fn branch_create(&self, name: &str) -> anyhow::Result<()> {
        let output = self.git_cmd(&["checkout", "-b", name])?;
        println!("{}", output);
        Ok(())
    }
    
    /// Switch branch
    fn branch_switch(&self, name: &str) -> anyhow::Result<()> {
        let output = self.git_cmd(&["checkout", name])?;
        println!("{}", output);
        Ok(())
    }
    
    /// Show log
    fn log(&self, args: &[String]) -> anyhow::Result<()> {
        let mut git_args = vec!["log"];
        for arg in args {
            git_args.push(arg);
        }
        
        let output = self.git_cmd(&git_args)?;
        println!("{}", output);
        Ok(())
    }
    
    /// Show diff
    fn diff(&self, files: Option<&String>) -> anyhow::Result<()> {
        let args = if let Some(f) = files {
            vec!["diff", f]
        } else {
            vec!["diff"]
        };
        
        let output = self.git_cmd(&args)?;
        println!("{}", output);
        Ok(())
    }
    
    /// Stash changes
    fn stash(&self, message: Option<&str>) -> anyhow::Result<()> {
        let args = if let Some(m) = message {
            vec!["stash", "push", "-m", m]
        } else {
            vec!["stash"]
        };
        
        let output = self.git_cmd(&args)?;
        println!("{}", output);
        Ok(())
    }
    
    /// Pop stash
    fn stash_pop(&self) -> anyhow::Result<()> {
        let output = self.git_cmd(&["stash", "pop"])?;
        println!("{}", output);
        Ok(())
    }
}

#[async_trait]
impl Skill for GitOperationsSkill {
    fn name(&self) -> &str {
        "git-operations"
    }
    
    fn description(&self) -> &str {
        "Git repository operations and version control"
    }
    
    async fn execute(&self, args: &[String]) -> anyhow::Result<()> {
        if args.is_empty() {
            println!("Git Operations Skill");
            println!("\nUsage:");
            println!("  klp git clone <url> [dir]    - Clone repository");
            println!("  klp git status               - Show status");
            println!("  klp git add <files>          - Stage files");
            println!("  klp git commit -m <msg>      - Commit changes");
            println!("  klp git push [remote]        - Push to remote");
            println!("  klp git pull [remote]        - Pull from remote");
            println!("  klp git branch list          - List branches");
            println!("  klp git branch create <name> - Create branch");
            println!("  klp git log [options]        - Show log");
            println!("  klp git diff [files]         - Show diff");
            println!("  klp git stash [message]      - Stash changes");
            return Ok(());
        }
        
        match args[0].as_str() {
            "clone" => {
                if args.len() > 1 {
                    let dir = args.get(2).map(|s| s.as_str());
                    self.clone(&args[1], dir)?;
                } else {
                    warn!("URL required");
                }
            }
            "status" => {
                self.status()?;
            }
            "add" => {
                if args.len() > 1 {
                    self.add(&args[1..].to_vec())?;
                }
            }
            "commit" => {
                // Parse -m flag
                if let Some(pos) = args.iter().position(|a| a == "-m") {
                    if let Some(msg) = args.get(pos + 1) {
                        self.commit(msg)?;
                    }
                } else if args.len() > 1 {
                    self.commit(&args[1..].join(" "))?;
                }
            }
            "push" => {
                let remote = args.get(1).map(|s| s.as_str());
                let branch = args.get(2).map(|s| s.as_str());
                self.push(remote, branch)?;
            }
            "pull" => {
                let remote = args.get(1).map(|s| s.as_str());
                let branch = args.get(2).map(|s| s.as_str());
                self.pull(remote, branch)?;
            }
            "branch" => {
                if args.len() > 1 {
                    match args[1].as_str() {
                        "list" => self.branch_list()?,
                        "create" => {
                            if let Some(name) = args.get(2) {
                                self.branch_create(name)?;
                            }
                        }
                        "switch" => {
                            if let Some(name) = args.get(2) {
                                self.branch_switch(name)?;
                            }
                        }
                        _ => println!("Unknown branch command: {}", args[1]),
                    }
                } else {
                    self.branch_list()?;
                }
            }
            "log" => {
                self.log(&args[1..].to_vec())?;
            }
            "diff" => {
                let files = args.get(1);
                self.diff(files)?;
            }
            "stash" => {
                if args.len() > 1 && args[1] == "pop" {
                    self.stash_pop()?;
                } else {
                    let message = args.get(1).map(|s| s.as_str());
                    self.stash(message)?;
                }
            }
            _ => {
                println!("Unknown command: {}", args[0]);
            }
        }
        
        Ok(())
    }
}
