//! Agent implementations for the Ralph multi-agent autonomous development system.
//!
//! This module provides specialized agents for each phase of the development pipeline:
//! - [`PrdAgent`] - Generates structured requirements from user prompts
//! - [`ArchitectAgent`] - Creates system design and task breakdown from PRD
//! - [`RalphLoopAgent`] - Iteratively implements tasks until completion using
//!   specialist sub-agent roles (Scaffolder, Test Engineer, Documenter,
//!   Optimizer, Process Agent)

pub mod architect_agent;
pub mod loop_agent;
pub mod prd_agent;
pub mod role_prompts;

pub use architect_agent::ArchitectAgent;
pub use loop_agent::{CompletionStatus, RalphLoopAgent, RalphLoopAgentBuilder};
pub use prd_agent::{PrdAgent, PrdAgentBuilder, PrdResult};
