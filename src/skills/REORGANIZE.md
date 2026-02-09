# Skills Reorganization Plan

## Current Structure (Flat)
```
skills/
├── *.rs           # All skill files at root level
├── mod.rs          # Skills module
└── skill_tree.md   # Skill reference
```

## Target Structure (Organized by Subfolders)
```
skills/
├── core/           # Core/internal skills
│   ├── mod.rs
│   ├── sys.rs
│   └── sys.md
│
├── git/            # Git-related skills
│   ├── mod.rs
│   ├── git.rs
│   └── git.md
│
├── browser/        # Browser automation
│   ├── mod.rs
│   ├── chrome.rs
│   └── chrome.md
│
├── network/        # Network/connectivity skills
│   ├── mod.rs
│   ├── connect.rs
│   └── connect.md
│
├── monitoring/     # Monitoring skills
│   ├── mod.rs
│   ├── monitor.rs
│   └── monitor.md
│
├── security/       # Security skills
│   ├── mod.rs
│   ├── osint.rs
│   ├── osint.md
│   ├── pentest.rs
│   └── pentest.md
│
├── report/         # Reporting skills
│   ├── mod.rs
│   ├── report.rs
│   └── report.md
│
├── creation/       # Skill/sub-agent creation
│   ├── mod.rs
│   ├── skill_creator.rs
│   ├── skill_creator.md
│   ├── skill-creator/
│   └── subagent-creator/
│
├── maintenance/   # Maintenance skills
│   ├── mod.rs
│   └── selfdestruct.rs
│
├── git_shortcut/  # NEW: Git shortcut skill
│   ├── mod.rs
│   ├── skill.rs
│   └── SKILL.md
│
├── mod.rs         # Main skills module (updated)
└── skill_tree.md  # Updated skill reference
```

## Migration Steps

1. Create subfolders for each skill category
2. Move corresponding .rs and .md files into subfolders
3. Update mod.rs to reference new locations
4. Create SKILL.md files for each skill
5. Update skill_tree.md with new structure

