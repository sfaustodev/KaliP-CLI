---
name: anthropic-skills
description: Reference to Anthropic's official skills repository for Claude. Import data-analysis, document-creation, creative-writing, and enterprise communication skills. Use when needing official Claude skills or cross-agent compatibility. Triggers on "anthropic skills", "claude skills", "import anthropic".
---

# Anthropic Skills

Official skills from Anthropic for Claude Code and Claude API.

## Overview

Anthropic maintains an official skills repository at `github.com/anthropics/skills` containing example skills that demonstrate how to extend Claude's capabilities.

## Repository

- **URL**: https://github.com/anthropics/skills
- **Stars**: 65,000+
- **License**: Various (check individual skills)
- **Format**: SKILL.md specification

## Available Skills

### Data Analysis
- **data-analysis** - CSV, JSON, and data processing
- **pandas-workflow** - Pandas dataframe operations
- **chart-generation** - Create charts and visualizations

### Document Creation
- **document-creation** - Generate formatted documents
- **brand-guidelines** - Apply company branding
- **contract-review** - Legal document analysis

### Creative Writing
- **creative-writing** - Story and content generation
- **blog-posts** - Technical blog writing
- **documentation** - Technical documentation

### Enterprise Communications
- **email-drafting** - Professional email writing
- **meeting-notes** - Meeting transcription and summaries
- **presentation-slides** - Slide deck creation

## Import Options

### Option A: Local Clone (Recommended)

```bash
# Clone repository locally
git clone https://github.com/anthropics/skills.git ~/.ai-agents/skills/anthropic

# Set environment variable
export ANTHROPIC_SKILLS_PATH="$HOME/.ai-agents/skills/anthropic"

# Skills are now available offline
```

**Pros:**
- ✅ Works offline
- ✅ Fast access
- ✅ Can modify/customize
- ✅ Version controlled

**Cons:**
- ❌ Requires periodic `git pull` for updates
- ❌ Takes disk space (~50MB)

### Option B: Remote Reference

```bash
# Use GitHub API to fetch skills on-demand
export ANTHROPIC_SKILLS_URL="https://api.github.com/repos/anthropics/skills/contents"

# Skills fetched when needed (requires internet)
```

**Pros:**
- ✅ Always up-to-date
- ✅ No local storage
- ✅ Automatic updates

**Cons:**
- ❌ Requires internet
- ❌ Rate limits apply
- ❌ Slower access

## Usage in KaliP

```bash
# Import specific skill
klp skill import anthropic data-analysis

# List available Anthropic skills
klp skill list --source anthropic

# Use imported skill
klp skill run data-analysis "process sales.csv"
```

## Cross-Agent Compatibility

| Skill | OpenCode | Claude | Codex | KLP |
|-------|----------|--------|-------|-----|
| data-analysis | ✅ | ✅ | ⚠️ | ✅ |
| document-creation | ✅ | ✅ | ⚠️ | ✅ |
| creative-writing | ✅ | ✅ | ⚠️ | ✅ |
| email-drafting | ✅ | ✅ | ⚠️ | ✅ |

⚠️ Codex may require minor adaptations

## Official Resources

- **Main Repo**: https://github.com/anthropics/skills
- **Documentation**: https://docs.anthropic.com/en/docs/agents-and-tools/agent-skills
- **Specification**: https://agentskills.io
- **Examples**: Repository contains working examples

## Claude Code Plugins

Anthropic also maintains official plugins:
- **Repo**: https://github.com/anthropics/claude-plugins-official
- **Usage**: Extend Claude Code with MCP servers

## Integration

These skills integrate with:
- Claude Code CLI
- Claude API
- KaliP (via compatibilization)
- OpenCode (via skill-creator format)

## Source

This is a reference skill pointing to external resources.
Clone locally for full functionality.
