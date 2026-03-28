//! Test the Architect Agent with an existing PRD.
//!
//! Run with: cargo run -p adk-ralph --example test_architect_agent
//!
//! Prerequisites: Run test_prd_agent first to generate prd.md

use adk_ralph::agents::ArchitectAgent;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for debug output
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    // Load .env file
    dotenvy::dotenv().ok();

    // Use the same project directory as PRD test
    let project_path = PathBuf::from("/tmp/ralph-prd-test");

    // Check if PRD exists
    let prd_path = project_path.join("prd.md");
    if !prd_path.exists() {
        println!("❌ PRD not found at: {}", prd_path.display());
        println!("   Run test_prd_agent first to generate the PRD.");
        return Ok(());
    }

    println!("🚀 Testing Architect Agent");
    println!("📁 Project path: {}", project_path.display());
    println!("📄 PRD: {}", prd_path.display());
    println!();

    // Build the Architect agent
    println!("⏳ Building Architect agent...");
    let architect_agent = ArchitectAgent::builder()
        .project_path(&project_path)
        .build()
        .await?;
    println!("✅ Architect agent built");
    println!();

    println!("⏳ Generating design and tasks...");

    // Generate design and tasks
    match architect_agent.generate().await {
        Ok((design, tasks)) => {
            println!("✅ Design and Tasks Generated!");
            println!();
            println!("📊 Design Summary:");
            println!("   Project: {}", design.project);
            println!("   Components: {}", design.components.len());
            for comp in &design.components {
                println!("     - {}: {}", comp.name, comp.purpose);
            }
            println!();
            println!("📋 Tasks Summary:");
            println!("   Project: {}", tasks.project);
            println!("   Total Tasks: {}", tasks.tasks.len());
            for task in &tasks.tasks {
                println!(
                    "     {} - {} (Priority: {}, Complexity: {})",
                    task.id, task.title, task.priority, task.estimated_complexity
                );
            }
            println!();
            println!("📄 Files saved:");
            println!("   - {}/design.md", project_path.display());
            println!("   - {}/tasks.json", project_path.display());
        }
        Err(e) => {
            println!("❌ Error: {}", e);

            // Check if files were created anyway
            let design_path = project_path.join("design.md");
            let tasks_path = project_path.join("tasks.json");

            if design_path.exists() {
                println!(
                    "📄 design.md exists ({} bytes)",
                    std::fs::metadata(&design_path)
                        .map(|m| m.len())
                        .unwrap_or(0)
                );
            }
            if tasks_path.exists() {
                println!(
                    "📄 tasks.json exists ({} bytes)",
                    std::fs::metadata(&tasks_path).map(|m| m.len()).unwrap_or(0)
                );
            }
        }
    }

    Ok(())
}
