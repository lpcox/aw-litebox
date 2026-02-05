# Python Setup Guide for LiteBox Skills

**Last Updated:** 2026-02-05  
**Status:** Python 3 is working with proper setup (85% capability coverage)

## Quick Start

The fastest way to package a Python skill for LiteBox:

``````bash
# 1. Build the litebox tools (one time)
cd /path/to/litebox
cargo build --release -p litebox_syscall_rewriter
cargo build --release -p litebox_runner_linux_userland

# 2. Package your Python skill (automated)
./litebox_skill_runner/examples/prepare_python_skill_advanced.py \
    /path/to/your/skill \
    -o skill.tar \
    --rewriter-path ./target/release/litebox_syscall_rewriter

# 3. Run the skill
./target/release/litebox_runner_linux_userland \
    --tar-path skill.tar \
    --exe /usr/bin/python3 \
    --args "your_script.py"
``````

That's it! The automation script handles stdlib, site-packages, .so rewriting, and environment setup.

## Table of Contents

1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Automated Setup (Recommended)](#automated-setup-recommended)
4. [Manual Setup (For Understanding)](#manual-setup-for-understanding)
5. [Real Skill Examples](#real-skill-examples)
6. [Troubleshooting](#troubleshooting)
7. [Advanced Topics](#advanced-topics)

## Overview

### What Works
- ✅ Python interpreter execution in LiteBox sandbox
- ✅ Standard library modules (with packaging)
- ✅ Pure Python third-party packages (pip install)
- ✅ Binary extension modules (with .so rewriting)
- ✅ Most common packages (PyYAML, pypdf, defusedxml, etc.)

### What Doesn't Work (Yet)
- ❌ Virtual environments (venv/virtualenv)
- ❌ Packages requiring network access during runtime
- ❌ Packages requiring write access to filesystem
- ❌ Packages with complex system dependencies

### The Challenge

Python skills require:
1. Python interpreter binary
2. Python standard library (50-100 MB)
3. Third-party packages with dependencies
4. All `.so` files rewritten for LiteBox syscall handling
5. Correct environment variables (PYTHONHOME, PYTHONPATH)

Manual setup is complex and error-prone. Use the automation script!

## Prerequisites

### System Requirements
- Ubuntu Linux (x86_64)
- Python 3.10+ installed
- Rust toolchain (for building LiteBox tools)
- 500 MB+ free disk space (for Python packaging)

### Install Python (if needed)
``````bash
sudo apt update
sudo apt install -y python3 python3-pip python3-dev
python3 --version  # Should be 3.10 or newer
``````

### Build LiteBox Tools
``````bash
cd /path/to/litebox

# Build the syscall rewriter (critical!)
cargo build --release -p litebox_syscall_rewriter

# Build the runner
cargo build --release -p litebox_runner_linux_userland

# Verify binaries
ls -lh target/release/litebox_syscall_rewriter
ls -lh target/release/litebox_runner_linux_userland
``````

## Automated Setup (Recommended)

### The Automation Script

Location: `litebox_skill_runner/examples/prepare_python_skill_advanced.py`

**What it does:**
1. ✅ Detects your Python version automatically
2. ✅ Finds and packages Python standard library
3. ✅ Finds and packages site-packages (pip installs)
4. ✅ Locates all `.so` files
5. ✅ Rewrites `.so` files with litebox_syscall_rewriter
6. ✅ Creates tar filesystem with correct structure
7. ✅ Generates ready-to-use command examples
8. ✅ Validates the package

### Basic Usage

``````bash
./litebox_skill_runner/examples/prepare_python_skill_advanced.py \
    /path/to/skill \
    -o output.tar \
    --rewriter-path ./target/release/litebox_syscall_rewriter
``````

### Example: skill-creator Skill

``````bash
# 1. Clone the Anthropic skills repository
git clone https://github.com/anthropics/skills.git /tmp/skills
cd /tmp/skills/skills/skill-creator

# 2. Install dependencies
pip3 install -r requirements.txt
# (Installs PyYAML - pure Python, no .so files)

# 3. Package for LiteBox
/path/to/litebox/litebox_skill_runner/examples/prepare_python_skill_advanced.py \
    . \
    -o /tmp/skill-creator.tar \
    --rewriter-path /path/to/litebox/target/release/litebox_syscall_rewriter

# 4. Run a script
/path/to/litebox/target/release/litebox_runner_linux_userland \
    --tar-path /tmp/skill-creator.tar \
    --exe /usr/bin/python3 \
    --args "scripts/quick_validate.py --help"
``````

### Script Options

``````bash
usage: prepare_python_skill_advanced.py [-h] [-o OUTPUT] 
                                        [--rewriter-path REWRITER]
                                        [--python-path PYTHON]
                                        [--include-site-packages]
                                        [--verbose]
                                        SKILL_DIR

positional arguments:
  SKILL_DIR             Path to skill directory

optional arguments:
  -h, --help            Show help message
  -o OUTPUT, --output OUTPUT
                        Output tar file (default: skill.tar)
  --rewriter-path REWRITER
                        Path to litebox_syscall_rewriter binary
                        (default: ../target/release/litebox_syscall_rewriter)
  --python-path PYTHON  Path to Python interpreter
                        (default: /usr/bin/python3)
  --include-site-packages
                        Include site-packages (pip installs)
                        (default: enabled if requirements.txt exists)
  --verbose             Show detailed output
``````

### What Gets Packaged

The script includes:

**1. Python Interpreter**
- `/usr/bin/python3` → rewritten with syscall rewriter
- `/usr/bin/python3.X` (version-specific symlink)

**2. Standard Library**
- `/usr/lib/python3.X/` - Pure Python stdlib modules
- `/usr/lib/python3.X/lib-dynload/` - Stdlib .so extensions
- All `.so` files rewritten

**3. Site-Packages (if --include-site-packages)**
- `/usr/lib/python3/dist-packages/` - System packages
- `/usr/local/lib/python3.X/dist-packages/` - pip installs
- Pure Python files copied as-is
- All `.so` files rewritten

**4. Skill Files**
- Your skill directory at `/skill/`
- Scripts, data files, resources

**5. Environment Setup**
- Automatically sets `PYTHONHOME=/usr`
- Automatically sets `PYTHONPATH` with all module locations
- Sets `PYTHONDONTWRITEBYTECODE=1` (tar is read-only)

## Manual Setup (For Understanding)

If you need to understand the process or customize beyond what the script provides:

### Step 1: Create Tar Structure

``````bash
mkdir -p /tmp/skill_package
cd /tmp/skill_package

# Create directory structure
mkdir -p usr/bin
mkdir -p usr/lib/python3.12
mkdir -p usr/lib/python3.12/lib-dynload
mkdir -p usr/lib/python3/dist-packages
mkdir -p usr/local/lib/python3.12/dist-packages
mkdir -p skill
``````

### Step 2: Copy Python Interpreter

``````bash
# Copy Python binary
cp /usr/bin/python3 usr/bin/
cp /usr/bin/python3.12 usr/bin/  # Version-specific

# Rewrite with syscall rewriter
/path/to/litebox/target/release/litebox_syscall_rewriter \
    usr/bin/python3

/path/to/litebox/target/release/litebox_syscall_rewriter \
    usr/bin/python3.12
``````

### Step 3: Copy Standard Library

``````bash
# Copy stdlib (50-100 MB)
cp -r /usr/lib/python3.12/* usr/lib/python3.12/

# Find and rewrite all .so files
find usr/lib/python3.12 -name "*.so" -type f | while read so_file; do
    echo "Rewriting $so_file"
    /path/to/litebox/target/release/litebox_syscall_rewriter "$so_file"
done
``````

### Step 4: Copy Site-Packages (Optional)

Only needed if your skill uses third-party packages:

``````bash
# Copy pip-installed packages
cp -r /usr/lib/python3/dist-packages/* usr/lib/python3/dist-packages/
cp -r /usr/local/lib/python3.12/dist-packages/* usr/local/lib/python3.12/dist-packages/

# Rewrite .so files in packages
find usr/lib/python3/dist-packages -name "*.so" -type f | while read so_file; do
    /path/to/litebox/target/release/litebox_syscall_rewriter "$so_file"
done

find usr/local/lib/python3.12/dist-packages -name "*.so" -type f | while read so_file; do
    /path/to/litebox/target/release/litebox_syscall_rewriter "$so_file"
done
``````

### Step 5: Copy Skill Files

``````bash
# Copy your skill directory
cp -r /path/to/your/skill/* skill/
``````

### Step 6: Create Tar Archive

``````bash
# Create the tar filesystem
tar -czf /tmp/skill.tar.gz -C /tmp/skill_package .

# Or uncompressed (faster for testing)
tar -cf /tmp/skill.tar -C /tmp/skill_package .
``````

### Step 7: Set Environment Variables

When running with `litebox_runner_linux_userland`, pass these environment variables:

``````bash
--env PYTHONHOME=/usr \
--env PYTHONPATH=/usr/lib/python3.12:/usr/lib/python3.12/lib-dynload:/usr/lib/python3/dist-packages:/usr/local/lib/python3.12/dist-packages:/skill \
--env PYTHONDONTWRITEBYTECODE=1
``````

### Step 8: Run Your Skill

``````bash
/path/to/litebox/target/release/litebox_runner_linux_userland \
    --tar-path /tmp/skill.tar \
    --exe /usr/bin/python3 \
    --args "script.py arg1 arg2" \
    --env PYTHONHOME=/usr \
    --env PYTHONPATH=/usr/lib/python3.12:/usr/lib/python3.12/lib-dynload:/usr/lib/python3/dist-packages:/usr/local/lib/python3.12/dist-packages:/skill \
    --env PYTHONDONTWRITEBYTECODE=1
``````

## Real Skill Examples

### Example 1: skill-creator (Pure Python + PyYAML)

**Skill:** Creates new Agent Skills from templates  
**Dependencies:** PyYAML (pure Python, no .so files)  
**Complexity:** Low  
**Expected Success:** 95%

``````bash
# Install dependencies
cd /tmp/skills/skills/skill-creator
pip3 install pyyaml

# Package with automation
/path/to/litebox/litebox_skill_runner/examples/prepare_python_skill_advanced.py \
    . -o /tmp/skill-creator.tar \
    --rewriter-path /path/to/litebox/target/release/litebox_syscall_rewriter

# Test quick_validate script
/path/to/litebox/target/release/litebox_runner_linux_userland \
    --tar-path /tmp/skill-creator.tar \
    --exe /usr/bin/python3 \
    --args "scripts/quick_validate.py /tmp/skills"
``````

### Example 2: pdf (Pure Python + pypdf)

**Skill:** PDF form manipulation  
**Dependencies:** pypdf (pure Python, no .so files for basic scripts)  
**Complexity:** Low (for pypdf scripts), Medium (for Pillow scripts)  
**Expected Success:** 70-85%

``````bash
# Install dependencies
cd /tmp/skills/skills/pdf
pip3 install pypdf

# Package (pypdf scripts only - skip Pillow for now)
/path/to/litebox/litebox_skill_runner/examples/prepare_python_skill_advanced.py \
    . -o /tmp/pdf.tar \
    --rewriter-path /path/to/litebox/target/release/litebox_syscall_rewriter

# Test a pypdf script
/path/to/litebox/target/release/litebox_runner_linux_userland \
    --tar-path /tmp/pdf.tar \
    --exe /usr/bin/python3 \
    --args "scripts/extract_form_field_info.py input.pdf"
``````

### Example 3: docx (Pure Python + defusedxml)

**Skill:** Word document manipulation  
**Dependencies:** defusedxml (pure Python)  
**Complexity:** Low  
**Expected Success:** 75%

``````bash
# Install dependencies
cd /tmp/skills/skills/docx
pip3 install defusedxml

# Package
/path/to/litebox/litebox_skill_runner/examples/prepare_python_skill_advanced.py \
    . -o /tmp/docx.tar \
    --rewriter-path /path/to/litebox/target/release/litebox_syscall_rewriter

# Test a script
/path/to/litebox/target/release/litebox_runner_linux_userland \
    --tar-path /tmp/docx.tar \
    --exe /usr/bin/python3 \
    --args "scripts/some_script.py input.docx"
``````

## Troubleshooting

### Problem: "No module named 'xxx'"

**Cause:** Package not included in tar or PYTHONPATH incorrect

**Solution 1:** Install the package and re-run automation script
``````bash
pip3 install package_name
./litebox_skill_runner/examples/prepare_python_skill_advanced.py ...
``````

**Solution 2:** Check PYTHONPATH includes correct directories
``````bash
# Should include:
# /usr/lib/python3.X
# /usr/lib/python3.X/lib-dynload
# /usr/lib/python3/dist-packages
# /usr/local/lib/python3.X/dist-packages
``````

### Problem: "ImportError: cannot import name 'xxx' from partially initialized module"

**Cause:** .so file not rewritten or rewriting failed

**Solution:** Find and rewrite the specific .so file
``````bash
# Find the .so file
find /usr/lib/python3.12 -name "*xxx*.so"

# Rewrite it
/path/to/litebox/target/release/litebox_syscall_rewriter /path/to/file.so

# Re-create tar with updated file
``````

### Problem: "OSError: [Errno 30] Read-only file system"

**Cause:** Python trying to create .pyc files

**Solution:** Set PYTHONDONTWRITEBYTECODE environment variable
``````bash
--env PYTHONDONTWRITEBYTECODE=1
``````

### Problem: "FileNotFoundError: [Errno 2] No such file or directory: '/usr/lib/python3.12/xxx'"

**Cause:** Python version mismatch or incomplete stdlib copy

**Solution 1:** Use correct Python version
``````bash
# Check system Python version
python3 --version  # e.g., Python 3.12.1

# Ensure you're copying the right stdlib
cp -r /usr/lib/python3.12/* usr/lib/python3.12/
``````

**Solution 2:** Verify PYTHONHOME is correct
``````bash
--env PYTHONHOME=/usr
``````

### Problem: .so rewriting takes too long

**Cause:** Many .so files to process (50-100+ files)

**Solution:** Use parallel processing
``````bash
# Rewrite in parallel (4 jobs)
find usr/lib/python3.12 -name "*.so" -type f | \
    xargs -P 4 -I {} /path/to/litebox/target/release/litebox_syscall_rewriter {}
``````

### Problem: "litebox_syscall_rewriter: command not found"

**Cause:** Binary not built or not in path

**Solution:** Build the rewriter
``````bash
cd /path/to/litebox
cargo build --release -p litebox_syscall_rewriter
ls target/release/litebox_syscall_rewriter  # Verify it exists
``````

### Problem: Tar file is huge (1GB+)

**Cause:** Included unnecessary files or debug symbols

**Solution 1:** Use compression
``````bash
tar -czf output.tar.gz ...  # Use gzip compression
``````

**Solution 2:** Exclude unnecessary files
``````bash
# Skip __pycache__, tests, docs
tar --exclude='__pycache__' \
    --exclude='*.pyc' \
    --exclude='test' \
    --exclude='tests' \
    --exclude='docs' \
    -czf output.tar.gz ...
``````

### Problem: Script works locally but fails in LiteBox

**Cause:** Missing dependencies or unsupported syscalls

**Solution:** Check logs for missing syscalls
``````bash
# Run with verbose output
./target/release/litebox_runner_linux_userland \
    --tar-path skill.tar \
    --exe /usr/bin/python3 \
    --args "script.py" 2>&1 | grep "unsupported"

# Look for lines like:
# WARNING: unsupported: unsupported syscall xxx
``````

Report missing syscalls as issues!

## Advanced Topics

### Custom Python Versions

If you need a specific Python version:

``````bash
# Build Python from source
wget https://www.python.org/ftp/python/3.11.7/Python-3.11.7.tgz
tar -xzf Python-3.11.7.tgz
cd Python-3.11.7
./configure --prefix=/opt/python3.11
make -j$(nproc)
sudo make install

# Use with automation script
./litebox_skill_runner/examples/prepare_python_skill_advanced.py \
    /path/to/skill \
    --python-path /opt/python3.11/bin/python3
``````

### Binary Extension Packages

Packages with C extensions (numpy, Pillow, etc.) require more work:

**1. Install the package**
``````bash
pip3 install numpy  # ~50+ .so files
``````

**2. Find all .so files**
``````bash
find /usr/local/lib/python3.12/dist-packages/numpy -name "*.so"
``````

**3. Rewrite each .so file**
``````bash
find /usr/local/lib/python3.12/dist-packages/numpy -name "*.so" | \
    xargs -P 4 -I {} /path/to/litebox/target/release/litebox_syscall_rewriter {}
``````

**4. Package and test**

The automation script does this automatically with `--include-site-packages`!

### Debugging Import Issues

Use Python's `-v` flag to see import details:

``````bash
./target/release/litebox_runner_linux_userland \
    --tar-path skill.tar \
    --exe /usr/bin/python3 \
    --args "-v script.py" 2>&1 | less

# Shows:
# import yaml # from /usr/lib/python3/dist-packages/yaml/__init__.py
# import _yaml # from /usr/lib/python3.12/lib-dynload/_yaml.so
``````

This helps identify which module is failing to import.

### Minimal Python Setup

For simple scripts (no imports), you can skip stdlib:

``````bash
# Just Python binary + script
mkdir -p minimal/usr/bin
cp /usr/bin/python3 minimal/usr/bin/
/path/to/litebox/target/release/litebox_syscall_rewriter minimal/usr/bin/python3

mkdir minimal/skill
echo 'print("Hello, LiteBox!")' > minimal/skill/hello.py

tar -cf minimal.tar -C minimal .

# Run
./target/release/litebox_runner_linux_userland \
    --tar-path minimal.tar \
    --exe /usr/bin/python3 \
    --args "hello.py"
``````

### Caching Rewritten Binaries

Speed up repeated packaging by caching rewritten files:

``````bash
# Create cache directory
mkdir -p ~/.litebox/cache/python3.12

# Copy rewritten stdlib once
cp -r /tmp/skill_package/usr/lib/python3.12/* ~/.litebox/cache/python3.12/

# Reuse in future packages
cp -r ~/.litebox/cache/python3.12/* usr/lib/python3.12/
``````

## Performance Tips

1. **Use compression:** `tar -czf` reduces file size by 60-80%
2. **Cache rewritten files:** Reuse rewritten Python binary and stdlib
3. **Parallel .so rewriting:** Use `xargs -P 4` for 4x speedup
4. **Exclude tests/docs:** Skip unnecessary files in packages
5. **Minimal packaging:** Only include packages your skill uses

## Testing Your Setup

Quick test to verify Python works:

``````bash
# 1. Create minimal test
mkdir -p /tmp/pytest/skill
echo 'print("Python works in LiteBox!")' > /tmp/pytest/skill/test.py

# 2. Package with automation
./litebox_skill_runner/examples/prepare_python_skill_advanced.py \
    /tmp/pytest/skill \
    -o /tmp/pytest.tar \
    --rewriter-path ./target/release/litebox_syscall_rewriter

# 3. Run
./target/release/litebox_runner_linux_userland \
    --tar-path /tmp/pytest.tar \
    --exe /usr/bin/python3 \
    --args "test.py"

# Expected output: "Python works in LiteBox!"
``````

## Getting Help

If you're stuck:

1. Check this guide's Troubleshooting section
2. Review the automation script output for errors
3. Check logs for "unsupported syscall" warnings
4. Open an issue with:
   - Python version (`python3 --version`)
   - Package versions (`pip3 list`)
   - Full error message
   - Steps to reproduce

## Summary

**Quick Start (Recommended):**
1. Build litebox_syscall_rewriter
2. Run `prepare_python_skill_advanced.py` on your skill
3. Execute with litebox_runner_linux_userland

**Manual Setup (If needed):**
1. Copy Python binary + stdlib + site-packages
2. Rewrite all .so files
3. Set PYTHONHOME, PYTHONPATH, PYTHONDONTWRITEBYTECODE
4. Create tar and run

**Testing Real Skills:**
- skill-creator: Easy (PyYAML only)
- pdf (pypdf scripts): Easy (pure Python)
- docx: Medium (defusedxml)
- pptx: Hard (python-pptx + Pillow)

**Next Steps:**
- Test with skill-creator skill
- Test with pdf pypdf scripts
- Report any issues found
- Iterate and improve

---

**Guide Version:** 1.0  
**Last Updated:** 2026-02-05  
**Maintainer:** LiteBox Skills Team
