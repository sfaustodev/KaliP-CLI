---
name: selfdestruct
description: System cleanup, trace wiping, and maintenance operations. Use for clearing logs, wiping traces, cleanup operations, or system maintenance. Triggers on "wipe", "clean", "cleanup", "clear logs", "maintenance", "self-destruct".
---

# Selfdestruct

System cleanup, trace wiping, and maintenance operations.

## Quick Start

```bash
# Wipe bash history
klp security wipe bash

# Clear logs
klp security logs clear

# Encrypt logs
klp security logs encrypt

# Full cleanup
klp security wipe all
```

## Commands

### Trace Wiping
- `wipe bash` - Clear bash history
- `wipe chrome` - Clear browser data
- `wipe logs` - Clear system logs
- `wipe all` - Comprehensive wipe

### Log Management
- `logs encrypt` - Encrypt operation logs
- `logs decrypt` - Decrypt logs
- `logs clear` - Clear logs
- `logs rotate` - Rotate log files

### Maintenance
- `clean temp` - Clear temporary files
- `clean cache` - Clear cache
- `clean old` - Remove old files
- `stealth on` - Enable stealth mode
- `stealth off` - Disable stealth mode

## Examples

### Stealth Operations
```bash
# Enable stealth mode
klp security stealth on

# Perform operations...

# Wipe traces when done
klp security wipe all
klp security stealth off
```

### Log Encryption
```bash
# Encrypt logs with password
klp security logs encrypt

# Decrypt for review
klp security logs decrypt
```

### Cleanup Operations
```bash
# Clean temporary files
klp security clean temp

# Clear all caches
klp security clean cache

# Remove files older than 30 days
klp security clean old --days 30
```

## Stealth Mode

When stealth mode is enabled:
- Shell history is disabled
- Logs are encrypted automatically
- Temporary files are wiped on exit
- Network traces are minimized

## Safety

- Confirmation required for destructive operations
- Encrypted logs can be recovered with password
- Backup important data before wiping
- Stealth mode does not make you invisible

## Source Code

See: `src/skills/selfdestruct.rs`
