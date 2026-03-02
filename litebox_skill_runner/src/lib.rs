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

    /// Detects the runtime from a script file's shebang or extension
    ///
    /// # Errors
    /// Returns an error if the runtime cannot be determined
    pub fn detect_from_file(path: &Path) -> Result<Self, String> {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            return match ext {
                "sh" => Ok(Self::Shell),
                "js" => Ok(Self::Node),
                "py" => Ok(Self::Python),
                _ => Err(format!("Unsupported file extension: {ext}")),
            };
        }
        Err("Could not determine runtime from file".to_string())
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
    /// Invokes the runtime's interpreter on the given script path and returns
    /// its combined stdout on success, or an error message (including the
    /// process exit code) on failure.
    ///
    /// # Errors
    /// Returns an error if the runtime is unsupported, the interpreter cannot
    /// be spawned, or the script exits with a non-zero status.
    pub fn execute(&self, script_path: &Path, runtime: Runtime) -> Result<String, String> {
        if !runtime.is_supported() {
            return Err(format!(
                "Runtime {runtime:?} is not supported in this environment"
            ));
        }

        let output = std::process::Command::new(runtime.interpreter_path())
            .arg(script_path)
            .output()
            .map_err(|e| format!("Failed to spawn interpreter: {e}"))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).into_owned())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let code = output.status.code().unwrap_or(-1);
            Err(format!("Script exited with code {code}: {stderr}"))
        }
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

    #[test]
    fn test_execute_unsupported_runtime() {
        let runner = SkillRunner::new();
        let result = runner.execute(Path::new("/tmp/test.bash"), Runtime::Bash);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not supported"));
    }

    #[test]
    fn test_execute_shell_script() {
        use std::io::Write as _;
        let script_path = std::env::temp_dir().join("litebox_skill_runner_test_exec.sh");
        let mut f = std::fs::File::create(&script_path).expect("create temp script");
        writeln!(f, "#!/bin/sh\necho hello from litebox_skill_runner").expect("write script");

        let runner = SkillRunner::new();
        let result = runner.execute(&script_path, Runtime::Shell);
        let _ = std::fs::remove_file(&script_path);

        assert!(result.is_ok(), "Expected success, got: {result:?}");
        assert!(result.unwrap().contains("hello from litebox_skill_runner"));
    }

    #[test]
    fn test_execute_failing_script() {
        use std::io::Write as _;
        let script_path = std::env::temp_dir().join("litebox_skill_runner_test_fail.sh");
        let mut f = std::fs::File::create(&script_path).expect("create temp script");
        writeln!(f, "#!/bin/sh\nexit 42").expect("write script");

        let runner = SkillRunner::new();
        let result = runner.execute(&script_path, Runtime::Shell);
        let _ = std::fs::remove_file(&script_path);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("42"));
    }
}
