# Anthropic Skills Testing Plan

**Version:** 1.0  
**Date:** 2026-02-05  
**Purpose:** Systematic validation of LiteBox compatibility with all Anthropic Skills

## Overview

This document provides a comprehensive testing methodology for validating LiteBox's ability to run all 16 skills from the [Anthropic Skills Repository](https://github.com/anthropics/skills).

**Current Status:**
- Skills tested: 0/16 (0%)
- Expected compatibility: 81-88% (13-14 skills)
- Confirmed compatibility: 0% (awaiting testing)

**Goal:** Test all skills systematically, document results, identify gaps, and iterate to 100% compatibility.

## Testing Methodology

### Test Phases

**Phase 1: Tier 1 Skills (Week 1)**
- Priority: HIGHEST
- Skills: 3 (skill-creator, web-artifacts-builder, algorithmic-art)
- Expected success: 95-100%
- Goal: Prove basic capability and validate testing process

**Phase 2: Tier 2 Skills (Week 2-3)**
- Priority: HIGH
- Skills: 5 (pdf, docx, pptx, xlsx, theme-factory)
- Expected success: 60-75%
- Goal: Validate Python packaging automation with C extensions

**Phase 3: Tier 3 Skills (Week 4)**
- Priority: MEDIUM
- Skills: 2 (slack-gif-creator, brand-guidelines)
- Expected success: 40-60%
- Goal: Handle complex dependencies

**Phase 4: Deferred Skills**
- Priority: LOW
- Skills: 2 (mcp-builder, webapp-testing)
- Expected success: 20-30%
- Blocker: Network access and browser automation

**Documentation Skills:** 4 skills (doc-coauthoring, canvas-design, frontend-design, internal-comms)
- No executable testing needed (100% compatible by design)

### Test Execution Workflow

For each skill:

1. **Setup** - Clone skill, install dependencies
2. **Package** - Use automation script to create LiteBox tar
3. **Test** - Run all scripts with various inputs
4. **Document** - Record results, errors, and observations
5. **Debug** - If failures, investigate and fix
6. **Iterate** - Re-test after fixes
7. **Report** - Update compatibility matrix

### Success Criteria

**Per Skill:**
- ✅ All scripts execute without crashing
- ✅ Output is correct and matches expected behavior
- ✅ No unsupported syscall errors
- ✅ Reasonable performance (within 2-5x native)

**Overall:**
- ✅ 10+ skills working (63%)
- ✅ All Tier 1 skills working (100%)
- ✅ Clear documentation of failures
- ✅ Bug reports for each blocking issue

## Tier 1 Skills (Week 1)

### 1. skill-creator ⭐ HIGHEST PRIORITY

**Description:** Creates new Agent Skills from templates  
**Language:** Python 3  
**Dependencies:** PyYAML (pure Python, no .so files)  
**Scripts:** 3 Python files  
**Complexity:** Low  
**Expected Success:** 95%

#### Setup

``````bash
# Clone skills repo
git clone https://github.com/anthropics/skills.git /tmp/skills
cd /tmp/skills/skills/skill-creator

# Install dependencies
pip3 install -r requirements.txt
# Installs: pyyaml>=6.0
``````

#### Package for LiteBox

``````bash
# Use automation script
/path/to/litebox/litebox_skill_runner/examples/prepare_python_skill_advanced.py \
    . \
    -o /tmp/skill-creator.tar \
    --rewriter-path /path/to/litebox/target/release/litebox_syscall_rewriter \
    --verbose
``````

#### Test Cases

**Test 1: quick_validate.py - Validate skills directory**
``````bash
./target/release/litebox_runner_linux_userland \
    --tar-path /tmp/skill-creator.tar \
    --exe /usr/bin/python3 \
    --args "scripts/quick_validate.py /tmp/skills"

# Expected: Validation report for all skills
# Success: No errors, prints validation summary
``````

**Test 2: init_skill.py - Create new skill**
``````bash
./target/release/litebox_runner_linux_userland \
    --tar-path /tmp/skill-creator.tar \
    --exe /usr/bin/python3 \
    --args "scripts/init_skill.py my-test-skill /tmp/output"

# Expected: New skill directory created at /tmp/output/my-test-skill
# Success: Directory exists with SKILL.md and basic structure
``````

**Test 3: package_skill.py - Package skill to .skill file**
``````bash
./target/release/litebox_runner_linux_userland \
    --tar-path /tmp/skill-creator.tar \
    --exe /usr/bin/python3 \
    --args "scripts/package_skill.py /tmp/skills/skills/algorithmic-art /tmp/output"

# Expected: Creates algorithmic-art.skill (zip file)
# Success: File exists and is valid zip
``````

#### Expected Results

| Test | Expected Outcome | Pass/Fail | Notes |
|------|------------------|-----------|-------|
| quick_validate.py | Validation report | ? | First test |
| init_skill.py | New skill created | ? | Directory structure test |
| package_skill.py | .skill file created | ? | Zip creation test |

#### Potential Issues

- ❌ PyYAML import fails → Check site-packages included
- ❌ File I/O errors → Check tar filesystem paths
- ❌ Permission errors → Check write access to output dir

#### Debug Steps

If tests fail:
1. Check Python import verbose: `python3 -v scripts/quick_validate.py`
2. Verify PyYAML installed: `pip3 show pyyaml`
3. Check .so files rewritten: `ldd` on any .so files
4. Review logs for unsupported syscalls

---

### 2. web-artifacts-builder

**Description:** Create and update web artifacts  
**Language:** Shell scripts (bash/sh)  
**Dependencies:** None (pure shell)  
**Scripts:** 2 shell scripts  
**Complexity:** Very Low  
**Expected Success:** 100%

#### Setup

``````bash
cd /tmp/skills/skills/web-artifacts-builder
# No dependencies to install!
``````

#### Package for LiteBox

``````bash
# Shell scripts work out of the box, but need to package skill directory
mkdir -p /tmp/web-tar/skill
cp -r /tmp/skills/skills/web-artifacts-builder/* /tmp/web-tar/skill/
tar -cf /tmp/web-artifacts-builder.tar -C /tmp/web-tar .
``````

#### Test Cases

**Test 1: init-artifact.sh - Initialize artifact**
``````bash
./target/release/litebox_runner_linux_userland \
    --tar-path /tmp/web-artifacts-builder.tar \
    --exe /bin/sh \
    --args "skill/scripts/init-artifact.sh my-artifact /tmp/output"

# Expected: Creates new artifact directory
# Success: Directory exists with index.html
``````

**Test 2: update-artifact.sh - Update existing artifact**
``````bash
./target/release/litebox_runner_linux_userland \
    --tar-path /tmp/web-artifacts-builder.tar \
    --exe /bin/sh \
    --args "skill/scripts/update-artifact.sh /tmp/output/my-artifact content.html"

# Expected: Updates artifact content
# Success: Content updated in artifact
``````

#### Expected Results

| Test | Expected Outcome | Pass/Fail | Notes |
|------|------------------|-----------|-------|
| init-artifact.sh | Directory created | ? | Shell I/O test |
| update-artifact.sh | Content updated | ? | File manipulation test |

#### Potential Issues

- ❌ /bin/sh not in tar → Include shell binary
- ❌ Path issues → Verify skill/ prefix correct

#### Debug Steps

1. Test shell directly: `./scripts/init-artifact.sh` locally
2. Check tar contents: `tar -tvf /tmp/web-artifacts-builder.tar`
3. Verify paths match between tar and command

---

### 3. algorithmic-art

**Description:** Generate algorithmic art with JavaScript  
**Language:** JavaScript (Node.js)  
**Dependencies:** Node.js (proven working)  
**Scripts:** 1 JavaScript file  
**Complexity:** Low  
**Expected Success:** 100%

#### Setup

``````bash
cd /tmp/skills/skills/algorithmic-art
# No npm packages needed for basic template
``````

#### Package for LiteBox

``````bash
# Node.js binary is auto-detected by runner, just package skill
mkdir -p /tmp/art-tar/skill
cp -r /tmp/skills/skills/algorithmic-art/* /tmp/art-tar/skill/

# Include Node.js (if not system-installed)
# The runner will handle Node.js dependencies automatically

tar -cf /tmp/algorithmic-art.tar -C /tmp/art-tar .
``````

#### Test Cases

**Test 1: generator_template.js - Generate art**
``````bash
./target/release/litebox_runner_linux_userland \
    --tar-path /tmp/algorithmic-art.tar \
    --exe /usr/bin/node \
    --args "skill/templates/generator_template.js"

# Expected: Outputs SVG or canvas art code
# Success: Valid SVG/HTML output
``````

**Test 2: With parameters**
``````bash
./target/release/litebox_runner_linux_userland \
    --tar-path /tmp/algorithmic-art.tar \
    --exe /usr/bin/node \
    --args "skill/templates/generator_template.js --seed 12345"

# Expected: Deterministic art from seed
# Success: Output matches expected pattern
``````

#### Expected Results

| Test | Expected Outcome | Pass/Fail | Notes |
|------|------------------|-----------|-------|
| generator_template.js | SVG output | ? | Node.js baseline |
| With parameters | Seeded output | ? | Parameter passing |

#### Potential Issues

- ❌ Node.js binary missing → Ensure /usr/bin/node or /usr/local/bin/node in tar
- ❌ Module import errors → Check if script uses require()

#### Debug Steps

1. Test Node.js directly: Check existing Node.js tests in litebox
2. Run script locally: `node templates/generator_template.js`
3. Check Node.js version compatibility

---

## Tier 2 Skills (Week 2-3)

### 4. pdf

**Description:** PDF form manipulation and extraction  
**Language:** Python 3  
**Dependencies:** pypdf (pure Python), Pillow (C extensions), pdf2image (system binary)  
**Scripts:** 8 Python files  
**Complexity:** Medium  
**Expected Success:** 70% (pypdf scripts), 50% (Pillow scripts)

#### Phased Testing Approach

**Phase 2A: pypdf-only scripts (5 scripts)**
- No C extensions, should work immediately
- Expected success: 85%

**Phase 2B: Pillow scripts (3 scripts)**
- Requires .so rewriting
- Expected success: 60%

#### Setup

``````bash
cd /tmp/skills/skills/pdf

# Phase 2A: pypdf only
pip3 install pypdf

# Phase 2B: Add Pillow
pip3 install pillow pdf2image
``````

#### Package for LiteBox

``````bash
# Phase 2A: pypdf only
/path/to/litebox/litebox_skill_runner/examples/prepare_python_skill_advanced.py \
    . \
    -o /tmp/pdf-pypdf.tar \
    --rewriter-path /path/to/litebox/target/release/litebox_syscall_rewriter

# Phase 2B: Full with Pillow
/path/to/litebox/litebox_skill_runner/examples/prepare_python_skill_advanced.py \
    . \
    -o /tmp/pdf-full.tar \
    --rewriter-path /path/to/litebox/target/release/litebox_syscall_rewriter \
    --include-site-packages
``````

#### Test Cases (Phase 2A - pypdf)

**Test 1: extract_form_field_info.py**
``````bash
./target/release/litebox_runner_linux_userland \
    --tar-path /tmp/pdf-pypdf.tar \
    --exe /usr/bin/python3 \
    --args "scripts/extract_form_field_info.py sample.pdf"
``````

**Test 2: check_fillable_fields.py**
``````bash
./target/release/litebox_runner_linux_userland \
    --tar-path /tmp/pdf-pypdf.tar \
    --exe /usr/bin/python3 \
    --args "scripts/check_fillable_fields.py sample.pdf"
``````

**Test 3-5:** Similar for other pypdf scripts

#### Test Cases (Phase 2B - Pillow)

After Phase 2A succeeds:

**Test 6: convert_pdf_to_images.py**
``````bash
./target/release/litebox_runner_linux_userland \
    --tar-path /tmp/pdf-full.tar \
    --exe /usr/bin/python3 \
    --args "scripts/convert_pdf_to_images.py sample.pdf /tmp/output"
``````

#### Expected Results

| Phase | Scripts | Expected | Notes |
|-------|---------|----------|-------|
| 2A (pypdf) | 5 | 85% | Pure Python |
| 2B (Pillow) | 3 | 60% | C extensions |

#### Potential Issues

- ❌ pypdf import fails → Check site-packages
- ❌ Pillow .so errors → Check all .so files rewritten
- ❌ poppler not found → Need to include system binary

---

### 5. docx

**Description:** Word document manipulation  
**Language:** Python 3  
**Dependencies:** defusedxml (pure Python)  
**Scripts:** 10 Python files (includes ooxml submodule)  
**Complexity:** Medium  
**Expected Success:** 75%

#### Setup

``````bash
cd /tmp/skills/skills/docx
pip3 install defusedxml
``````

#### Package for LiteBox

``````bash
/path/to/litebox/litebox_skill_runner/examples/prepare_python_skill_advanced.py \
    . \
    -o /tmp/docx.tar \
    --rewriter-path /path/to/litebox/target/release/litebox_syscall_rewriter
``````

#### Test Cases

Test representative scripts from scripts/ and ooxml/scripts/:

**Test 1-3:** Basic document operations  
**Test 4-6:** OOXML manipulation  
**Test 7-10:** Advanced features

#### Expected Results

| Scripts | Expected Success | Notes |
|---------|------------------|-------|
| 10 | 75% (7-8 scripts) | XML parsing heavy |

---

### 6. pptx

**Description:** PowerPoint manipulation  
**Language:** Python 3 + Node.js  
**Dependencies:** python-pptx (C extensions?), Pillow, Node.js  
**Scripts:** 9 Python + 1 JavaScript  
**Complexity:** Medium-High  
**Expected Success:** 75%

#### Phased Testing

**Phase A: Node.js script (html2pptx.js)**
- Expected: 100% (Node.js proven working)

**Phase B: Python scripts**
- Expected: 70% (C extension challenges)

#### Test approach similar to pdf skill

---

### 7. xlsx

**Description:** Excel spreadsheet manipulation  
**Language:** Python 3  
**Dependencies:** openpyxl (may have C extensions)  
**Scripts:** 1 Python file  
**Complexity:** Medium  
**Expected Success:** 60%

#### Setup and testing similar to docx

---

## Tier 3 Skills (Week 4)

### 8. slack-gif-creator

**Description:** Create animated GIFs for Slack  
**Language:** Python 3  
**Dependencies:** Pillow, numpy (~50+ .so files), imageio, ffmpeg  
**Scripts:** 4 Python files  
**Complexity:** HIGH  
**Expected Success:** 50%

#### Challenge

- numpy has 50+ .so files to rewrite
- ffmpeg binary dependency
- Memory-intensive operations

#### Phased Testing

1. **Phase A:** Basic imports (numpy, Pillow)
2. **Phase B:** Image manipulation
3. **Phase C:** GIF creation with ffmpeg

#### Expected to identify significant gaps

---

## Deferred Skills

### 9. mcp-builder

**Blocker:** Network access required (Anthropic API calls)  
**Expected:** 30% (blocked by infrastructure)  
**Defer:** Until network access implemented

### 10. webapp-testing

**Blocker:** Browser automation (Playwright/Selenium)  
**Expected:** 20% (very complex dependencies)  
**Defer:** Out of scope for initial implementation

---

## Documentation-Only Skills

These skills have no executable scripts and work through documentation/templates:

1. ✅ brand-guidelines (100%)
2. ✅ canvas-design (100%)
3. ✅ doc-coauthoring (100%)
4. ✅ frontend-design (100%)
5. ✅ internal-comms (100%)
6. ✅ theme-factory (100%)

**Testing:** Verify SKILL.md can be parsed and read. No execution needed.

---

## Test Results Template

For each skill, document using this template:

``````markdown
## Skill: [skill-name]

**Date Tested:** YYYY-MM-DD  
**Tester:** [name]  
**LiteBox Version:** [commit hash]

### Setup
- Dependencies installed: [list]
- Tar size: [size in MB]
- Packaging time: [seconds]

### Test Results

| Test | Command | Expected | Actual | Pass/Fail | Notes |
|------|---------|----------|--------|-----------|-------|
| 1 | ... | ... | ... | ✅/❌ | ... |
| 2 | ... | ... | ... | ✅/❌ | ... |

### Summary
- **Scripts tested:** X/Y
- **Scripts passing:** X/Y (Z%)
- **Overall result:** ✅ Working / ⚠️ Partial / ❌ Blocked

### Issues Found
1. [Issue description] - [Bug report link]
2. [Issue description] - [Bug report link]

### Recommendations
- [Next steps]
- [Improvements needed]
``````

---

## Tracking Progress

### Overall Metrics

Track in `SKILLS_COMPATIBILITY_MATRIX.md`:

``````markdown
| Skill | Status | Scripts | Passing | % | Last Tested |
|-------|--------|---------|---------|---|-------------|
| skill-creator | 🟢 | 3 | 3 | 100% | 2026-02-06 |
| web-artifacts | ✅ | 2 | 2 | 100% | 2026-02-06 |
| ... | ... | ... | ... | ... | ... |
``````

### Weekly Milestones

**Week 1:**
- ✅ Tier 1 complete (3/3 skills)
- ✅ Testing methodology validated
- ✅ ~19% skills confirmed working

**Week 2:**
- ✅ Tier 2 Phase A (pypdf, defusedxml skills)
- ⚠️ Tier 2 Phase B started (C extensions)
- ✅ ~40% skills tested

**Week 3:**
- ✅ Tier 2 complete
- ✅ ~65% skills tested
- ✅ ~50% skills confirmed working

**Week 4:**
- ✅ Tier 3 attempted
- ✅ All testable skills validated
- ✅ ~80% skills tested
- ✅ Comprehensive compatibility report

---

## Bug Reporting

When issues are found, create bug reports with:

**Title:** `[skill-name] [issue-summary]`

**Example:** `[skill-creator] PyYAML import fails - module not found`

**Template:**
``````markdown
## Bug Report

**Skill:** [skill-name]  
**Script:** [script-name]  
**Severity:** Critical / High / Medium / Low

### Description
[Clear description of the issue]

### Steps to Reproduce
1. [Step 1]
2. [Step 2]
3. [Step 3]

### Expected Behavior
[What should happen]

### Actual Behavior
[What actually happens]

### Error Output
```
[Full error message and logs]
```

### Environment
- LiteBox commit: [hash]
- Python version: [version]
- System: [Ubuntu version]

### Potential Root Cause
[Initial analysis]

### Suggested Fix
[If known]
``````

---

## Automation Scripts

### Bulk Testing Script

``````bash
#!/bin/bash
# test_all_tier1.sh - Test all Tier 1 skills

SKILLS_DIR="/tmp/skills"
LITEBOX_DIR="/path/to/litebox"
OUTPUT_DIR="/tmp/test-results"

mkdir -p "$OUTPUT_DIR"

# skill-creator
echo "Testing skill-creator..."
cd "$SKILLS_DIR/skills/skill-creator"
python3 "$LITEBOX_DIR/litebox_skill_runner/examples/prepare_python_skill_advanced.py" \
    . -o "$OUTPUT_DIR/skill-creator.tar" \
    --rewriter-path "$LITEBOX_DIR/target/release/litebox_syscall_rewriter"

"$LITEBOX_DIR/target/release/litebox_runner_linux_userland" \
    --tar-path "$OUTPUT_DIR/skill-creator.tar" \
    --exe /usr/bin/python3 \
    --args "scripts/quick_validate.py $SKILLS_DIR" \
    > "$OUTPUT_DIR/skill-creator-results.txt" 2>&1

# web-artifacts-builder
echo "Testing web-artifacts-builder..."
# ... similar ...

# algorithmic-art
echo "Testing algorithmic-art..."
# ... similar ...

echo "All Tier 1 tests complete. Results in $OUTPUT_DIR/"
``````

---

## Success Metrics

### Tier 1 (Week 1)
- ✅ 3/3 skills tested (100%)
- ✅ 3/3 skills working (100%)
- ✅ 0 critical bugs

### Tier 2 (Week 2-3)
- ✅ 5/5 skills tested (100%)
- ✅ 4/5 skills working (80%)
- ⚠️ 0-2 critical bugs identified and fixed

### Overall (Week 4)
- ✅ 10/10 testable skills tested (100%)
- ✅ 8/10 testable skills working (80%)
- ✅ 14/16 total skills compatible (88%) with docs-only
- ✅ All critical bugs fixed or documented

---

## Next Steps After Testing

1. **Document Results**
   - Update CAPABILITIES.md
   - Update SKILLS_COMPATIBILITY_MATRIX.md
   - Create detailed test reports

2. **Fix Identified Issues**
   - Prioritize by impact
   - Implement syscall additions
   - Improve packaging automation

3. **Iterate**
   - Re-test after fixes
   - Track improvements
   - Aim for 100% coverage

4. **Production Readiness**
   - Performance benchmarks
   - Security audit
   - Documentation polish
   - User guide

---

**Plan Version:** 1.0  
**Created:** 2026-02-05  
**Status:** Ready for execution  
**Next Review:** After Week 1 testing complete
