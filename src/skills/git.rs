//! Git Skill - Git workflow shortcuts

use async_trait::async_trait;
use std::process::Command;
use tracing::info;

use crate::config::Config;
use crate::skills::Skill;

pub struct GitSkill {
    config: Config,
}

impl GitSkill {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        Ok(Self { config: config.clone() })
    }
}

#[async_trait]
impl Skill for GitSkill {
    fn name(&self) -> &str {
        "git"
    }

    fn description(&self) -> &str {
        "Git workflow shortcuts and utilities"
    }

    async fn execute(&self, args: &[String]) -> anyhow::Result<()> {
        if args.is_empty() {
            println!("Git Skill - Available commands:");
            println!("  status           - Show git status");
            println!("  add-all          - Add all files (git add -A)");
            println!("  commit <message> - Commit with message");
            println!("  shortcut <msg>   - Full workflow: status -> add -A -> commit");
            println!("  push             - Push to remote");
            println!("  pull             - Pull from remote");
            return Ok(());
        }

        match args[0].as_str() {
            "status" => {
                let output = Command::new("git")
                    .arg("status")
                    .output()?;
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            "add-all" => {
                let output = Command::new("git")
                    .args(&["add", "-A"])
                    .output()?;
                println!("{}", String::from_utf8_lossy(&output.stdout));
                if !output.stderr.is_empty() {
                    println!("Error: {}", String::from_utf8_lossy(&output.stderr));
                }
            }
            "commit" => {
                if args.len() < 2 {
                    println!("Usage: klp skill git commit <message>");
                    return Ok(());
                }
                let message = args[1..].join(" ");
                let output = Command::new("git")
                    .args(&["commit", "-m", &message])
                    .output()?;
                println!("{}", String::from_utf8_lossy(&output.stdout));
                if !output.stderr.is_empty() {
                    println!("Error: {}", String::from_utf8_lossy(&output.stderr));
                }
            }
            "shortcut" => {
                if args.len() < 2 {
                    println!("Usage: klp skill git shortcut <commit-message>");
                    return Ok(());
                }
                let message = args[1..].join(" ");

                println!("🔍 Checking git status...");
                let status_output = Command::new("git")
                    .arg("status")
                    .output()?;
                println!("{}", String::from_utf8_lossy(&status_output.stdout));

                println!("📁 Adding all files...");
                let add_output = Command::new("git")
                    .args(&["add", "-A"])
                    .output()?;
                if !add_output.stderr.is_empty() {
                    println!("Add error: {}", String::from_utf8_lossy(&add_output.stderr));
                }

                println!("💾 Committing with message: {}", message);
                let commit_output = Command::new("git")
                    .args(&["commit", "-m", &message])
                    .output()?;
                println!("{}", String::from_utf8_lossy(&commit_output.stdout));
                if !commit_output.stderr.is_empty() {
                    println!("Commit error: {}", String::from_utf8_lossy(&commit_output.stderr));
                }

                println!("✅ Git workflow completed!");
            }
            "push" => {
                let output = Command::new("git")
                    .arg("push")
                    .output()?;
                println!("{}", String::from_utf8_lossy(&output.stdout));
                if !output.stderr.is_empty() {
                    println!("Error: {}", String::from_utf8_lossy(&output.stderr));
                }
            }
            "pull" => {
                let output = Command::new("git")
                    .arg("pull")
                    .output()?;
                println!("{}", String::from_utf8_lossy(&output.stdout));
                if !output.stderr.is_empty() {
                    println!("Error: {}", String::from_utf8_lossy(&output.stderr));
                }
            }
            _ => {
                println!("Unknown git command: {}", args[0]);
                println!("Available commands: status, add-all, commit, shortcut, push, pull");
            }
        }

        Ok(())
    }
}
