---
name: osint
description: Open Source Intelligence gathering including domain reconnaissance, social media analysis, metadata extraction, and public data collection. Use for security research, reconnaissance, or gathering publicly available information. Triggers on "osint", "reconnaissance", "domain lookup", "whois", "metadata".
---

# OSINT (Open Source Intelligence)

Gather publicly available information for security research and reconnaissance.

## Quick Start

```bash
# Domain reconnaissance
klp osint domain <domain.com>

# IP lookup
klp osint ip <ip-address>

# Email verification
klp osint email <email@example.com>

# Metadata extraction
klp osint metadata <file>
```

## Commands

### Domain Analysis
- `domain <domain>` - Domain information
- `whois <domain>` - WHOIS lookup
- `dns <domain>` - DNS records
- `subdomains <domain>` - Find subdomains

### Network Reconnaissance
- `ip <address>` - IP information
- `port <host>` - Port scan
- `ssl <host>` - SSL/TLS analysis

### Data Extraction
- `metadata <file>` - Extract file metadata
- `social <username>` - Social media lookup
- `email <address>` - Email verification

### Reporting
- `report <target>` - Generate OSINT report
- `export <format>` - Export data (json, csv, pdf)

## Examples

### Domain Reconnaissance
```bash
# Full domain analysis
klp osint domain example.com

# DNS records
klp osint dns example.com

# Find subdomains
klp osint subdomains example.com
```

### Network Analysis
```bash
# IP geolocation
klp osint ip 8.8.8.8

# Quick port scan
klp osint port scanme.nmap.org --top-ports

# SSL certificate info
klp osint ssl example.com
```

### Metadata Extraction
```bash
# Image metadata
klp osint metadata photo.jpg

# Document metadata
klp osint metadata document.pdf
```

### Social Media
```bash
# Username search across platforms
klp osint social username123
```

## Safety & Ethics

**IMPORTANT:** This skill is for **authorized security research only.**

- ✅ **DO** use on systems you own or have written permission to test
- ✅ **DO** use for defensive security operations
- ✅ **DO** follow responsible disclosure practices
- ❌ **DO NOT** use for unauthorized access or stalking
- ❌ **DO NOT** violate terms of service
- ❌ **DO NOT** engage in illegal activities

## Configuration

```toml
[osint]
threads = 10
timeout_seconds = 30
rate_limit = "1s"

[osint.api_keys]
shodan = "your-shodan-key"
virustotal = "your-vt-key"
```

## Source Code

See: `src/skills/osint.rs`
