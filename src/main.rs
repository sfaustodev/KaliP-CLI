//! KLP - Kali Language Processor
//! 
//! An AI-powered CLI agent for ethical hacking, OSINT, pentesting, and system control.
//! Built exclusively for Kali Linux x86_64.

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use tracing::{info, warn};

use klp::config::Config;
use klp::skills::SkillManager;
use klp::subagents::SubagentManager;
use klp::security::SecurityManager;

#[derive(Parser)]
#[command(
    name = "klp",
    about = "Kali Language Processor - AI-powered CLI agent for ethical hacking",
    version = "0.1.0",
    author = "KLP Team <klp@kalilinux.org>"
)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(short, long, global = true)]
    verbose: bool,
    
    #[arg(short, long, global = true)]
    stealth: bool,
}

#[derive(Subcommand)]
enum Commands {
    Code {
        #[arg(value_name = "FILE")]
        file: Option<PathBuf>,
    },
    
    Generate {
        #[arg(value_name = "PROMPT")]
        prompt: String,
        
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
        
        #[arg(short, long, value_enum, default_value = "grok")]
        api: ApiChoice,
    },
    
    Skill {
        #[command(subcommand)]
        command: SkillCommands,
    },
    
    Agent {
        #[command(subcommand)]
        command: AgentCommands,
    },
    
    Security {
        #[command(subcommand)]
        command: SecurityCommands,
    },
    
    Chrome {
        #[command(subcommand)]
        command: ChromeCommands,
    },
    
    Connect {
        #[arg(value_name = "TARGET")]
        target: String,
        
        #[arg(short, long, value_enum, default_value = "ssh")]
        protocol: Protocol,
        
        #[arg(short, long)]
        port: Option<u16>,
    },
    
    Report {
        #[arg(short, long, value_enum, default_value = "full")]
        kind: ReportType,
        
        #[arg(short, long, default_value = "klp_report.pdf")]
        output: PathBuf,
    },
    
    Ask {
        #[arg(value_name = "COMMAND")]
        command: String,
        
        #[arg(short, long)]
        yes: bool,
    },
    
    Init {
        #[arg(short, long)]
        force: bool,
    },
    
    Config {
        #[arg(short, long)]
        edit: bool,
    },
}

#[derive(Subcommand)]
enum SkillCommands {
    Add {
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    List,
    Remove {
        #[arg(value_name = "NAME")]
        name: String,
    },
    Run {
        #[arg(value_name = "NAME")]
        name: String,
        #[arg(value_name = "ARGS")]
        args: Vec<String>,
    },
}

#[derive(Subcommand)]
enum AgentCommands {
    Start {
        #[arg(value_name = "NAME")]
        name: String,
    },
    Stop {
        #[arg(value_name = "NAME")]
        name: String,
    },
    List,
    Status {
        #[arg(value_name = "NAME")]
        name: String,
    },
}

#[derive(Subcommand)]
enum SecurityCommands {
    Stealth {
        #[arg(value_name = "MODE")]
        mode: String,
    },
    Defcon {
        #[arg(value_name = "LEVEL")]
        level: u8,
    },
    Logs {
        #[command(subcommand)]
        command: LogCommands,
    },
    Wipe {
        #[arg(value_name = "TARGET")]
        target: String,
    },
}

#[derive(Subcommand)]
enum LogCommands {
    Encrypt,
    Decrypt,
    Status,
}

#[derive(Subcommand)]
enum ChromeCommands {
    Open,
    Auto {
        #[arg(value_name = "ACTION")]
        action: String,
    },
    Schedule {
        #[arg(value_name = "TASK")]
        task: String,
        #[arg(short, long)]
        interval: u32,
    },
}

#[derive(ValueEnum, Clone, Debug)]
enum ApiChoice {
    Grok,
    Groq,
    Openai,
    Local,
}

#[derive(ValueEnum, Clone, Debug)]
enum Protocol {
    Ssh,
    Rdp,
    Vnc,
    Icp,
    Ncp,
}

#[derive(ValueEnum, Clone, Debug)]
enum ReportType {
    Full,
    Osint,
    Pentest,
    System,
    Logs,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    // Initialize tracing
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(if cli.verbose {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)?;
    
    info!("KLP v0.1.0 - Kali Language Processor");
    info!("Platform: Kali Linux x86_64");
    
    // Load configuration
    let config = Config::load().await?;
    
    // Initialize security manager
    let security = SecurityManager::new(&config)?;
    
    // Handle stealth mode
    if cli.stealth {
        security.enable_stealth_mode().await?;
        info!("Stealth mode enabled");
    }
    
    // Execute command
    match cli.command {
        Commands::Code { file } => {
            info!("Launching TUI code editor");
            klp::tui::run_tui(file, config).await?;
        }
        
        Commands::Generate { prompt, output, api } => {
            info!("Generating code with prompt: {}", prompt);
            let api_choice = match api {
                ApiChoice::Grok => klp::codegen::ApiChoice::Grok,
                ApiChoice::Groq => klp::codegen::ApiChoice::Groq,
                ApiChoice::Openai => klp::codegen::ApiChoice::Openai,
                ApiChoice::Local => klp::codegen::ApiChoice::Local,
            };
            let code = klp::codegen::generate_code(&prompt, api_choice, &config).await?;
            
            if let Some(output_path) = output {
                tokio::fs::write(&output_path, code).await?;
                info!("Code written to: {}", output_path.display());
            } else {
                println!("{}", code);
            }
        }
        
        Commands::Skill { command } => {
            let skill_manager = SkillManager::new(&config).await?;
            
            match command {
                SkillCommands::Add { file } => {
                    skill_manager.add_skill(&file).await?;
                    info!("Skill added from: {}", file.display());
                }
                SkillCommands::List => {
                    let skills = skill_manager.list_skills().await?;
                    println!("Installed Skills:");
                    for skill in skills {
                        println!("  - {}", skill);
                    }
                }
                SkillCommands::Remove { name } => {
                    skill_manager.remove_skill(&name).await?;
                    info!("Skill removed: {}", name);
                }
                SkillCommands::Run { name, args } => {
                    skill_manager.run_skill(&name, &args).await?;
                }
            }
        }
        
        Commands::Agent { command } => {
            let agent_manager = SubagentManager::new(&config).await?;
            
            match command {
                AgentCommands::Start { name } => {
                    agent_manager.start_agent(&name).await?;
                    info!("Agent started: {}", name);
                }
                AgentCommands::Stop { name } => {
                    agent_manager.stop_agent(&name).await?;
                    info!("Agent stopped: {}", name);
                }
                AgentCommands::List => {
                    let agents = agent_manager.list_agents().await?;
                    println!("Running Agents:");
                    for agent in agents {
                        println!("  - {}", agent);
                    }
                }
                AgentCommands::Status { name } => {
                    let status = agent_manager.agent_status(&name).await?;
                    println!("Agent {} status: {}", name, status);
                }
            }
        }
        
        Commands::Security { command } => {
            match command {
                SecurityCommands::Stealth { mode } => {
                    match mode.as_str() {
                        "on" => {
                            security.enable_stealth_mode().await?;
                            info!("Stealth mode enabled");
                        }
                        "off" => {
                            security.disable_stealth_mode().await?;
                            info!("Stealth mode disabled");
                        }
                        _ => {
                            warn!("Invalid stealth mode. Use 'on' or 'off'");
                        }
                    }
                }
                SecurityCommands::Defcon { level } => {
                    security.initiate_defcon(level).await?;
                }
                SecurityCommands::Logs { command } => {
                    match command {
                        LogCommands::Encrypt => {
                            security.encrypt_logs().await?;
                            info!("Logs encrypted");
                        }
                        LogCommands::Decrypt => {
                            security.decrypt_logs().await?;
                            info!("Logs decrypted");
                        }
                        LogCommands::Status => {
                            let status = security.log_status().await?;
                            println!("Log status: {}", status);
                        }
                    }
                }
                SecurityCommands::Wipe { target } => {
                    security.wipe_traces(&target).await?;
                    info!("Wiped: {}", target);
                }
            }
        }
        
        Commands::Chrome { command } => {
            let chrome = klp::chrome::ChromeController::new(&config).await?;
            
            match command {
                ChromeCommands::Open => {
                    chrome.open_browser().await?;
                    info!("Chrome opened");
                }
                ChromeCommands::Auto { action } => {
                    chrome.execute_action(&action).await?;
                    info!("Chrome action executed: {}", action);
                }
                ChromeCommands::Schedule { task, interval } => {
                    chrome.schedule_task(&task, interval).await?;
                    info!("Scheduled {} every {} minutes", task, interval);
                }
            }
        }
        
        Commands::Connect { target, protocol, port } => {
            let connector = klp::connect::Connector::new(&config).await?;
            connector.connect(&target, protocol, port).await?;
        }
        
        Commands::Report { kind, output } => {
            let report = klp::report::ReportGenerator::new(&config).await?;
            let report_type = match kind {
                ReportType::Full => klp::report::ReportType::Full,
                ReportType::Osint => klp::report::ReportType::Osint,
                ReportType::Pentest => klp::report::ReportType::Pentest,
                ReportType::System => klp::report::ReportType::System,
                ReportType::Logs => klp::report::ReportType::Logs,
            };
            report.generate(report_type, &output).await?;
            info!("Report generated: {}", output.display());
        }
        
        Commands::Ask { command, yes } => {
            let nlp = klp::nlp::NaturalLanguageProcessor::new(&config).await?;
            nlp.process(&command, !yes).await?;
        }
        
        Commands::Init { force } => {
            Config::initialize(force).await?;
            info!("KLP initialized successfully");
            println!("Configuration directory: ~/.klp/");
            println!("Edit ~/.klp/config.toml to configure API keys");
        }
        
        Commands::Config { edit } => {
            if edit {
                Config::edit().await?;
            } else {
                let config_path = Config::path()?;
                println!("Configuration file: {}", config_path.display());
                let content = tokio::fs::read_to_string(&config_path).await?;
                println!("\n{}", content);
            }
        }
    }
    
    Ok(())
}
