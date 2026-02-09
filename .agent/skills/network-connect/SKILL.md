---
name: network-connect
description: Network connectivity and remote access operations including SSH, RDP, VNC, and custom protocols. Use for connecting to remote systems, managing connections, or automating remote workflows. Triggers on "connect", "ssh", "remote", "rdp", "vnc".
---

# Network Connect

Connect to remote systems via SSH, RDP, VNC, and other protocols.

## Quick Start

```bash
# SSH connection
klp connect ssh user@hostname --port 22

# RDP connection
klp connect rdp hostname --user admin

# VNC connection
klp connect vnc hostname --port 5900

# Custom protocol
klp connect <hostname> --protocol custom --port 8080
```

## Commands

### SSH Operations
- `ssh <user@host>` - Connect via SSH
- `ssh exec <user@host> <command>` - Execute remote command
- `ssh copy <local> <user@host>:<remote>` - SCP file transfer
- `ssh key add <key>` - Add SSH key
- `ssh key list` - List SSH keys

### RDP Operations
- `rdp <host>` - Connect via RDP
- `rdp <host> --user <user>` - Connect with username
- `rdp <host> --resolution 1920x1080` - Set resolution

### VNC Operations
- `vnc <host>` - Connect via VNC
- `vnc <host> --port 5900` - Specify port
- `vnc <host> --password <pass>` - Set password

### Connection Management
- `list` - List active connections
- `disconnect <id>` - Disconnect session
- `status <id>` - Check connection status

## Examples

### SSH Workflows
```bash
# Connect to server
klp connect ssh admin@server.example.com

# Execute command remotely
klp connect ssh exec admin@server "uptime"

# Copy files
klp connect ssh copy ./local.txt admin@server:/remote/path/
```

### RDP Sessions
```bash
# Connect to Windows server
klp connect rdp windows-server --user Administrator

# With custom resolution
klp connect rdp windows-server --resolution 2560x1440
```

### Batch Operations
```bash
# Execute on multiple servers
klp connect ssh exec user@server1,user@server2 "apt update"
```

## Configuration

Configure in `~/.klp/config.toml`:
```toml
[connect.ssh]
default_user = "admin"
key_path = "~/.ssh/id_rsa"

[connect.rdp]
default_resolution = "1920x1080"
fullscreen = false

[connect.vnc]
default_port = 5900
quality = "high"
```

## Security

- SSH keys preferred over passwords
- Connection timeouts enforced
- Session logging available
- VPN tunnel support

## Source Code

See: `src/skills/connect.rs`
