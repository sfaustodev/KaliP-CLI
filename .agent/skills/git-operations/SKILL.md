---
name: git-operations
description: Git repository operations including clone, commit, push, pull, branch management, and advanced git workflows. Use for repository management, version control tasks, or git automation. Triggers on "git", "repository", "commit", "push", "pull", "branch".
---

# Git Operations

Manage Git repositories with advanced workflows and automation.

## Quick Start

```bash
# Repository operations
klp git clone <url> [directory]
klp git status
klp git commit -m "message"
klp git push
klp git pull

# Branch management
klp git branch list
klp git branch create <name>
klp git branch switch <name>
klp git branch delete <name>

# Advanced operations
klp git stash
klp git stash pop
klp git merge <branch>
klp git rebase <branch>
```

## Commands

### Repository Setup
- `clone <url> [dir]` - Clone repository
- `init [dir]` - Initialize new repository
- `remote add <name> <url>` - Add remote
- `remote remove <name>` - Remove remote

### Basic Operations
- `status` - Show working tree status
- `add <files>` - Stage files
- `commit -m <msg>` - Commit changes
- `push [remote] [branch]` - Push to remote
- `pull [remote] [branch]` - Pull from remote
- `fetch [remote]` - Fetch from remote

### Branch Management
- `branch list` - List branches
- `branch create <name>` - Create branch
- `branch switch <name>` - Switch branch
- `branch delete <name>` - Delete branch
- `branch merge <name>` - Merge branch

### History & Diff
- `log [options]` - Show commit history
- `diff [files]` - Show differences
- `show <commit>` - Show commit details
- `blame <file>` - Show line annotations

### Advanced
- `stash [message]` - Stash changes
- `stash pop` - Apply stashed changes
- `rebase <branch>` - Rebase current branch
- `reset <mode> <commit>` - Reset to commit
- `tag <name>` - Create tag
- `clean` - Remove untracked files

## Examples

### Daily Workflow
```bash
# Start work
klp git pull origin main
klp git branch create feature-login

# Make changes
klp git add .
klp git commit -m "Add login functionality"

# Share work
klp git push origin feature-login
```

### Advanced Scenarios
```bash
# Interactive rebase
klp git rebase -i HEAD~5

# Cherry-pick commit
klp git cherry-pick abc123

# Bisect debugging
klp git bisect start
klp git bisect bad
klp git bisect good v1.0
```

## Automation

### Git Hooks
Configure hooks in `.githooks/`:
```bash
klp git hooks install
klp git hooks enable pre-commit
```

### Bulk Operations
```bash
# Update all repos in directory
klp git bulk pull

# Status across multiple repos
klp git bulk status
```

## Source Code

See: `src/skills/git.rs`
