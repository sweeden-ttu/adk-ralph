//! Reusable specialist role prompt templates for the Ralph loop.
//!
//! These role contracts are composed into loop instructions so multiple agents
//! can reuse the same role definitions without duplicating prompt text.

/// Specialist roles used by the Ralph implementation loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialistRole {
    Scaffolder,
    TestEngineer,
    Documenter,
    Optimizer,
    Process,
}

impl SpecialistRole {
    pub fn name(self) -> &'static str {
        match self {
            SpecialistRole::Scaffolder => "Scaffolder Agent (Implementer)",
            SpecialistRole::TestEngineer => "Test Engineer Agent (Quality Guard)",
            SpecialistRole::Documenter => "Documenter Agent (Scribe)",
            SpecialistRole::Optimizer => "Optimizer Agent (Refactoring Partner)",
            SpecialistRole::Process => "Process Agent (Code Supervisor)",
        }
    }

    pub fn prompt(self) -> &'static str {
        match self {
            SpecialistRole::Scaffolder => SCAFFOLDER_PROMPT,
            SpecialistRole::TestEngineer => TEST_ENGINEER_PROMPT,
            SpecialistRole::Documenter => DOCUMENTER_PROMPT,
            SpecialistRole::Optimizer => OPTIMIZER_PROMPT,
            SpecialistRole::Process => PROCESS_PROMPT,
        }
    }
}

const SCAFFOLDER_PROMPT: &str = r#"- Scaffolder Agent:
  - Read relevant files using `file` with operation "read"
  - Write implementation code using `file` with operation "write"
  - Keep implementation scoped to one task"#;

const TEST_ENGINEER_PROMPT: &str = r#"- Test Engineer Agent:
  - Create/update tests for the implementation
  - Define edge cases based on acceptance criteria"#;

const DOCUMENTER_PROMPT: &str = r#"- Documenter Agent:
  - Update README/docs/comments only where changed behavior requires it"#;

const OPTIMIZER_PROMPT: &str = r#"- Optimizer Agent:
  - Refactor obvious clarity/performance issues that are in-task and low risk"#;

const PROCESS_PROMPT: &str = r#"- Process Agent:
  - Critique the combined changes
  - Reflect and prioritize only high-impact fixes before moving on"#;

/// Ordered set of all specialist roles used per task.
pub const SPECIALIST_ROLES: [SpecialistRole; 5] = [
    SpecialistRole::Scaffolder,
    SpecialistRole::TestEngineer,
    SpecialistRole::Documenter,
    SpecialistRole::Optimizer,
    SpecialistRole::Process,
];

/// Build a reusable markdown section describing all specialist role contracts.
pub fn specialist_role_contracts_markdown() -> String {
    let team_line = SPECIALIST_ROLES
        .iter()
        .map(|role| role.name())
        .collect::<Vec<_>>()
        .join(", ");

    let role_prompts = SPECIALIST_ROLES
        .iter()
        .map(|role| role.prompt())
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "You must operate as a coordinated team of specialist sub-agents during each task:\n- {}\n\n{}",
        team_line, role_prompts
    )
}

