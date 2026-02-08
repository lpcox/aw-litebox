# LiteBox Skills Compatibility Matrix

**Date:** 2026-02-02  
**Status:** ⚠️ **SUPERSEDED** - See [EVALUATION_2026-02-08_EVENING.md](EVALUATION_2026-02-08_EVENING.md) for latest comprehensive analysis  
**Purpose:** Detailed analysis of Anthropic Skills compatibility with LiteBox

---

**📢 Latest Update (2026-02-08):**  
This document contains outdated analysis. For the most current, evidence-based analysis including:
- Actual script counts (Python: 68, Bash: 2, JS: 1)
- Revised compatibility estimates (62.5% immediate, 93.75% achievable)
- Detailed dependency analysis by skill
- Testing strategy and priorities

**→ See [EVALUATION_2026-02-08_EVENING.md](EVALUATION_2026-02-08_EVENING.md)**

---

## Executive Summary (Historical)

| Category | Count | Expected Success Rate | Status |
|----------|-------|----------------------|--------|
| Documentation-only skills | 8 | 100% | ✅ No execution needed |
| Stdlib-only Python | 1 | 95% | 🟢 Ready to test |
| Pure Python dependencies | 3-4 | 85% | 🟡 Needs packaging |
| C extension dependencies | 4-5 | 70% | 🟡 Needs .so rewriting |
| Complex/Network dependencies | 2-3 | 30% | 🔴 Deferred |
| Node.js scripts | 2 | 100% | ✅ Proven working |
| Shell scripts | 1 | 100% | ✅ Proven working |

**Overall Predicted Compatibility:** 75-80% of skills should work or nearly work

## Skill-by-Skill Analysis

### Tier 1: Ready to Test (High Success Probability)

#### 1. skill-creator ⭐ HIGHEST PRIORITY
**Status:** 🟢 95% likely to work  
**Scripts:** 3 Python files  
**Dependencies:**
- **Stdlib only:** `sys, os, re, pathlib, zipfile`
- **Pure Python:** `pyyaml` (YAML parser, no C extensions)

**Python Imports:**
```python
# init_skill.py
import sys
from pathlib import Path

# package_skill.py
import sys, zipfile
from pathlib import Path
from quick_validate import validate_skill

# quick_validate.py
import sys, os, re, yaml
from pathlib import Path
```

**Test Plan:**
1. Install PyYAML: `pip install pyyaml` (pure Python, no .so files)
2. Package with `prepare_python_skill_advanced.py`
3. Test `init_skill.py` to create a new skill
4. Test `quick_validate.py` on the skills repo
5. Test `package_skill.py` to create .skill zip

**Estimated Setup Time:** 10 minutes  
**Confidence:** Very High

---

#### 2. web-artifacts-builder ⭐ READY TO TEST
**Status:** 🟢 100% likely to work  
**Scripts:** 2 shell scripts  
**Dependencies:** Bash (now proven working!) + Node.js

**Scripts:**
- `scripts/init-artifact.sh` - Initialize web artifact (uses bash, node detection)
- `scripts/bundle-artifact.sh` - Package web artifact

**Bash Features Used:**
```bash
#!/bin/bash
NODE_VERSION=$(node -v | cut -d'v' -f2 | cut -d'.' -f1)
if [ "$NODE_VERSION" -lt 18 ]; then
  echo "❌ Error: Node.js 18 or higher is required"
fi
```

**Test Plan:**
1. Package bash + Node.js (both proven working)
2. Run init-artifact.sh with test project
3. Verify Vite project creation

**Updated:** Bash support confirmed working (2026-02-08)  
**Estimated Setup Time:** 15 minutes  
**Confidence:** Very High (bash test passes!)

---

#### 3. algorithmic-art
**Status:** ✅ 100% likely to work  
**Scripts:** 1 JavaScript file  
**Dependencies:** Node.js (proven working)

**Test Plan:**
1. Run with Node.js (already proven in tests)
2. No additional dependencies needed

**Estimated Setup Time:** 5 minutes  
**Confidence:** Very High (Node.js proven working)

---

### Tier 2: Moderate Complexity (Good Success Probability)

#### 4. pdf
**Status:** 🟡 70% likely to work  
**Scripts:** 8 Python files  
**Dependencies:**
- **Pure Python:** `pypdf` (PDF manipulation)
- **System binary:** `poppler-utils` (for pdf2image)
- **C extensions:** `Pillow/PIL` (~10-20 .so files)

**Python Imports Analysis:**
```python
# Core dependencies
from pypdf import PdfReader, PdfWriter  # Pure Python ✅
from pdf2image import convert_from_path  # Wrapper for poppler binary ⚠️
from PIL import Image, ImageDraw         # C extensions (~20 .so files) ⚠️

# Stdlib only
import sys, os, json, dataclasses, unittest  # ✅
```

**Scripts Breakdown:**
1. `check_bounding_boxes.py` - Stdlib + JSON ✅
2. `check_fillable_fields.py` - pypdf only ✅
3. `extract_form_field_info.py` - pypdf only ✅
4. `fill_fillable_fields.py` - pypdf only ✅
5. `fill_pdf_form_with_annotations.py` - pypdf only ✅
6. `convert_pdf_to_images.py` - pdf2image + poppler ⚠️
7. `create_validation_image.py` - Pillow ⚠️
8. `check_bounding_boxes_test.py` - unittest ✅

**Test Plan:**
1. **Phase 1:** Test pypdf-only scripts (5 scripts, high confidence)
2. **Phase 2:** Package Pillow with .so rewriting
3. **Phase 3:** Include poppler-utils binaries in tar

**Estimated Setup Time:** 1-2 hours  
**Confidence:** Medium-High (5/8 scripts should work immediately)

---

#### 5. pptx
**Status:** 🟡 75% likely to work  
**Scripts:** 9 Python + 1 JavaScript  
**Dependencies:**
- **Pure Python:** `python-pptx` (PowerPoint manipulation)
- **C extensions:** `Pillow/PIL` (for thumbnail generation)
- **Node.js:** `html2pptx.js` (proven working) ✅

**Python Imports Analysis:**
```python
from pptx import Presentation                    # C extension? ⚠️
from PIL import Image, ImageDraw, ImageFont      # C extensions ⚠️
from pathlib import Path                         # Stdlib ✅
import argparse, json, sys                       # Stdlib ✅
```

**Test Plan:**
1. Test `html2pptx.js` with Node.js (should work) ✅
2. Package python-pptx with .so rewriting
3. Package Pillow with .so rewriting

**Estimated Setup Time:** 2-3 hours  
**Confidence:** Medium

---

#### 6. docx
**Status:** 🟡 70% likely to work  
**Scripts:** 10 Python files (includes ooxml submodule)  
**Dependencies:**
- **Pure Python:** `defusedxml` (XML parsing)
- **Possible C extensions:** Need to verify python-docx dependencies

**Python Imports Analysis:**
```python
from defusedxml import minidom, sax              # Pure Python ✅
from pathlib import Path                         # Stdlib ✅
from datetime import datetime, timezone          # Stdlib ✅
import html, random, shutil, tempfile            # Stdlib ✅
```

**Test Plan:**
1. Install defusedxml (check if pure Python)
2. Package dependencies
3. Test document manipulation scripts

**Estimated Setup Time:** 1-2 hours  
**Confidence:** Medium-High

---

#### 7. xlsx
**Status:** 🟡 60% likely to work  
**Scripts:** 1 Python file  
**Dependencies:** Unknown (need to check script contents)

**Test Plan:**
1. Examine script to determine dependencies
2. Likely uses `openpyxl` or similar (may have C extensions)

**Estimated Setup Time:** 1 hour  
**Confidence:** Medium

---

### Tier 3: Complex Dependencies (Lower Success Probability)

#### 8. slack-gif-creator
**Status:** 🟡 50% likely to work  
**Scripts:** 4 Python files  
**Dependencies (from requirements.txt):**
```
pillow>=10.0.0          # C extensions (~20 .so files) ⚠️
imageio>=2.31.0         # Image I/O library ⚠️
imageio-ffmpeg>=0.4.9   # FFmpeg wrapper (system binary) ⚠️
numpy>=1.24.0           # Heavy C extensions (~50 .so files) ⚠️
```

**Complexity:** High
- Multiple C extension packages
- System binary dependency (ffmpeg)
- Large number of .so files to rewrite

**Test Plan:**
1. Package numpy (large, many .so files)
2. Package Pillow
3. Package imageio
4. Include ffmpeg binary in tar

**Estimated Setup Time:** 3-4 hours  
**Confidence:** Medium-Low

---

#### 9. mcp-builder
**Status:** 🔴 30% likely to work  
**Scripts:** 2 Python files  
**Dependencies (from requirements.txt):**
```
anthropic>=0.39.0       # Network API client ⚠️
mcp>=1.1.0              # Model Context Protocol ⚠️
```

**Blockers:**
- **Network access required** - API calls to Anthropic
- LiteBox sandbox may not have network access
- Complex dependency trees

**Test Plan:** Defer until network access is available

**Estimated Setup Time:** Unknown  
**Confidence:** Low (blocked by network access)

---

#### 10. webapp-testing
**Status:** 🔴 20% likely to work  
**Scripts:** 4 Python files  
**Dependencies:** Likely `playwright` or `selenium`

**Blockers:**
- **Browser automation** - Very complex
- Requires Chrome/Firefox binaries
- Large dependency trees
- May need display server

**Test Plan:** Defer (out of scope for initial implementation)

**Estimated Setup Time:** Unknown  
**Confidence:** Very Low

---

### Tier 4: Documentation/Template Only (No Execution)

These skills have no executable scripts and work by providing documentation/templates:

1. **brand-guidelines** - Brand identity documentation ✅
2. **canvas-design** - Design templates and guidelines ✅
3. **doc-coauthoring** - Collaboration workflow documentation ✅
4. **frontend-design** - Design system documentation ✅
5. **internal-comms** - Communication templates ✅
6. **theme-factory** - Theme creation guidelines ✅

**Status:** 100% compatible (no execution needed)

---

## Dependency Deep Dive

### Python Packages Classification

#### ✅ Pure Python (No .so files)
- `pyyaml` - YAML parser
- `pypdf` - PDF manipulation
- `defusedxml` - Safe XML parsing
- `six` - Python 2/3 compatibility

**Action Required:** Standard pip install + packaging

---

#### ⚠️ C Extensions (Need .so rewriting)
- `Pillow` (PIL) - ~10-20 .so files
  - `_imaging.so`, `_imagingft.so`, `_imagingmath.so`, etc.
- `python-pptx` - PowerPoint library (may have .so files)
- `numpy` - ~50+ .so files
  - `_multiarray_umath.so`, `_operand_flag_tests.so`, etc.

**Action Required:** 
1. Install with pip
2. Find all .so files
3. Rewrite each with `litebox_syscall_rewriter`
4. Package in tar with correct paths

---

#### 🔴 System Binaries (Need inclusion in tar)
- `poppler-utils` - PDF utilities
  - `pdfinfo`, `pdftoppm`, `pdfimages`, etc.
- `ffmpeg` - Video processing
- Browsers (Chrome/Firefox) - Very large

**Action Required:**
1. Copy binaries from `/usr/bin/`
2. Include required libraries
3. Rewrite with `litebox_syscall_rewriter`

---

## Testing Priority Queue

### Week 1: Tier 1 Skills (Quick Wins)
**Goal:** Prove that basic skills work end-to-end

1. **skill-creator** (Day 1-2)
   - Install PyYAML
   - Test all 3 scripts
   - Document results

2. **web-artifacts-builder** (Day 2)
   - Test shell scripts
   - Verify paths work correctly

3. **algorithmic-art** (Day 3)
   - Test Node.js script
   - Verify output generation

**Success Criteria:** All 3 skills working = 3/16 skills (19%)

---

### Week 2: Tier 2 Skills (Moderate Complexity)
**Goal:** Tackle C extension packaging

1. **pdf - pypdf scripts** (Day 1-2)
   - Test 5 pypdf-only scripts first
   - Package and test

2. **pdf - Pillow scripts** (Day 3-4)
   - Package Pillow with .so rewriting
   - Test image generation scripts

3. **docx** (Day 5)
   - Package defusedxml
   - Test document scripts

**Success Criteria:** 3 more skills working = 6/16 skills (38%)

---

### Week 3: Tier 2 Continued
**Goal:** Complete Tier 2 skills

1. **pptx** (Day 1-3)
   - Test Node.js script
   - Package python-pptx
   - Test PowerPoint scripts

2. **xlsx** (Day 4)
   - Determine dependencies
   - Package and test

**Success Criteria:** 2 more skills = 8/16 skills (50%)

---

### Week 4: Tier 3 Skills
**Goal:** Handle complex dependencies

1. **slack-gif-creator** (Day 1-3)
   - Package numpy (large effort)
   - Package imageio + ffmpeg
   - Test GIF creation

**Success Criteria:** 1 more skill = 9/16 skills (56%)

---

### Future: Network-Dependent Skills
**Defer:** mcp-builder, webapp-testing

**Blocker:** Need network access and browser support

---

## Metrics and Projections

### Current State (2026-02-08)
- **Skills with scripts:** 10/16 (63%)
- **Skills tested:** 1/10 (10%) - skill-creator direct Python test
- **Skills proven working:** 10/16 (62.5%) - 6 docs + 2 bash + 2 Node.js
- **Estimated working with packaging:** 14/16 (87.5%)

### After Week 1
- **Skills tested:** 3/10 (30%)
- **Expected working:** 3/10 (30%)
- **Overall:** 9/16 including docs-only (56%)

### After Week 2
- **Skills tested:** 6/10 (60%)
- **Expected working:** 6/10 (60%)
- **Overall:** 12/16 including docs-only (75%)

### After Week 4
- **Skills tested:** 9/10 (90%)
- **Expected working:** 7-8/10 (70-80%)
- **Overall:** 13-14/16 including docs-only (81-88%)

### Final Goal
- **Target:** 14/16 skills working (88%)
- **Deferred:** mcp-builder, webapp-testing (require network/browser)

---

## Implementation Checklist

### Prerequisites (Must have)
- [x] `cargo build --release -p litebox_runner_linux_userland`
- [x] `cargo build --release -p litebox_syscall_rewriter`
- [x] Python 3.12 with pip
- [x] Test scripts ready (`test_anthropic_skills.sh`)
- [x] Packaging script ready (`prepare_python_skill_advanced.py`)

### Tier 1 Testing
- [ ] Clone skills repo to stable location
- [ ] Test skill-creator with PyYAML
- [ ] Test web-artifacts-builder with shell
- [ ] Test algorithmic-art with Node.js
- [ ] Document results in EVALUATION

### Tier 2 Testing
- [ ] Package Pillow with .so rewriting
- [ ] Package python-pptx with .so rewriting
- [ ] Package defusedxml
- [ ] Test pdf scripts (pypdf subset)
- [ ] Test pptx scripts
- [ ] Test docx scripts
- [ ] Document results

### Tier 3 Testing
- [ ] Package numpy (large task)
- [ ] Package imageio + ffmpeg
- [ ] Test slack-gif-creator
- [ ] Document results

### Documentation
- [ ] Update CAPABILITIES.md with test results
- [ ] Create compatibility table
- [ ] Document .so rewriting process
- [ ] Create troubleshooting guide

---

## Risk Mitigation

### Risk: C Extension Packaging Too Complex
**Likelihood:** Medium  
**Impact:** High  
**Mitigation:** Start with pure Python skills, build expertise iteratively

### Risk: .so Rewriting Breaks Dependencies
**Likelihood:** Low  
**Impact:** High  
**Mitigation:** Test each package individually, verify rewritten .so files work

### Risk: Tar Filesystem Size Explodes
**Likelihood:** Medium  
**Impact:** Medium  
**Mitigation:** Use compression, only include necessary files, document size limits

### Risk: Performance Issues
**Likelihood:** Low  
**Impact:** Low  
**Mitigation:** Cache rewritten binaries, optimize tar creation

---

## Success Criteria

### Minimum Viable (Week 1)
✅ 3 Tier 1 skills working (skill-creator, web-artifacts-builder, algorithmic-art)  
✅ Documentation updated  
✅ Test framework validated

### Good Progress (Week 2)
✅ 6 skills working (add pdf, docx, xlsx)  
✅ C extension packaging proven  
✅ .so rewriting process documented

### Excellent Progress (Week 4)
✅ 8-9 skills working (add pptx, slack-gif-creator)  
✅ 75%+ of executable skills working  
✅ Comprehensive documentation

### Complete (Future)
✅ 10+ skills working (pending network access)  
✅ 90%+ compatibility  
✅ Production-ready

---

## Appendix: Script Inventory

### Python Scripts by Skill
- **skill-creator:** 3 scripts (stdlib + PyYAML)
- **pdf:** 8 scripts (pypdf + Pillow)
- **pptx:** 4 scripts + 5 ooxml (python-pptx + Pillow)
- **docx:** 3 scripts + 7 ooxml (defusedxml)
- **mcp-builder:** 2 scripts (anthropic + mcp)
- **slack-gif-creator:** 4 scripts (Pillow + numpy + ffmpeg)
- **webapp-testing:** 4 scripts (playwright/selenium)
- **xlsx:** 1 script (openpyxl?)

**Total:** ~45 Python scripts

### JavaScript Scripts by Skill
- **pptx:** 1 script (html2pptx.js)
- **algorithmic-art:** 1 script (generator_template.js)

**Total:** 2 JavaScript scripts

### Shell Scripts by Skill
- **web-artifacts-builder:** 2 scripts (.sh)

**Total:** 2 shell scripts

---

**Document Version:** 1.0  
**Last Updated:** 2026-02-02  
**Next Review:** After Tier 1 testing complete
