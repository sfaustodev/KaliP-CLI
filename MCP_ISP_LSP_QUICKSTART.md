# MCP + ISP + LSP Quick Start Guide

Complete setup guide for **MCP (Model Context Protocol)**, **ISP (Internet Service Provider)**, and **LSP (Language Server Protocol)** across all AI agents: **OpenCode**, **Claude**, **Codex**, **KLP**, and **Fenrir**.

---

## Table of Contents

1. [MCP (Model Context Protocol)](#mcp-model-context-protocol)
2. [ISP (Internet Service Provider)](#isp-internet-service-provider)
3. [LSP (Language Server Protocol)](#lsp-language-server-protocol)
4. [Per-Agent Configuration](#per-agent-configuration)
5. [Shared Environment Setup](#shared-environment-setup)

---

## MCP (Model Context Protocol)

MCP servers provide AI agents with tools, context, and capabilities through a standardized protocol.

### Essential MCP Servers

We recommend these **7 core MCP servers** for all agents:

#### 1. Filesystem
**Purpose**: Read/write local files
```json
{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/path/to/workspace"]
    }
  }
}
```

#### 2. Git
**Purpose**: Repository operations
```json
{
  "mcpServers": {
    "git": {
      "command": "uvx",
      "args": ["mcp-server-git", "--repository", "/path/to/repo"]
    }
  }
}
```

#### 3. GitHub
**Purpose**: GitHub API integration
```json
{
  "mcpServers": {
    "github": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-github"],
      "env": {
        "GITHUB_PERSONAL_ACCESS_TOKEN": "${GITHUB_TOKEN}"
      }
    }
  }
}
```

#### 4. Fetch
**Purpose**: Web content fetching
```json
{
  "mcpServers": {
    "fetch": {
      "command": "uvx",
      "args": ["mcp-server-fetch"]
    }
  }
}
```

#### 5. Anthropic
**Purpose**: Claude API access
```json
{
  "mcpServers": {
    "anthropic": {
      "command": "npx",
      "args": ["-y", "@anthropic-ai/mcp-server"],
      "env": {
        "ANTHROPIC_API_KEY": "${ANTHROPIC_API_KEY}"
      }
    }
  }
}
```

#### 6. Context7
**Purpose**: Documentation search
```json
{
  "mcpServers": {
    "context7": {
      "command": "npx",
      "args": ["-y", "@upstash/context7-mcp"]
    }
  }
}
```

#### 7. Environment Bridge (Custom)
**Purpose**: Shared .env file across all agents
```json
{
  "mcpServers": {
    "env-bridge": {
      "command": "python3",
      "args": ["${HOME}/.ai-agents/mcp-servers/env-bridge/server.py"],
      "env": {
        "SHARED_ENV_PATH": "${HOME}/.ai-agents/.env"
      }
    }
  }
}
```

### Installation

```bash
# Install Node.js (required for npx)
brew install node

# Install uv (required for uvx)
curl -LsSf https://astral.sh/uv/install.sh | sh

# Install Python dependencies for env-bridge
pip3 install python-dotenv
```

### Prerequisites

- **Node.js**: `brew install node`
- **uv**: `curl -LsSf https://astral.sh/uv/install.sh | sh`
- **Python 3**: Usually pre-installed on macOS

---

## ISP (Internet Service Provider)

### Recommended ISPs for AI Development

#### 🥇 Best: Google Fiber or Verizon Fios
- **Latency**: <10ms
- **Download/Upload**: Symmetrical 1Gbps+
- **Reliability**: 99.9% uptime
- **Why**: Lowest latency for AI API calls, symmetrical speeds for uploads

#### 🥈 Good: AT&T Fiber or Comcast Xfinity (Fiber)
- **Latency**: 10-20ms
- **Download/Upload**: 500Mbps-1Gbps
- **Reliability**: Good
- **Why**: Widely available, good performance

#### Requirements for AI Agents
- **Latency**: <20ms (lower is better)
- **Bandwidth**: 100+ Mbps symmetrical
- **Stability**: Minimal packet loss
- **Type**: Fiber optic preferred over cable

### Testing Your Connection

```bash
# Test latency to OpenAI
curl -o /dev/null -s -w "%{time_total}\n" https://api.openai.com/v1/models

# Test latency to Anthropic
curl -o /dev/null -s -w "%{time_total}\n" https://api.anthropic.com/v1/models

# General speed test
brew install speedtest-cli
speedtest-cli
```

### VPN Considerations

If using a VPN for security:
- **WireGuard**: Best performance for AI workloads
- **Latency impact**: Should add <5ms
- **Location**: Choose server closest to API endpoints

---

## LSP (Language Server Protocol)

LSP provides IDE features (autocomplete, go-to-definition, etc.) for code editors.

### Installation Priority

#### 1. **Rust** (Primary - Start Here)
```bash
# Install rust-analyzer (best-in-class)
brew install rust-analyzer

# Or via rustup
rustup component add rust-analyzer
```

#### 2. TypeScript/JavaScript (Documented)
```bash
# Via npm (documented but not installed by default)
npm install -g typescript-language-server
```

#### 3. Python (Documented)
```bash
# Option A: Zuban (Rust-based, fastest)
cargo install zuban

# Option B: Pyright (Microsoft)
npm install -g pyright

# Option C: Pylance (VSCode built-in)
# Already included with VSCode
```

#### 4. Go (Documented)
```bash
# Install gopls
go install golang.org/x/tools/gopls@latest
```

### Rust-Analyzer Setup (Primary)

```bash
# 1. Install
brew install rust-analyzer

# 2. Verify
rust-analyzer --version

# 3. Configure in your editor (see Per-Agent section)
```

### LSP Configuration Format

Most agents use this configuration structure:

```json
{
  "lsp": {
    "rust-analyzer": {
      "command": "rust-analyzer",
      "rootPatterns": ["Cargo.toml"],
      "filetypes": ["rust"]
    }
  }
}
```

---

## Per-Agent Configuration

### 1. OpenCode

**Config Location**: `~/.config/opencode/settings.json`

```json
{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "${HOME}/KALIP"]
    },
    "git": {
      "command": "uvx",
      "args": ["mcp-server-git", "--repository", "${HOME}/KALIP"]
    },
    "github": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-github"],
      "env": {
        "GITHUB_PERSONAL_ACCESS_TOKEN": "${GITHUB_TOKEN}"
      }
    },
    "fetch": {
      "command": "uvx",
      "args": ["mcp-server-fetch"]
    },
    "anthropic": {
      "command": "npx",
      "args": ["-y", "@anthropic-ai/mcp-server"],
      "env": {
        "ANTHROPIC_API_KEY": "${ANTHROPIC_API_KEY}"
      }
    },
    "context7": {
      "command": "npx",
      "args": ["-y", "@upstash/context7-mcp"]
    },
    "env-bridge": {
      "command": "python3",
      "args": ["${HOME}/.ai-agents/mcp-servers/env-bridge/server.py"],
      "env": {
        "SHARED_ENV_PATH": "${HOME}/.ai-agents/.env"
      }
    }
  },
  "lsp": {
    "rust-analyzer": {
      "command": "rust-analyzer",
      "filetypes": ["rust"]
    }
  }
}
```

**Setup Commands**:
```bash
# Create config directory
mkdir -p ~/.config/opencode

# Copy MCP config
cp .agent/skills/mcp-servers/mcp-config.json ~/.config/opencode/mcp.json

# Install LSP
brew install rust-analyzer
```

### 2. Claude (Claude Code CLI)

**Config Location**: `~/.config/claude/settings.json`

```json
{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "${HOME}/KALIP"]
    },
    "git": {
      "command": "uvx",
      "args": ["mcp-server-git"]
    },
    "github": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-github"]
    },
    "fetch": {
      "command": "uvx",
      "args": ["mcp-server-fetch"]
    },
    "context7": {
      "command": "npx",
      "args": ["-y", "@upstash/context7-mcp"]
    },
    "env-bridge": {
      "command": "python3",
      "args": ["${HOME}/.ai-agents/mcp-servers/env-bridge/server.py"]
    }
  }
}
```

**Setup Commands**:
```bash
# Claude Code installs MCP servers automatically
# Just add the config above to ~/.config/claude/settings.json

# Or use Claude Code CLI
claude config set mcp.servers.filesystem.command "npx -y @modelcontextprotocol/server-filesystem ${HOME}"
```

### 3. Codex (OpenAI)

**Config Location**: `~/.config/codex/config.json`

```json
{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "${HOME}/KALIP"]
    },
    "git": {
      "command": "uvx",
      "args": ["mcp-server-git"]
    },
    "github": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-github"]
    },
    "fetch": {
      "command": "uvx",
      "args": ["mcp-server-fetch"]
    },
    "env-bridge": {
      "command": "python3",
      "args": ["${HOME}/.ai-agents/mcp-servers/env-bridge/server.py"]
    }
  },
  "skills": {
    "path": "${HOME}/.ai-agents/skills/openai"
  }
}
```

**Setup Commands**:
```bash
# Install Codex CLI
npm install -g @openai/codex

# Configure
mkdir -p ~/.config/codex
# Add config above to ~/.config/codex/config.json
```

### 4. KLP (KaliP)

**Config Location**: `~/.klp/config.toml`

```toml
[mcp]
enabled = true
config_path = "~/.ai-agents/mcp-servers/mcp-config.json"

[mcp.servers]
filesystem = { enabled = true, path = "~/KALIP" }
git = { enabled = true }
github = { enabled = true }
fetch = { enabled = true }
anthropic = { enabled = true }
context7 = { enabled = true }
env-bridge = { enabled = true, shared_env = "~/.ai-agents/.env" }

[lsp]
enabled = true

[lsp.rust]
command = "rust-analyzer"
filetypes = ["rust"]
root_patterns = ["Cargo.toml"]
```

**Setup Commands**:
```bash
# MCP servers are configured in KaliP config
# Copy the MCP config
cp .agent/skills/mcp-servers/mcp-config.json ~/.ai-agents/mcp-servers/

# Install rust-analyzer
brew install rust-analyzer
```

### 5. Fenrir

**Status**: Placeholder - requires manual configuration

**Instructions for Fenrir Setup**:

```markdown
## Fenrir MCP Setup (Manual)

1. **Locate Fenrir Config**:
   - Check: `~/.config/fenrir/` or `~/CLI Fenrir/fenrir/`
   - Find: `config.json`, `settings.json`, or `mcp.json`

2. **Add MCP Servers**:
   Copy this MCP configuration to Fenrir's config:

   ```json
   {
     "mcpServers": {
       "filesystem": {
         "command": "npx",
         "args": ["-y", "@modelcontextprotocol/server-filesystem", "${HOME}/KALIP"]
       },
       "git": {
         "command": "uvx",
         "args": ["mcp-server-git"]
       },
       "github": {
         "command": "npx",
         "args": ["-y", "@modelcontextprotocol/server-github"]
       },
       "fetch": {
         "command": "uvx",
         "args": ["mcp-server-fetch"]
       },
       "env-bridge": {
         "command": "python3",
         "args": ["${HOME}/.ai-agents/mcp-servers/env-bridge/server.py"],
         "env": {
           "SHARED_ENV_PATH": "${HOME}/.ai-agents/.env"
         }
       }
     }
   }
   ```

3. **LSP Setup**:
   - Install rust-analyzer: `brew install rust-analyzer`
   - Add to Fenrir's LSP config

4. **Verify**:
   - Restart Fenrir
   - Test MCP connection: Ask agent to read a file
   - Test LSP: Open a Rust file and check for autocomplete
```

---

## Shared Environment Setup

### Central .env File

Location: `~/.ai-agents/.env`

```bash
# Create directory and file
mkdir -p ~/.ai-agents/mcp-servers/env-bridge
touch ~/.ai-agents/.env

# Set permissions
chmod 600 ~/.ai-agents/.env
```

### Required API Keys

Add to `~/.ai-agents/.env`:

```bash
# OpenAI
OPENAI_API_KEY=sk-...

# Anthropic
ANTHROPIC_API_KEY=sk-ant-...

# GitHub
GITHUB_TOKEN=ghp_...

# Context7 (optional)
UPSTASH_REDIS_URL=https://...
UPSTASH_REDIS_TOKEN=...

# Other services
OPEN_CODE_API_KEY=...
GROQ_API_KEY=gsk_...
```

### Environment Bridge Usage

The env-bridge MCP server allows all agents to share this .env file:

```bash
# In any agent, you can:
env_get OPENAI_API_KEY        # Retrieve key
env_set NEW_KEY value         # Set new key
context_save project_context "Current task: refactor auth"  # Share context
```

---

## Verification Checklist

### MCP Verification

```bash
# Test each MCP server
# 1. Filesystem - should read files
# 2. Git - should show git status
# 3. GitHub - should list repos
# 4. Fetch - should fetch URLs
# 5. Anthropic - should query Claude
# 6. Context7 - should search docs
# 7. Env-bridge - should read .env
```

### ISP Verification

```bash
# Test latency
curl -o /dev/null -s -w "OpenAI: %{time_total}s\n" https://api.openai.com/v1/models
curl -o /dev/null -s -w "Anthropic: %{time_total}s\n" https://api.anthropic.com/v1/models

# Should be <200ms for good performance
```

### LSP Verification

```bash
# Test rust-analyzer
rust-analyzer --version
# Should output version number

# Test in editor
# Open any .rs file and check for:
# - Autocomplete
# - Error highlighting
# - Go-to-definition
```

---

## Troubleshooting

### MCP Issues

**Problem**: MCP server not starting
```bash
# Check if command exists
which npx
which uvx

# Reinstall if needed
brew reinstall node
curl -LsSf https://astral.sh/uv/install.sh | sh
```

**Problem**: Permission denied
```bash
# Fix permissions
chmod +x ~/.ai-agents/mcp-servers/env-bridge/server.py
chmod 600 ~/.ai-agents/.env
```

### LSP Issues

**Problem**: rust-analyzer not found
```bash
# Reinstall
brew reinstall rust-analyzer

# Or use rustup
rustup component add rust-analyzer
```

### ISP Issues

**Problem**: High latency
- Check VPN settings
- Test without VPN
- Consider switching ISPs
- Use regional API endpoints

---

## Quick Reference

| Component | Priority | Install Command |
|-----------|----------|----------------|
| rust-analyzer (LSP) | 🔴 High | `brew install rust-analyzer` |
| Filesystem (MCP) | 🔴 High | `npx -y @modelcontextprotocol/server-filesystem` |
| Git (MCP) | 🔴 High | `uvx mcp-server-git` |
| GitHub (MCP) | 🟡 Medium | `npx -y @modelcontextprotocol/server-github` |
| Fetch (MCP) | 🟡 Medium | `uvx mcp-server-fetch` |
| Anthropic (MCP) | 🟡 Medium | `npx -y @anthropic-ai/mcp-server` |
| Context7 (MCP) | 🟢 Optional | `npx -y @upstash/context7-mcp` |
| Env-bridge (MCP) | 🔴 High | Custom Python server |

---

## Support

- **MCP Docs**: https://modelcontextprotocol.io
- **LSP Docs**: https://microsoft.github.io/language-server-protocol/
- **KaliP Skills**: See `.agent/skills/mcp-servers/`

---

**Last Updated**: 2026-02-09
**Version**: 1.0
