---
name: compatibilization
description: Discover, search, and manage AI skills from multiple sources including Anthropic, OpenAI, local directories, and npm packages. Use when you need to find available skills, import external skills, or check skill compatibility across different agent systems.
---

# Compatibilization Skill

Discover and manage AI skills from multiple sources for seamless cross-agent compatibility.

## Quick Start

```bash
# Search for all available skills
klp skill search

# Import skills from Anthropic repository
klp skill import anthropic

# Import skills from OpenAI repository
klp skill import openai

# List installed skills
klp skill list

# Check skill compatibility
klp skill compat <skill-name>
```

## Supported Sources

### 1. Anthropic Skills (GitHub)
- **Repository**: `github.com/anthropics/skills`
- **Official Claude skills**: data-analysis, document-creation, creative-writing
- **Usage**: Import directly or reference remotely

### 2. OpenAI Skills (GitHub)
- **Repository**: `github.com/openai/skills`
- **Codex skills**: code-generation, testing, documentation
- **Usage**: Import for Codex agent compatibility

### 3. Local Skills
- **Location**: `~/.config/opencode/skills/`
- **External npm skills**: `@tech-leads-club/agent-skills`
- **Custom skills**: User-created `.rs` files

### 4. KaliP Built-in Skills
- **Location**: `src/skills/` or `.agent/skills/`
- **Core skills**: system, git, chrome, network, security

## Import Methods

### Option A: Local Clone (Recommended for Offline Use)
```bash
# Clone Anthropic skills locally
git clone https://github.com/anthropics/skills.git ~/.ai-agents/skills/anthropic

# Clone OpenAI skills locally
git clone https://github.com/openai/skills.git ~/.ai-agents/skills/openai

# Set environment variable
export ANTHROPIC_SKILLS_PATH="$HOME/.ai-agents/skills/anthropic"
export OPENAI_SKILLS_PATH="$HOME/.ai-agents/skills/openai"
```

### Option B: Remote Reference (Token-Efficient)
```bash
# Use GitHub URLs directly without cloning
export ANTHROPIC_SKILLS_URL="https://github.com/anthropics/skills"
export OPENAI_SKILLS_URL="https://github.com/openai/skills"

# Skills are fetched on-demand via GitHub API
```

## Skill Discovery

### Search All Sources
```rust
// Example: Search for "database" skills across all sources
let results = skill_search("database").await;
// Returns: Anthropic DB skills, OpenAI DB skills, Local DB skills
```

### Check Compatibility Matrix
| Skill Source | OpenCode | Claude | Codex | KLP |
|-------------|----------|---------|-------|-----|
| Anthropic | ✅ | ✅ | ⚠️ | ✅ |
| OpenAI | ✅ | ⚠️ | ✅ | ✅ |
| Local (.rs) | ✅ | ✅ | ✅ | ✅ |
| MCP Servers | ✅ | ✅ | ✅ | ✅ |

## Directory Structure

```
.agent/skills/
├── compatibilization/      # This skill
├── anthropic-skills/       # Anthropic reference
├── openai-skills/          # OpenAI reference
├── skill-creator/          # Create new skills
├── system-operations/      # System commands
├── chrome-automation/      # Browser control
├── git-operations/         # Git workflows
├── network-connect/        # SSH/RDP/VNC
├── monitoring/             # System monitoring
├── osint/                  # OSINT tools
├── pentesting/             # Security testing
├── reporting/              # Report generation
├── selfdestruct/           # Cleanup tools
├── external/               # Symlinks to ~/.config/opencode/skills/
└── mcp-servers/            # MCP server configs
```

## Usage Examples

### Import and Use External Skill
```bash
# 1. Import from Anthropic
klp skill import anthropic/data-analysis

# 2. Check compatibility
klp skill compat data-analysis

# 3. Use the skill
klp skill run data-analysis "analyze sales.csv"
```

### Cross-Agent Skill Sharing
```bash
# Export KaliP skill for other agents
klp skill export system-operations --format=skill-md

# Import works with Claude/Codex format
klp skill import ./system-operations.skill.md
```

## References

- **Anthropic Skills Repo**: [github.com/anthropics/skills](https://github.com/anthropics/skills)
- **OpenAI Skills Repo**: [github.com/openai/skills](https://github.com/openai/skills)
- **Agent Skills Spec**: [agentskills.io](https://agentskills.io)
- **MCP Protocol**: [modelcontextprotocol.io](https://modelcontextprotocol.io)

## Source Code

See: `src/skills/compatibilization/mod.rs`
