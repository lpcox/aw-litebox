// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//! `LiteBox` Skill Runner
//!
//! This crate provides functionality to execute Anthropic Agent Skills within `LiteBox`'s
//! sandboxed environment. It supports shell scripts, Node.js, Python, and Bash execution.

#![warn(missing_docs)]
#![warn(clippy::pedantic)]

use std::path::Path;

/// Supported runtime environments for skill execution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Runtime {
    /// POSIX shell (/bin/sh)
    Shell,
    /// Node.js JavaScript runtime
    Node,
    /// Python 3 interpreter
    Python,
    /// Bash shell
    Bash,
}

impl Runtime {
    /// Returns the default interpreter path for this runtime
    #[must_use]
    pub fn interpreter_path(&self) -> &'static str {
        match self {
            Self::Shell => "/bin/sh",
            Self::Node => "/usr/bin/node",
            Self::Python => "/usr/bin/python3",
            Self::Bash => "/bin/bash",
        }
    }

    /// Detects the runtime from a script file's extension or shebang line.
    ///
    /// First checks the file extension (`.sh`, `.js`, `.py`, `.bash`). If the
    /// file has no recognized extension, reads the first line and matches
    /// against common shebang patterns.
    ///
    /// # Errors
    /// Returns an error if the runtime cannot be determined from either the
    /// extension or the shebang line.
    pub fn detect_from_file(path: &Path) -> Result<Self, String> {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            return match ext {
                "sh" => Ok(Self::Shell),
                "js" => Ok(Self::Node),
                "py" => Ok(Self::Python),
                "bash" => Ok(Self::Bash),
                _ => Err(format!("Unsupported file extension: .{ext}")),
            };
        }

        // Fall back to shebang detection for extensionless files
        Self::detect_from_shebang(path)
            .ok_or_else(|| format!("Could not determine runtime for: {}", path.display()))
    }

    /// Reads the first line of a file and returns the runtime indicated by
    /// a `#!` shebang, or `None` if no shebang is recognized.
    fn detect_from_shebang(path: &Path) -> Option<Self> {
        use std::io::{BufRead, BufReader};
        let file = std::fs::File::open(path).ok()?;
        let mut lines = BufReader::new(file).lines();
        let first_line = lines.next()?.ok()?;

        if !first_line.starts_with("#!") {
            return None;
        }

        // Extract interpreter path and optional argument (e.g. `env bash`)
        let mut parts = first_line[2..].split_whitespace();
        let interpreter = parts.next().unwrap_or("").trim_end_matches('/');

        // When the interpreter is `env`, the actual runtime is the first argument
        let name = if interpreter.rsplit('/').next() == Some("env") {
            parts.next().unwrap_or("")
        } else {
            interpreter.rsplit('/').next().unwrap_or(interpreter)
        };

        match name {
            "sh" => Some(Self::Shell),
            "bash" => Some(Self::Bash),
            "node" | "nodejs" => Some(Self::Node),
            "python" | "python3" => Some(Self::Python),
            _ => None,
        }
    }

    /// Returns whether this runtime is currently supported in `LiteBox`
    #[must_use]
    pub fn is_supported(&self) -> bool {
        match self {
            Self::Shell | Self::Node | Self::Python => true,
            Self::Bash => false, // Requires getpgrp syscall
        }
    }
}

/// Represents an Anthropic Agent Skill
#[derive(Debug, Clone)]
pub struct Skill {
    /// Name of the skill
    pub name: String,
    /// Description of what the skill does
    pub description: String,
    /// Path to the skill directory
    pub path: std::path::PathBuf,
}

impl Skill {
    /// Creates a new Skill from a directory containing a SKILL.md file
    ///
    /// # Errors
    /// Returns an error if the skill cannot be loaded
    pub fn load_from_directory(dir: &Path) -> Result<Self, String> {
        let skill_file = dir.join("SKILL.md");
        if !skill_file.exists() {
            return Err(format!("SKILL.md not found in {}", dir.display()));
        }

        // For now, use the directory name as the skill name
        // A full implementation would parse the YAML frontmatter
        let name = dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        Ok(Self {
            name: name.clone(),
            description: format!("Skill: {name}"),
            path: dir.to_path_buf(),
        })
    }

    /// Lists all script files in the skill directory
    ///
    /// # Errors
    /// Returns an error if the directory cannot be read
    pub fn list_scripts(&self) -> Result<Vec<std::path::PathBuf>, String> {
        let scripts_dir = self.path.join("scripts");
        if !scripts_dir.exists() {
            return Ok(Vec::new());
        }

        Ok(std::fs::read_dir(&scripts_dir)
            .map_err(|e| format!("Failed to read scripts directory: {e}"))?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.is_file())
            .collect::<Vec<_>>())
    }
}

/// Builder for configuring skill execution
#[derive(Debug, Default)]
pub struct SkillRunner {
    /// Additional dependencies to include
    dependencies: Vec<String>,
}

impl SkillRunner {
    /// Creates a new `SkillRunner`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a dependency path to include in the execution environment
    #[must_use]
    pub fn with_dependencies(mut self, deps: Vec<String>) -> Self {
        self.dependencies = deps;
        self
    }

    /// Executes a script with the specified runtime
    ///
    /// # Errors
    /// Returns an error if execution fails
    pub fn execute(&self, _script_path: &Path, _runtime: Runtime) -> Result<String, String> {
        // Placeholder implementation
        // A full implementation would:
        // 1. Validate the runtime is supported
        // 2. Prepare the execution environment
        // 3. Use litebox_runner to execute the script
        // 4. Capture and return output
        Err("Execution not yet implemented".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_interpreter_paths() {
        assert_eq!(Runtime::Shell.interpreter_path(), "/bin/sh");
        assert_eq!(Runtime::Node.interpreter_path(), "/usr/bin/node");
        assert_eq!(Runtime::Python.interpreter_path(), "/usr/bin/python3");
        assert_eq!(Runtime::Bash.interpreter_path(), "/bin/bash");
    }

    #[test]
    fn test_runtime_detection_from_extension() {
        assert_eq!(
            Runtime::detect_from_file(Path::new("script.sh")).unwrap(),
            Runtime::Shell
        );
        assert_eq!(
            Runtime::detect_from_file(Path::new("app.js")).unwrap(),
            Runtime::Node
        );
        assert_eq!(
            Runtime::detect_from_file(Path::new("main.py")).unwrap(),
            Runtime::Python
        );
        assert_eq!(
            Runtime::detect_from_file(Path::new("run.bash")).unwrap(),
            Runtime::Bash
        );
    }

    #[test]
    fn test_runtime_detection_unknown_extension() {
        assert!(Runtime::detect_from_file(Path::new("file.zsh")).is_err());
    }

    #[test]
    fn test_runtime_detection_from_shebang() {
        use std::io::Write;
        let dir = tempfile::tempdir().unwrap();

        let cases: &[(&str, &[u8], Runtime)] = &[
            ("sh_script", b"#!/bin/sh\necho hi\n", Runtime::Shell),
            (
                "bash_script",
                b"#!/usr/bin/env bash\necho hi\n",
                Runtime::Bash,
            ),
            (
                "node_script",
                b"#!/usr/bin/node\nconsole.log('hi')\n",
                Runtime::Node,
            ),
            (
                "py_script",
                b"#!/usr/bin/python3\nprint('hi')\n",
                Runtime::Python,
            ),
        ];

        for (name, content, expected) in cases {
            let path = dir.path().join(name);
            std::fs::File::create(&path)
                .unwrap()
                .write_all(content)
                .unwrap();
            assert_eq!(
                Runtime::detect_from_file(&path).unwrap(),
                *expected,
                "failed for {name}"
            );
        }
    }

    #[test]
    fn test_runtime_detection_no_shebang() {
        use std::io::Write;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("no_shebang");
        std::fs::File::create(&path)
            .unwrap()
            .write_all(b"echo hi\n")
            .unwrap();
        assert!(Runtime::detect_from_file(&path).is_err());
    }

    #[test]
    fn test_runtime_support() {
        assert!(Runtime::Shell.is_supported());
        assert!(Runtime::Node.is_supported());
        assert!(Runtime::Python.is_supported());
        assert!(!Runtime::Bash.is_supported());
    }

    #[test]
    fn test_skill_runner_creation() {
        let runner = SkillRunner::new();
        assert!(runner.dependencies.is_empty());
    }

    #[test]
    fn test_skill_runner_with_dependencies() {
        let runner =
            SkillRunner::new().with_dependencies(vec!["dep1".to_string(), "dep2".to_string()]);
        assert_eq!(runner.dependencies.len(), 2);
    }
}
