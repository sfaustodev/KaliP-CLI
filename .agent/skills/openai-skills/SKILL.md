---
name: openai-skills
description: Reference to OpenAI's official skills repository for Codex and Agents SDK. Import code-generation, testing, documentation, and multi-agent workflow skills. Use when needing official OpenAI skills or cross-agent compatibility. Triggers on "openai skills", "codex skills", "import openai".
---

# OpenAI Skills

Official skills from OpenAI for Codex CLI and Agents SDK.

## Overview

OpenAI maintains an official skills repository at `github.com/openai/skills` containing skills designed for their Codex coding agent and Agents SDK framework.

## Repository

- **URL**: https://github.com/openai/skills
- **Stars**: 7,000+
- **License**: MIT
- **Format**: SKILL.md specification (Agent Skills standard)

## Available Skills

### Code Generation
- **code-generation** - Generate code from descriptions
- **code-review** - Review and suggest improvements
- **refactoring** - Automated code refactoring
- **test-generation** - Generate unit tests

### Development Workflows
- **git-workflow** - Advanced git operations
- **ci-cd** - CI/CD pipeline management
- **deployment** - Deployment automation
- **docker-setup** - Container configuration

### Documentation
- **doc-generation** - Generate documentation
- **api-docs** - API documentation
- **readme-generator** - Create README files

### Testing & Quality
- **test-automation** - Automated testing
- **linting** - Code quality checks
- **security-scan** - Security analysis

### Multi-Agent
- **agent-orchestration** - Coordinate multiple agents
- **task-delegation** - Delegate tasks to agents
- **workflow-automation** - Complex workflows

## Import Options

### Option A: Local Clone (Recommended)

```bash
# Clone repository locally
git clone https://github.com/openai/skills.git ~/.ai-agents/skills/openai

# Set environment variable
export OPENAI_SKILLS_PATH="$HOME/.ai-agents/skills/openai"

# Skills are now available offline
```

**Pros:**
- ✅ Works offline
- ✅ Fast access
- ✅ Can modify/customize
- ✅ Version controlled

**Cons:**
- ❌ Requires periodic `git pull` for updates
- ❌ Takes disk space (~30MB)

### Option B: Remote Reference

```bash
# Use GitHub API to fetch skills on-demand
export OPENAI_SKILLS_URL="https://api.github.com/repos/openai/skills/contents"

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

## Agents SDK

OpenAI's Agents SDK provides the framework for these skills:

```python
# Example Agents SDK usage
from agents import Agent, Runner

agent = Agent(
    name="Code Assistant",
    instructions="You are a helpful coding assistant",
    tools=[...]  # Import OpenAI skills as tools
)

result = await Runner.run(agent, "Generate a Python API")
```

**SDK Repository**: https://github.com/openai/openai-agents-python

## Usage in KaliP

```bash
# Import specific skill
klp skill import openai code-generation

# List available OpenAI skills
klp skill list --source openai

# Use imported skill
klp skill run code-generation "create a REST API"
```

## Cross-Agent Compatibility

| Skill | OpenCode | Claude | Codex | KLP |
|-------|----------|--------|-------|-----|
| code-generation | ✅ | ⚠️ | ✅ | ✅ |
| test-generation | ✅ | ⚠️ | ✅ | ✅ |
| git-workflow | ✅ | ✅ | ✅ | ✅ |
| docker-setup | ✅ | ✅ | ✅ | ✅ |

⚠️ Claude may require minor adaptations for Codex-specific features

## Official Resources

- **Skills Repo**: https://github.com/openai/skills
- **Agents SDK**: https://github.com/openai/openai-agents-python
- **Documentation**: https://openai.github.io/openai-agents-python/
- **Codex Docs**: https://developers.openai.com/codex/skills

## Agent Skills Standard

Both Anthropic and OpenAI support the open Agent Skills standard:
- **Specification**: https://agentskills.io
- **Format**: SKILL.md with YAML frontmatter
- **Goal**: Write once, use everywhere

## Integration

These skills integrate with:
- Codex CLI
- OpenAI Agents SDK
- KaliP (via compatibilization)
- OpenCode (via skill-creator format)

## Source

This is a reference skill pointing to external resources.
Clone locally for full functionality.
