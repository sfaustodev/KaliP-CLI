//! Monitoring Skill - System and application monitoring

use async_trait::async_trait;
use std::collections::HashMap;
use tracing::{info, warn};

use crate::config::Config;
use crate::skills::Skill;

/// System metrics
#[derive(Debug, Clone)]
pub struct SystemMetrics {
    pub cpu_percent: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub disk_used: u64,
    pub disk_total: u64,
    pub load_average: f32,
}

/// Alert condition
#[derive(Debug, Clone)]
pub struct Alert {
    pub id: String,
    pub condition: String,
    pub threshold: f32,
    pub active: bool,
}

pub struct MonitoringSkill {
    config: Config,
    alerts: HashMap<String, Alert>,
}

impl MonitoringSkill {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        Ok(Self {
            config: config.clone(),
            alerts: HashMap::new(),
        })
    }
    
    /// Get system metrics
    fn get_system_metrics(&self) -> anyhow::Result<SystemMetrics> {
        // Placeholder implementation
        // In a real implementation, this would use sysinfo crate
        
        Ok(SystemMetrics {
            cpu_percent: 23.5,
            memory_used: 8_589_934_592, // 8GB
            memory_total: 17_179_869_184, // 16GB
            disk_used: 120_000_000_000, // 120GB
            disk_total: 500_000_000_000, // 500GB
            load_average: 1.23,
        })
    }
    
    /// Display system metrics
    fn show_system(&self) -> anyhow::Result<()> {
        let metrics = self.get_system_metrics()?;
        
        let mem_used_gb = metrics.memory_used as f64 / 1_073_741_824.0;
        let mem_total_gb = metrics.memory_total as f64 / 1_073_741_824.0;
        let mem_percent = (mem_used_gb / mem_total_gb) * 100.0;
        
        let disk_used_gb = metrics.disk_used as f64 / 1_000_000_000.0;
        let disk_total_gb = metrics.disk_total as f64 / 1_000_000_000.0;
        let disk_percent = (disk_used_gb / disk_total_gb) * 100.0;
        
        println!("System Metrics:");
        println!("  CPU: {:.1}%", metrics.cpu_percent);
        println!("  Memory: {:.1}GB / {:.1}GB ({:.0}%)", 
            mem_used_gb, mem_total_gb, mem_percent);
        println!("  Disk: {:.0}GB / {:.0}GB ({:.0}%)", 
            disk_used_gb, disk_total_gb, disk_percent);
        println!("  Load Average: {:.2}", metrics.load_average);
        
        Ok(())
    }
    
    /// List processes
    fn list_processes(&self) -> anyhow::Result<()> {
        use std::process::Command;
        
        let output = if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
            Command::new("ps")
                .args(["aux", "--sort=-%cpu"])
                .output()?
        } else {
            Command::new("tasklist").output()?
        };
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Show header and top 10 processes
        let lines: Vec<&str> = stdout.lines().collect();
        if lines.len() > 0 {
            println!("{}", lines[0]); // Header
        }
        for line in lines.iter().skip(1).take(10) {
            println!("{}", line);
        }
        
        Ok(())
    }
    
    /// Monitor specific process
    fn monitor_process(&self, name: &str) -> anyhow::Result<()> {
        use std::process::Command;
        
        info!("Monitoring process: {}", name);
        
        let output = Command::new("pgrep")
            .arg("-f")
            .arg(name)
            .output()?;
        
        if output.status.success() {
            let pids = String::from_utf8_lossy(&output.stdout);
            println!("Processes matching '{}':", name);
            for pid in pids.lines() {
                println!("  PID: {}", pid);
            }
        } else {
            println!("No processes found matching '{}'", name);
        }
        
        Ok(())
    }
    
    /// Health check
    fn health_check(&self) -> anyhow::Result<()> {
        println!("System Health Check:");
        
        // Check disk space
        let metrics = self.get_system_metrics()?;
        let disk_percent = (metrics.disk_used as f64 / metrics.disk_total as f64) * 100.0;
        
        if disk_percent > 90.0 {
            println!("  ❌ Disk usage critical: {:.0}%", disk_percent);
        } else if disk_percent > 80.0 {
            println!("  ⚠️  Disk usage warning: {:.0}%", disk_percent);
        } else {
            println!("  ✅ Disk usage normal: {:.0}%", disk_percent);
        }
        
        // Check memory
        let mem_percent = (metrics.memory_used as f64 / metrics.memory_total as f64) * 100.0;
        if mem_percent > 90.0 {
            println!("  ❌ Memory usage critical: {:.0}%", mem_percent);
        } else {
            println!("  ✅ Memory usage normal: {:.0}%", mem_percent);
        }
        
        // Check CPU
        if metrics.cpu_percent > 90.0 {
            println!("  ⚠️  CPU usage high: {:.0}%", metrics.cpu_percent);
        } else {
            println!("  ✅ CPU usage normal: {:.0}%", metrics.cpu_percent);
        }
        
        Ok(())
    }
    
    /// Add alert
    fn add_alert(&mut self, condition: &str, threshold: f32) {
        let id = format!("alert-{}", self.alerts.len() + 1);
        let alert = Alert {
            id: id.clone(),
            condition: condition.to_string(),
            threshold,
            active: true,
        };
        
        self.alerts.insert(id.clone(), alert);
        println!("Alert added: {} - {} > {:.0}%", id, condition, threshold);
    }
    
    /// List alerts
    fn list_alerts(&self) {
        println!("Active Alerts:");
        for (id, alert) in &self.alerts {
            let status = if alert.active { "Active" } else { "Inactive" };
            println!("  {}: {} > {:.0}% [{}]", 
                id, alert.condition, alert.threshold, status);
        }
    }
}

#[async_trait]
impl Skill for MonitoringSkill {
    fn name(&self) -> &str {
        "monitoring"
    }
    
    fn description(&self) -> &str {
        "System monitoring and alerting"
    }
    
    async fn execute(&self, args: &[String]) -> anyhow::Result<()> {
        if args.is_empty() {
            println!("Monitoring Skill");
            println!("\nUsage:");
            println!("  klp monitor system             - Show system metrics");
            println!("  klp monitor process <name>     - Monitor process");
            println!("  klp monitor process list       - List processes");
            println!("  klp monitor health             - Health check");
            println!("  klp monitor watch              - Continuous monitoring");
            println!("  klp monitor alert add <cond>   - Add alert");
            println!("  klp monitor alert list         - List alerts");
            return Ok(());
        }
        
        match args[0].as_str() {
            "system" => {
                self.show_system()?;
            }
            "process" => {
                if args.len() > 1 {
                    if args[1] == "list" {
                        self.list_processes()?;
                    } else {
                        self.monitor_process(&args[1])?;
                    }
                } else {
                    self.list_processes()?;
                }
            }
            "health" => {
                self.health_check()?;
            }
            "watch" => {
                println!("Continuous monitoring mode (placeholder)");
                println!("Use --interval <seconds> to set update frequency");
            }
            "alert" => {
                if args.len() > 1 {
                    match args[1].as_str() {
                        "add" => {
                            if args.len() > 2 {
                                println!("Adding alert: {}", args[2]);
                            }
                        }
                        "list" => {
                            self.list_alerts();
                        }
                        _ => println!("Unknown alert command: {}", args[1]),
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
