//! Reusable specialist role prompt templates for the Ralph loop.
//!
//! These role contracts are composed into loop instructions so multiple agents
//! can reuse the same role definitions without duplicating prompt text.

pub mod documenter;
pub mod assets_manager_data_analyst;
pub mod optimizer;
pub mod process;
pub mod scaffolder;
pub mod test_engineer;

use assets_manager_data_analyst::ASSETS_MANAGER_DATA_ANALYST_PROMPT;
use documenter::DOCUMENTER_PROMPT;
use optimizer::OPTIMIZER_PROMPT;
use process::PROCESS_PROMPT;
use scaffolder::SCAFFOLDER_PROMPT;
use test_engineer::TEST_ENGINEER_PROMPT;

/// Specialist roles used by the Ralph implementation loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialistRole {
    Scaffolder,
    TestEngineer,
    Documenter,
    AssetsManagerDataAnalyst,
    Optimizer,
    Process,
}

impl SpecialistRole {
    pub fn name(self) -> &'static str {
        match self {
            SpecialistRole::Scaffolder => "Scaffolder Agent (Implementer)",
            SpecialistRole::TestEngineer => "Test Engineer Agent (Quality Guard)",
            SpecialistRole::Documenter => "Documenter Agent (Scribe)",
            SpecialistRole::AssetsManagerDataAnalyst => {
                "Assets Manager and Data Analyst Agent (Integrator/Librarian)"
            }
            SpecialistRole::Optimizer => "Optimizer Agent (Refactoring Partner)",
            SpecialistRole::Process => "Process Agent (Code Supervisor)",
        }
    }

    pub fn prompt(self) -> &'static str {
        match self {
            SpecialistRole::Scaffolder => SCAFFOLDER_PROMPT,
            SpecialistRole::TestEngineer => TEST_ENGINEER_PROMPT,
            SpecialistRole::Documenter => DOCUMENTER_PROMPT,
            SpecialistRole::AssetsManagerDataAnalyst => ASSETS_MANAGER_DATA_ANALYST_PROMPT,
            SpecialistRole::Optimizer => OPTIMIZER_PROMPT,
            SpecialistRole::Process => PROCESS_PROMPT,
        }
    }
}

/// Ordered set of all specialist roles used per task.
pub const SPECIALIST_ROLES: [SpecialistRole; 6] = [
    SpecialistRole::Scaffolder,
    SpecialistRole::TestEngineer,
    SpecialistRole::Documenter,
    SpecialistRole::AssetsManagerDataAnalyst,
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
