//! Prompt contract for the Scaffolder role.

pub const SCAFFOLDER_PROMPT: &str = r#"- Scaffolder Agent:
  - Read relevant files using `file` with operation "read"
  - Write implementation code using `file` with operation "write"
  - Keep implementation scoped to one task
  - Replace mocked/placeholder implementation artifacts before completion (e.g., fake data files, stub scripts, TODO-only modules)"#;
