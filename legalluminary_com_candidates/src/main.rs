//! LegalluminaryComCandidates — A multi-agent workflow built with ADK-Rust
//!
//! Demonstrates sequential agent orchestration using SequentialAgent.

use adk_rust::prelude::*;
use adk_rust::Launcher;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    let api_key = std::env::var("GOOGLE_API_KEY")
        .expect("GOOGLE_API_KEY must be set in .env file");

    let model = Arc::new(GeminiModel::new(&api_key, "gemini-2.5-flash")?);

    // --- Specialist agents ---

    let researcher: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("researcher")
            .description("Gathers information on a topic")
            .instruction(
                "You are a research assistant. Given a topic, provide a concise summary \
                 of the key facts, recent developments, and important context.",
            )
            .model(model.clone())
            .build()?,
    );

    let writer: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("writer")
            .description("Writes engaging content from research")
            .instruction(
                "You are a skilled writer. Take the research provided and produce a clear, \
                 engaging article of about 300 words. Use a professional but approachable tone.",
            )
            .model(model.clone())
            .build()?,
    );

    let editor: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("editor")
            .description("Reviews and polishes written content")
            .instruction(
                "You are an editor. Review the article for clarity, grammar, and style. \
                 Output the improved version.",
            )
            .model(model.clone())
            .build()?,
    );

    // --- Sequential pipeline: Research → Write → Edit ---

    let pipeline = SequentialAgent::new(
        "LegalluminaryComCandidates",
        vec![researcher, writer, editor],
    );

    // Launch the pipeline in interactive console mode
    Launcher::new(Arc::new(pipeline)).run().await?;

    Ok(())
}
