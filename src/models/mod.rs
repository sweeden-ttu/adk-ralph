//! Data models for Ralph multi-agent autonomous development system.
//!
//! This module contains:
//! - PRD data structures for requirements management
//! - Design document structures for architecture
//! - Task list structures for implementation planning
//! - Progress log structures for tracking learnings
//! - Configuration management for multi-agent support

pub mod config;
pub mod design;
pub mod prd;
pub mod progress;
pub mod tasks;

// Re-export public API
pub use config::{
    AgentModelConfig, DebugLevel, MAX_ITERATIONS_LIMIT, MAX_RETRIES_LIMIT, MAX_TOKENS_LIMIT,
    ModelConfig, RalphConfig, RalphConfigBuilder, SUPPORTED_PROVIDERS, TelemetryConfig,
    ValidationError,
};
pub use design::{Component, DesignDocument, FileStructure, TechnologyStack};
pub use prd::{AcceptanceCriterion, PrdDocument, PrdStats, UserStory};
pub use progress::{ProgressEntry, ProgressLog, ProgressSummary, TestResults};
pub use tasks::{Phase, Sprint, Task, TaskComplexity, TaskList, TaskStatus};
