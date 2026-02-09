---
name: monitoring
description: System monitoring and alerting including CPU, memory, disk usage, process monitoring, and health checks. Use for tracking system performance, setting up alerts, or monitoring application health. Triggers on "monitor", "alert", "system health", "performance", "metrics".
---

# Monitoring

Monitor system performance, resources, and application health.

## Quick Start

```bash
# Show system metrics
klp monitor system

# Monitor specific process
klp monitor process <name>

# Watch resource usage
klp monitor watch --interval 5

# Health check
klp monitor health
```

## Commands

### System Metrics
- `system` - Show current system metrics
- `cpu` - CPU usage details
- `memory` - Memory usage details
- `disk` - Disk usage details
- `network` - Network I/O statistics

### Process Monitoring
- `process <name>` - Monitor specific process
- `process list` - List top processes
- `process kill <pid>` - Kill process

### Continuous Monitoring
- `watch` - Continuous monitoring mode
- `watch --interval <seconds>` - Set update interval
- `watch --log <file>` - Log to file

### Health Checks
- `health` - System health check
- `health --service <name>` - Check service health
- `health --endpoint <url>` - HTTP health check

### Alerts
- `alert add <condition>` - Add alert condition
- `alert list` - List active alerts
- `alert remove <id>` - Remove alert

## Examples

### System Overview
```bash
klp monitor system
# Output:
# CPU: 23% (8 cores)
# Memory: 8.2GB / 16GB (51%)
# Disk: 120GB / 500GB (24%)
# Load: 1.23
```

### Process Monitoring
```bash
# Monitor Chrome processes
klp monitor process chrome

# Find resource hogs
klp monitor process list --sort cpu
```

### Continuous Watch
```bash
# Watch system every 5 seconds
klp monitor watch --interval 5

# Watch and log to file
klp monitor watch --interval 10 --log /var/log/monitor.log
```

### Health Checks
```bash
# Check system health
klp monitor health

# Check website
klp monitor health --endpoint https://example.com

# Check service
klp monitor health --service nginx
```

### Alerting
```bash
# Alert when CPU > 80%
klp monitor alert add "cpu > 80%"

# Alert when disk < 10GB free
klp monitor alert add "disk_free < 10GB"

# Email notification
klp monitor alert add "memory > 90%" --email admin@example.com
```

## Configuration

```toml
[monitor]
interval_seconds = 5
log_file = "/var/log/klp-monitor.log"

[monitor.alerts]
cpu_threshold = 80.0
memory_threshold = 90.0
disk_threshold = 85.0

[monitor.notifications]
email = "admin@example.com"
webhook = "https://hooks.slack.com/..."
```

## Source Code

See: `src/skills/monitor.rs`
