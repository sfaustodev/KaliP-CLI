---
name: skill-creator
description: Generate new skill templates following the skill-creator specification. Use when creating new AI skills, scaffolding skill structures, or converting existing code to skill format. Triggers on "create skill", "new skill", "generate skill template".
---

# Skill Creator

Generate new skill templates following the official skill-creator specification.

## Quick Start

```bash
# Create a new skill
klp skill-creator create <skill-name>

# Create skill with specific type
klp skill-creator create <skill-name> --type rust
klp skill-creator create <skill-name> --type markdown

# Convert existing code to skill format
klp skill-creator convert <file.rs> --output <skill-name>
```

## Skill Structure

Generated skills follow this structure:
```
skill-name/
├── SKILL.md              # Required documentation
├── mod.rs                # Rust implementation (optional)
├── scripts/              # Helper scripts (optional)
│   └── example.sh
├── references/           # Documentation (optional)
│   └── api-docs.md
└── assets/               # Static files (optional)
    └── logo.png
```

## SKILL.md Template

```yaml
---
name: skill-name
description: Brief description of what this skill does and when to use it.
---

# Skill Name

## Overview

Brief explanation of the skill's purpose.

## Quick Start

```bash
# Example usage
klp skill-name <command> [args]
```

## Commands

- `command1` - Description
- `command2` - Description

## Examples

### Example 1: Basic Usage
```bash
klp skill-name do-something
```

### Example 2: Advanced Usage
```bash
klp skill-name do-something --option value
```

## References

- Link to relevant documentation
- Link to source code
```

## Usage Examples

### Create Security Skill
```bash
klp skill-creator create security-scanner --type rust
```

Generates:
- `.agent/skills/security-scanner/SKILL.md`
- `.agent/skills/security-scanner/mod.rs`

### Convert Python Script
```bash
klp skill-creator convert ./scripts/backup.py --output backup-skill
```

## Best Practices

1. **Keep SKILL.md under 500 lines**
2. **Use kebab-case for skill names**
3. **Include trigger phrases in description**
4. **Provide working examples**
5. **Link to source code in references**

## Source Code

See: `src/skills/skill_creator.rs`
