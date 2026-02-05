//! Code generation module for KLP
//! 
//! Handles online API-based and offline local model code generation

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tracing::{debug, error, info, warn};

use crate::config::{ApiKey, Config};

/// API choices for code generation
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ApiChoice {
    Grok,
    Groq,
    Openai,
    Local,
}

impl ApiChoice {
    pub fn next(self) -> Self {
        match self {
            ApiChoice::Grok => ApiChoice::Groq,
            ApiChoice::Groq => ApiChoice::Openai,
            ApiChoice::Openai => ApiChoice::Local,
            ApiChoice::Local => ApiChoice::Grok,
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            ApiChoice::Grok => "GROK",
            ApiChoice::Groq => "GROQ",
            ApiChoice::Openai => "OPENAI",
            ApiChoice::Local => "LOCAL",
        }
    }
}

/// Generate code based on prompt and selected API
pub async fn generate_code(
    prompt: &str,
    api_choice: ApiChoice,
    config: &Config,
) -> anyhow::Result<String> {
    info!("Generating code with {:?} API", api_choice);
    
    let code = match api_choice {
        ApiChoice::Grok => generate_with_grok(prompt, config).await,
        ApiChoice::Groq => generate_with_groq(prompt, config).await,
        ApiChoice::Openai => generate_with_openai(prompt, config).await,
        ApiChoice::Local => generate_with_local(prompt, config).await,
    };
    
    // Fallback to local if online fails
    if code.is_err() && config.codegen.use_local_fallback {
        warn!("Online generation failed, falling back to local model");
        generate_with_local(prompt, config).await
    } else {
        code
    }
}

/// Generate code using Grok API
async fn generate_with_grok(prompt: &str, config: &Config) -> anyhow::Result<String> {
    let api_key = config.apis.grok.as_ref()
        .ok_or_else(|| anyhow::anyhow!("Grok API key not configured"))?;
    
    let client = create_http_client(config)?;
    
    let endpoint = api_key.endpoint.as_ref()
        .map(|s| s.as_str())
        .unwrap_or("https://api.x.ai/v1/chat/completions");
    
    let model = api_key.model.as_ref()
        .map(|s| s.as_str())
        .unwrap_or("grok-beta");
    
    let request = serde_json::json!({
        "model": model,
        "messages": [
            {
                "role": "system",
                "content": "You are KLP CodeGen, a code generation assistant for ethical hacking and system control on Kali Linux. Generate clean, secure, well-commented Rust code."
            },
            {
                "role": "user",
                "content": prompt
            }
        ],
        "max_tokens": config.codegen.max_tokens,
        "temperature": config.codegen.temperature,
        "top_p": config.codegen.top_p,
    });
    
    let response = client
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", api_key.key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;
    
    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow::anyhow!("Grok API error: {}", error_text));
    }
    
    let api_response: serde_json::Value = response.json().await?;
    let code = api_response["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid response format from Grok"))?
        .to_string();
    
    Ok(extract_code(&code))
}

/// Generate code using Groq API
async fn generate_with_groq(prompt: &str, config: &Config) -> anyhow::Result<String> {
    let api_key = config.apis.groq.as_ref()
        .ok_or_else(|| anyhow::anyhow!("Groq API key not configured"))?;
    
    let client = create_http_client(config)?;
    
    let endpoint = "https://api.groq.com/openai/v1/chat/completions";
    let model = api_key.model.as_ref()
        .map(|s| s.as_str())
        .unwrap_or("llama3-8b-8192");
    
    let request = serde_json::json!({
        "model": model,
        "messages": [
            {
                "role": "system",
                "content": "You are KLP CodeGen, a code generation assistant for ethical hacking and system control on Kali Linux. Generate clean, secure, well-commented Rust code."
            },
            {
                "role": "user",
                "content": prompt
            }
        ],
        "max_tokens": config.codegen.max_tokens,
        "temperature": config.codegen.temperature,
    });
    
    let response = client
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", api_key.key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;
    
    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow::anyhow!("Groq API error: {}", error_text));
    }
    
    let api_response: serde_json::Value = response.json().await?;
    let code = api_response["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid response format from Groq"))?
        .to_string();
    
    Ok(extract_code(&code))
}

/// Generate code using OpenAI API
async fn generate_with_openai(prompt: &str, config: &Config) -> anyhow::Result<String> {
    let api_key = config.apis.openai.as_ref()
        .ok_or_else(|| anyhow::anyhow!("OpenAI API key not configured"))?;
    
    let client = create_http_client(config)?;
    
    let endpoint = "https://api.openai.com/v1/chat/completions";
    let model = api_key.model.as_ref()
        .map(|s| s.as_str())
        .unwrap_or("gpt-4");
    
    let request = serde_json::json!({
        "model": model,
        "messages": [
            {
                "role": "system",
                "content": "You are KLP CodeGen, a code generation assistant for ethical hacking and system control on Kali Linux. Generate clean, secure, well-commented Rust code."
            },
            {
                "role": "user",
                "content": prompt
            }
        ],
        "max_tokens": config.codegen.max_tokens,
        "temperature": config.codegen.temperature,
    });
    
    let response = client
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", api_key.key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;
    
    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
    }
    
    let api_response: serde_json::Value = response.json().await?;
    let code = api_response["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid response format from OpenAI"))?
        .to_string();
    
    Ok(extract_code(&code))
}

/// Generate code using local model (Phi-3-mini or Llama3-8B 4-bit)
async fn generate_with_local(prompt: &str, config: &Config) -> anyhow::Result<String> {
    info!("Using local model for code generation");
    
    // Check if local model path is configured
    if let Some(ref model_path) = config.codegen.local_model_path {
        if model_path.exists() {
            // Try to use llama.cpp or similar local inference
            return run_local_inference(prompt, model_path).await;
        }
    }
    
    // Fallback: use system llama.cpp if available
    let output = tokio::process::Command::new("which")
        .arg("llama-cli")
        .output()
        .await?;
    
    if output.status.success() {
        // Use llama.cpp CLI
        return run_llama_cpp(prompt).await;
    }
    
    // Last resort: provide a template response
    warn!("No local model found, returning template");
    Ok(generate_template_response(prompt))
}

async fn run_local_inference(
    prompt: &str,
    model_path: &std::path::Path,
) -> anyhow::Result<String> {
    // This would integrate with a local inference engine
    // For now, return a placeholder
    Ok(format!(
        "// Generated by local model at {}\n// Prompt: {}\n\nfn generated_function() {{\n    // TODO: Implement local inference\n    unimplmemented!()\n}}",
        model_path.display(),
        prompt
    ))
}

async fn run_llama_cpp(prompt: &str) -> anyhow::Result<String> {
    let output = tokio::process::Command::new("llama-cli")
        .args(&[
            "-p",
            prompt,
            "-n",
            "512",
            "--temp",
            "0.7",
        ])
        .output()
        .await?;
    
    if output.status.success() {
        let text = String::from_utf8_lossy(&output.stdout);
        Ok(extract_code(&text))
    } else {
        Err(anyhow::anyhow!("Local model execution failed"))
    }
}

fn generate_template_response(prompt: &str) -> String {
    format!(
        r#"// KLP Auto-Generated Code
// Prompt: {}
// Model: Template Fallback (no local model configured)

use std::process::Command;

pub fn execute() -> anyhow::Result<()> {{
    // TODO: Implement based on prompt
    println!("Executing: {:?}", {:?});
    
    Ok(())
}}

#[cfg(test)]
mod tests {{
    use super::*;
    
    #[test]
    fn test_execute() {{
        // TODO: Add tests
    }}
}}"#,
        prompt, prompt
    )
}

/// Extract code from markdown code blocks
fn extract_code(text: &str) -> String {
    // Look for ```rust or ``` code blocks
    if let Some(start) = text.find("```rust") {
        if let Some(end) = text[start + 7..].find("```") {
            return text[start + 7..start + 7 + end].trim().to_string();
        }
    }
    
    if let Some(start) = text.find("```") {
        if let Some(end) = text[start + 3..].find("```") {
            return text[start + 3..start + 3 + end].trim().to_string();
        }
    }
    
    // Return as-is if no code blocks found
    text.to_string()
}

/// Create HTTP client with configured timeout
fn create_http_client(config: &Config) -> anyhow::Result<Client> {
    let client = Client::builder()
        .timeout(Duration::from_secs(config.apis.timeout_seconds))
        .build()?;
    Ok(client)
}

/// Save generation to history
pub async fn save_to_history(prompt: &str, code: &str, config: &Config) -> anyhow::Result<()> {
    if !config.codegen.save_history {
        return Ok(());
    }
    
    let history_entry = serde_json::json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "prompt": prompt,
        "code": code,
    });
    
    // Append to history file
    let history_path = &config.codegen.history_path;
    let mut history = if history_path.exists() {
        let content = tokio::fs::read_to_string(history_path).await?;
        serde_json::from_str::<Vec<serde_json::Value>>(&content).unwrap_or_default()
    } else {
        Vec::new()
    };
    
    history.push(history_entry);
    
    // Keep only last 100 entries
    if history.len() > 100 {
        let skip_count = history.len() - 100;
        history = history.drain(skip_count..).collect();
    }
    
    let content = serde_json::to_string_pretty(&history)?;
    tokio::fs::write(history_path, content).await?;
    
    Ok(())
}
