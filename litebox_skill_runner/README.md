# LiteBox Skill Runner

A Rust library for executing [Anthropic Agent Skills](https://github.com/anthropics/skills) in LiteBox's sandboxed environment.

## Overview

The `litebox_skill_runner` crate provides the infrastructure to:
- Detect and load Anthropic Agent Skills from directories
- Identify the required runtime (shell, Node.js, Python, Bash)
- Execute scripts in a sandboxed LiteBox environment
- Manage dependencies and execution environments

## Supported Runtimes

| Runtime | Status | Notes |
|---------|--------|-------|
| Shell (`/bin/sh`) | ✅ Fully Supported | POSIX shell scripts work perfectly |
| Node.js | ✅ Fully Supported | JavaScript execution works out of the box |
| Python 3 | ✅ Supported | Works with manual setup |
| Bash | ⚠️ Partial Support | Requires `getpgrp` syscall implementation |

## Usage

```rust
use litebox_skill_runner::{Skill, SkillRunner, Runtime};
use std::path::Path;

// Load a skill from a directory
let skill = Skill::load_from_directory(Path::new("skills/my-skill"))?;

// Create a runner
let runner = SkillRunner::new();

// Execute a script
let output = runner.execute(Path::new("script.sh"), Runtime::Shell)?;
```

## Skill Structure

An Anthropic Agent Skill consists of:
```
skill-name/
├── SKILL.md          # Metadata and instructions (required)
├── scripts/          # Executable scripts (optional)
│   ├── setup.sh
│   ├── main.py
│   └── helper.js
└── resources/        # Additional files (optional)
```

## Current Status

- ✅ Basic skill loading and detection
- ✅ Runtime identification from file extensions and shebangs
- ✅ Type-safe runtime representation
- ✅ **Script execution with `std::process::Command`**
- ✅ Output capture and error handling

### Next Steps
1. Test execution with Node.js and Python scripts
2. Add YAML frontmatter parsing for SKILL.md files
3. Implement dependency packaging and environment setup
4. Add comprehensive integration tests with real Anthropic skills
5. Support environment variables and working directory configuration

## Examples

See the `tests/` directory for usage examples.

## License

MIT License. See [LICENSE](../LICENSE) for details.
