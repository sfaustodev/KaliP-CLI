---
name: reporting
description: Generate professional security reports in PDF, HTML, and Markdown formats. Use for creating penetration test reports, system audit reports, or documentation. Triggers on "generate report", "create report", "export", "pdf", "security report".
---

# Reporting

Generate professional security and system reports.

## Quick Start

```bash
# Generate security report
klp report --kind pentest --output report.pdf

# System audit report
klp report --kind system --output audit.html

# OSINT report
klp report --kind osint --target example.com --output osint.md
```

## Commands

### Report Types
- `report --kind pentest` - Penetration test report
- `report --kind system` - System audit report
- `report --kind osint` - OSINT report
- `report --kind full` - Comprehensive report

### Output Formats
- `--format pdf` - PDF document
- `--format html` - HTML page
- `--format md` - Markdown
- `--format json` - JSON data

### Report Options
- `--output <file>` - Output file path
- `--template <name>` - Use custom template
- `--company <name>` - Company name
- `--author <name>` - Author name
- `--logo <path>` - Add company logo

## Examples

### Penetration Test Report
```bash
# Basic pentest report
klp report --kind pentest --output pentest-report.pdf

# With custom branding
klp report --kind pentest \
  --company "Security Corp" \
  --author "John Doe" \
  --output pentest-report.pdf
```

### System Audit Report
```bash
# System audit
klp report --kind system --output system-audit.html

# Comprehensive report
klp report --kind full --output comprehensive-report.pdf
```

### OSINT Report
```bash
# OSINT findings
klp report --kind osint --target example.com --output osint-findings.md
```

## Report Templates

### Default Templates
- `pentest` - Professional pentest report
- `system` - System audit template
- `osint` - OSINT findings template
- `executive` - Executive summary
- `technical` - Technical details

### Custom Templates
Create templates in `~/.klp/templates/`:
```
~/.klp/templates/
├── custom-pentest.latex
├── custom-audit.html
└── custom-osint.md
```

## Report Sections

Standard pentest report includes:
1. Executive Summary
2. Scope and Methodology
3. Findings (Critical, High, Medium, Low, Info)
4. Recommendations
5. Appendices

## Source Code

See: `src/skills/report.rs`
