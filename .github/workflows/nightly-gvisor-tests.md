---
description: Nightly workflow that analyzes gVisor syscall tests to ensure complete syscall coverage for LiteBox skills
on:
  schedule: daily
permissions:
  contents: read
  issues: read
  pull-requests: read
tools:
  github:
    toolsets: [default]
  serena: ["rust"]
  web-fetch:
network:
  allowed:
    - github.com
    - api.github.com
    - raw.githubusercontent.com
safe-outputs:
  create-pull-request:
    title-prefix: "[gvisor-tests] "
    reviewers: ["lpcox"]
    draft: false
  add-comment:
    max: 2
  noop:
  missing-tool:
    create-issue: true
---

{{#runtime-import agentics/nightly-gvisor-tests.md}}
