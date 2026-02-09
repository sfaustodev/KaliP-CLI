---
name: chrome-automation
description: Automate Chrome/Chromium browser operations including navigation, screenshots, form filling, and data extraction. Use for web scraping, automated testing, browser-based workflows, or taking screenshots. Triggers on "chrome", "browser", "web automation", "screenshot", "scrape".
---

# Chrome Automation

Control Chrome/Chromium browser programmatically for automation tasks.

## Quick Start

```bash
# Launch Chrome
klp chrome open

# Navigate to URL
klp chrome navigate https://example.com

# Take screenshot
klp chrome screenshot --output screenshot.png

# Execute JavaScript
klp chrome eval "document.title"

# Fill form
klp chrome form --selector "#username" --value "admin"
```

## Commands

### Browser Control
- `open` - Launch Chrome/Chromium
- `close` - Close browser
- `navigate <url>` - Navigate to URL
- `back` / `forward` - Browser navigation
- `reload` - Refresh page

### Page Interaction
- `screenshot` - Capture page screenshot
- `eval <script>` - Execute JavaScript
- `click <selector>` - Click element
- `type <selector> <text>` - Type into input
- `form --selector <sel> --value <val>` - Fill form field

### Data Extraction
- `text <selector>` - Extract text content
- `html <selector>` - Extract HTML
- `links` - Extract all links
- `images` - Extract image URLs

### Automation
- `auto <action>` - Execute predefined action
- `schedule <task> --interval <seconds>` - Schedule recurring task

## Examples

### Web Scraping
```bash
# Navigate and extract data
klp chrome navigate https://news.ycombinator.com
klp chrome text ".titleline > a"
```

### Form Automation
```bash
# Login automation
klp chrome navigate https://example.com/login
klp chrome form --selector "#username" --value "user"
klp chrome form --selector "#password" --value "pass"
klp chrome click "#login-button"
```

### Scheduled Tasks
```bash
# Take hourly screenshots
klp chrome schedule "screenshot --output /tmp/hourly.png" --interval 3600
```

## Configuration

Set Chrome path in `~/.klp/config.toml`:
```toml
[chrome]
executable = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
headless = true
default_viewport = "1920x1080"
```

## Safety

- Browser runs in isolated context
- Supports headless mode for servers
- Respects robots.txt
- Rate limiting to prevent abuse

## Source Code

See: `src/skills/chrome.rs`
