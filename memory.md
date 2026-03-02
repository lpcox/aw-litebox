# Repo Assist Memory

## Last Updated
2026-03-02 05:50 UTC

## Run History
| Date | Run ID | Tasks |
|------|--------|-------|
| 2026-03-02 | 22563211887 | Task 10 (implement execute()), Task 11 (monthly summary update) |
| 2026-03-01 | 22537079240 | Task 4 (deps check), Task 5 (PR check), Task 11 (monthly summary - new March issue) |
| 2026-02-28 | 22514533142 | Task 3 (improvements - runtime detection PR), Task 11 (monthly summary) |
| 2026-02-27 | 22474570910 | Task 7 (labels #101-102), Task 11 (monthly summary) |
| 2026-02-26 | 22429715043 | Task 7 (labels), Task 9 (welcome - no new contributors), Task 11 (monthly summary) |
| 2026-02-25 | 22406141163 | Task 1 (triage), Task 7 (labels), Task 11 (monthly summary) |

## Tasks Last Run
- Task 1 (Triage): 2026-02-25
- Task 2 (Fix Issues): never
- Task 3 (Improvements): 2026-02-28
- Task 4 (Dependencies): 2026-03-01 (cargo-outdated unavailable; minimal external deps)
- Task 5 (Maintain PRs): 2026-03-01
- Task 6 (Stale Nudges): never
- Task 7 (Labels): 2026-02-27
- Task 8 (Release Prep): never
- Task 9 (Welcome): 2026-02-26
- Task 10 (Forward): 2026-03-02 - implemented SkillRunner::execute()
- Task 11 (Monthly Summary): 2026-03-02

## Monthly Summary Issue
- Month: 2026-03
- Issue number: #161
- Status: open
- Previous (Feb): #151 - CLOSED

## Issue Backlog Cursor
- Last processed: #102 (labelling run 2026-02-27)
- Total open issues: ~97 (new automated issues 159, 160 created Mar 1)
- Labelling status: Issues #89-#100, #103-#160 need labelling check; most are automated workflow summaries (off topic)
- Next run: Resume labelling from #103

## Comments Made
- #44: 2026-02-25 - flagged as stale draft PR
- #55: 2026-02-25 - flagged as stale draft PR

## Labels Applied
- #44: wontfix
- #55: wontfix
- #37, #43, #48, #49, #52, #53: off topic
- #56–#88 (30 issues): off topic (2026-02-26)
- #101, #102: off topic (2026-02-27)
- Note: Issues #89-#100, #103-#160 need labelling check; automated workflow summaries should get 'off topic'

## PR Status Notes
- PR #44: Draft, stale - targeting copilot/update-syscall-count-analysis (not main). Fix already in main via #51. Labelled wontfix. Commented 2026-02-25.
- PR #91: MERGED into main via merge commit a3beb4e
- PR #131: First skills testing docs. Documentation-only. Open since Feb 22.
- PR #134: skill-creator testing. Documentation-only. Open since Feb 23.
- PR #139: Complete validation coverage. Documentation-only. Open since Feb 23.
- PR #54, #66, #72, #78, #90, #97: Older analysis PRs, likely superseded.
- PR #158 (Repo Assist): feat(skill_runner): improve Runtime detection - branch repo-assist/improve-runtime-detection-27656f6b4c1997ea. CI has NOT been triggered since PR creation (2026-02-28). Code verified locally (8/8 tests pass, clippy clean). May need maintainer to approve CI run.
- PR (new, 2026-03-02): feat(skill_runner): implement SkillRunner::execute() - branch repo-assist/implement-skill-runner-execute-2026-03-02. 8/8 tests pass, clippy clean.

## CI Observation
- CI last ran on 2026-02-25 (on main branch - failure).
- PR #158 created 2026-02-28 but no CI run triggered.
- CI trigger: pull_request events + push to main
- Possible issue: GitHub Actions may require maintainer approval for bot-created PRs

## Maintainer Checked-Off Items
(none yet)

## Fix Attempts
(none yet)

## Improvement Ideas Submitted
- Runtime detection improvement (2026-02-28): .bash extension + shebang detection PR created. Branch: repo-assist/improve-runtime-detection-27656f6b4c1997ea, PR #158.
- SkillRunner::execute() implementation (2026-03-02): std::process::Command-based execution. Branch: repo-assist/implement-skill-runner-execute-2026-03-02. All 8 tests pass.

## Key Codebase Facts
- litebox_skill_runner/src/lib.rs: Runtime::Node.interpreter_path() returns "/usr/bin/node" - actual CI path is /usr/local/bin/node (but execute() now uses this via Command, so Node.js tests may fail in CI)
- Runtime::Bash.is_supported() returns false (comment: "Requires getpgrp syscall") - intentional for LiteBox sandbox context
- SkillRunner::execute() now implemented with std::process::Command (direct execution, not LiteBox-sandboxed)
- SkillRunner::execute() does NOT yet use self.dependencies (future improvement: inject into PATH)
- All .rs files must have Microsoft copyright header (enforced by dev_tests)
- Many open issues (~97) are automated workflow status summaries
- clippy::pedantic is enabled with RUSTFLAGS=-Dwarnings; doc_markdown lints require backtick-quoting of identifiers like `LiteBox`
- No release tags exist in the repo (git tag list is empty)
- No cargo-outdated installed; workspace has minimal external deps (skill_runner has none)

## Next Run Priority
1. Task 1 (Triage): Resume from cursor #103 - scan open issues
2. Task 2 (Fix Issues): Look for fixable bugs - NEVER run before
3. Task 6 (Stale Nudges): PRs #54, #66, #72, #78 are from bots so no nudge; skip
4. Task 7 (Labels): Resume labelling from #103
5. Task 8 (Release Prep): No tags, no releases - check if meaningful changes warrant release candidate
