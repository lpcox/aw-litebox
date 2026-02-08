//! LiteBox Skill Runner
//!
//! This crate provides infrastructure for testing and running Anthropic skills
//! in the LiteBox sandbox environment.

#![warn(missing_docs)]

/// Module for skill definitions and metadata
pub mod skill {
    /// Represents an Anthropic skill with its metadata
    #[derive(Debug, Clone)]
    pub struct Skill {
        /// Name of the skill
        pub name: String,
        /// Primary runtime (sh, node, python)
        pub runtime: Runtime,
        /// Whether the skill requires external dependencies
        pub has_dependencies: bool,
    }

    /// Runtime environments supported by skills
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Runtime {
        /// POSIX shell (/bin/sh)
        Shell,
        /// Node.js/JavaScript
        Node,
        /// Python 3
        Python,
        /// Bash (superset of sh)
        Bash,
    }

    impl Skill {
        /// Create a new skill definition
        pub fn new(name: impl Into<String>, runtime: Runtime) -> Self {
            Self {
                name: name.into(),
                runtime,
                has_dependencies: false,
            }
        }

        /// Mark skill as having external dependencies
        pub fn with_dependencies(mut self) -> Self {
            self.has_dependencies = true;
            self
        }
    }
}

/// Module for test execution
pub mod runner {
    use super::skill::{Runtime, Skill};

    /// Result of a skill test execution
    #[derive(Debug)]
    pub enum TestResult {
        /// Test passed successfully
        Pass,
        /// Test failed with error message
        Fail(String),
        /// Test was skipped (e.g., missing dependencies)
        Skip(String),
    }

    /// Execute a skill test (placeholder for now)
    pub fn run_skill(_skill: &Skill) -> TestResult {
        TestResult::Skip("Execution not yet implemented".to_string())
    }

    impl Runtime {
        /// Check if this runtime is currently supported
        pub fn is_supported(self) -> bool {
            match self {
                Runtime::Shell | Runtime::Node => true, // /bin/sh and Node.js work
                Runtime::Python | Runtime::Bash => false, // Needs automation/missing syscalls
            }
        }
    }
}
