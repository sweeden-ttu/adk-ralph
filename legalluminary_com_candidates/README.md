# legalluminary_com_candidates

A multi-agent workflow built with [ADK-Rust](https://github.com/adk-rust/adk).

## How It Works

Three specialist agents run in sequence:

1. **Researcher** — gathers key facts on the topic
2. **Writer** — turns the research into an article
3. **Editor** — polishes the final output

The output of each agent feeds into the next via `SequentialAgent`.

## Setup

1. Copy `.env.example` to `.env` and add your API key:
   ```bash
   cp .env.example .env
   ```

2. Build and run:
   ```bash
   cargo run
   ```

3. Enter a topic (e.g. "benefits of Rust for AI") and watch the pipeline run.

## Parallel Execution

You can also run agents concurrently with `ParallelAgent`:

```rust
let team = ParallelAgent::new("analysts", vec![analyst_a, analyst_b]);
```

Or combine both:

```rust
let workflow = SequentialAgent::new("pipeline", vec![
    Arc::new(ParallelAgent::new("research_team", vec![agent_a, agent_b])),
    writer,
    editor,
]);
```

## Project Structure

- `src/main.rs` — Workflow definition
- `.env` — API keys (not committed to git)
- `Cargo.toml` — Rust dependencies

## Learn More

- [ADK-Rust Docs](https://docs.rs/adk-rust)
- [Workflow Agents](https://docs.rs/adk-agent)
- [Gemini API](https://ai.google.dev/docs)
