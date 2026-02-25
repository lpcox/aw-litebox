---
description: Autonomous agent that implements support for shell scripts, Node.js, and Python in LiteBox to run all Anthropic skills. Runs once per day with a full rust/crate development environment and GitHub integration for PR creation and commenting.
on:
  schedule:
    - cron: "0 0 * * *"
permissions:
  contents: read
  issues: read
  pull-requests: read
steps:
  - name: Set up Rust toolchain
    run: |
      rustup toolchain install stable --profile minimal --no-self-update --component rustfmt,clippy --target x86_64-unknown-linux-gnu
  - name: Set up Nextest
    uses: taiki-e/install-action@v2
    with:
      tool: nextest@0.9.114
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
    - crates.io
safe-outputs:
  create-pull-request:
    title-prefix: "[litebox-skills] "
    reviewers: ["lpcox"]
    draft: false
  add-comment:
    max: 2
  create-issue:
    title-prefix: "[litebox-skills] "
    labels: [automation, litebox-skills]
    assignees: [lpcox]
    max: 3
  noop:
  missing-tool:
    create-issue: true
---

{{#runtime-import agentics/litebox-skills.md}}
