# LiteBox Skill Runner

A tool for executing [Agent Skills](https://agentskills.io) within LiteBox sandboxed environments.

## Overview

Agent Skills are modular packages that extend AI capabilities by providing specialized knowledge, workflows, and tools. This tool enables running skill scripts within a LiteBox sandbox on Ubuntu/x86 Linux systems.

## Features

- Parse and extract `.skill` files (zip archives containing SKILL.md and bundled resources)
- Execute skill scripts within LiteBox sandbox
- Support for Python scripts (with limitations noted below)

## Known Limitations

1. **No Shell Support**: LiteBox currently does not support running a shell (`/bin/sh`, `/bin/bash`). This means:
   - Shell scripts (`.sh` files) cannot be executed directly
   - Skills that rely on shell features will not work
   - Only direct binary execution (e.g., Python interpreter) is supported

2. **Python Library Packaging**: Running Python scripts requires:
   - Packaging Python standard libraries into the tar filesystem
   - Setting `PYTHONHOME` and `PYTHONPATH` environment variables
   - Rewriting syscalls in Python binary and shared libraries (`.so` files)
   
3. **Stateless Assumption**: Skills are assumed to be stateless for now

## Usage

### Basic Command

```bash
litebox_skill_runner <skill-path> --script <script-path> [script-args...]
```

### Options

- `<skill-path>`: Path to .skill file (zip) or skill directory
- `--script <path>`: Script to execute within the skill (relative path from skill root, e.g., `scripts/init_skill.py`)
- `--runner-path`: Path to litebox_runner_linux_userland binary (default: `../target/release/litebox_runner_linux_userland`)
- `--python-path`: Python interpreter path (default: `/usr/bin/python3`)
- `[script-args...]`: Additional arguments to pass to the script

### Example: Running skill-creator

The `skill-creator` skill provides tools for creating new agent skills. Here's how to run it:

```bash
# Assuming you have the skills repository cloned
cd /path/to/skills

# Run the init_skill.py script
litebox_skill_runner \
    skills/skill-creator \
    --script scripts/init_skill.py \
    my-new-skill --path /tmp/output
```

## Building

```bash
cargo build --release -p litebox_skill_runner
```

The binary will be available at `target/release/litebox_skill_runner`.

## Implementation Details

### Skill Structure

A skill consists of:
- `SKILL.md`: Metadata (YAML frontmatter) and instructions
- `scripts/`: Optional executable scripts (Python, Bash, etc.)
- `references/`: Optional reference documentation
- `assets/`: Optional asset files (templates, images, etc.)

### Execution Flow

1. Load and parse skill (from .skill zip or directory)
2. Extract SKILL.md metadata (name, description)
3. Create tar archive containing all skill resources
4. Execute script via litebox_runner_linux_userland with:
   - `--initial-files` pointing to the tar archive
   - `--interception-backend seccomp` for syscall interception
   - Appropriate interpreter (Python for .py files)

### Filesystem Layout

Within the LiteBox sandbox, the skill is mounted at `/skill/`:
```
/skill/
  ├── SKILL.md
  ├── scripts/
  ├── references/
  └── assets/
```

Scripts are executed with paths relative to the skill root (e.g., `/skill/scripts/init_skill.py`).

## Future Work

- Add shell support in LiteBox to enable running shell scripts
- Simplify Python library packaging
- Support for other interpreters (Node.js, Ruby, etc.)
- Interactive skill execution with stdin/stdout
- Better error handling and diagnostics
- Package manager integration for skill dependencies

## Example Skills

See the [Anthropic Skills Repository](https://github.com/anthropics/skills) for examples:
- `skill-creator`: Create new skills
- `pdf-editor`: PDF manipulation
- `docx-editor`: Document editing
- Many more...

## Contributing

Contributions are welcome! Please see the main LiteBox CONTRIBUTING.md for guidelines.

## License

MIT License - see LICENSE file for details.
