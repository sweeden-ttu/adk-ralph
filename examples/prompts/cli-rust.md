# Evidence Validation Orchestrator CLI - Rust

Create a Rust CLI tool called `evidenced` for operating the Trustworthy AI legal/government content validation pipeline.

## Purpose

A command-line orchestrator that validates extracted claims and citations against authoritative sources, generates provenance evidence bundles, and prepares datasets for downstream RAG indexing.

## Features

- **Validation Runs**
  - Run validation jobs from JSON or NDJSON claim input files
  - Support content types: legal news, judges, officials, elections, laws, court documents, templates
  - Verify required metadata fields (source URL, publication date, jurisdiction, citation format)
  - Route each item to pass/fail with machine-readable reasons

- **Source Adapters**
  - Pull reference records from configured source adapters
  - Include adapters for Texas Open Data and Texas Capitol Data endpoints
  - Cache source responses locally with TTL and deterministic cache keys
  - Support offline replay mode from snapshot files for reproducible testing

- **Evidence and Provenance**
  - Generate evidence bundles containing matched record IDs, snippets, timestamps, and confidence
  - Emit signed provenance manifests (JSON) for each run
  - Export pass/fail summaries as JSON and CSV
  - Include explainability fields describing why each claim passed or failed

- **Operations**
  - Dry-run mode for schema and configuration checks only
  - Resume interrupted runs using checkpoint files
  - Progress display and structured logs with correlation IDs
  - Configurable fail-fast or continue-on-error behavior

## Technical Requirements

- Use `clap` (derive) for subcommands and flags
- Use `serde`/`serde_json` for serialization
- Use `reqwest` + `tokio` for async adapter calls
- Use `tracing` + `tracing-subscriber` for structured logging
- Store run artifacts under XDG data dir (fallback `~/.evidenced/`)
- Support config file loading from `yaml` and environment overrides

## Testing

- Unit tests for validation rules and provenance manifest generation
- Integration tests for CLI commands and checkpoint resume behavior
- Contract tests for source adapter response mapping
- Property tests for deterministic artifact output
