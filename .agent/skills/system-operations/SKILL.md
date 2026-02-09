---
name: system-operations
description: System-level operations including process management, file operations, environment variables, and OS commands. Use when executing system commands, managing processes, checking system info, or performing file operations. Triggers on "system command", "run shell", "process management", "file operation".
---

# System Operations

Execute system-level commands and manage OS resources.

## Quick Start

```bash
# Execute shell command
klp sys exec "ls -la"

# Get system info
klp sys info

# Manage processes
klp sys ps               # List processes
klp sys kill <pid>       # Kill process

# File operations
klp sys file read <path>
klp sys file write <path> <content>
klp sys file delete <path>
```

## Commands

### System Information
- `info` - Display system information (OS, arch, memory, disk)
- `env` - Show environment variables
- `which <cmd>` - Find command location

### Process Management
- `ps` - List running processes
- `kill <pid>` - Terminate process
- `top` - Show system resource usage

### File Operations
- `file read <path>` - Read file contents
- `file write <path> <content>` - Write to file
- `file delete <path>` - Delete file
- `file copy <src> <dst>` - Copy file
- `file move <src> <dst>` - Move file

### Shell Execution
- `exec <command>` - Execute arbitrary shell command
- `eval <expression>` - Evaluate shell expression

## Examples

### Check System Resources
```bash
klp sys info
# Output:
# OS: macOS 14.0
# Architecture: x86_64
# Memory: 16GB
# Disk: 500GB available
```

### Manage Processes
```bash
# Find and kill a process
klp sys ps | grep firefox
klp sys kill 1234
```

### File Operations
```bash
# Quick file read
klp sys file read /etc/hosts

# Create temp file
klp sys file write /tmp/test.txt "Hello World"
```

## Safety

- Commands run with user permissions (no sudo by default)
- Confirmation required for destructive operations
- Environment variables are respected

## Source Code

See: `src/skills/sys.rs`
