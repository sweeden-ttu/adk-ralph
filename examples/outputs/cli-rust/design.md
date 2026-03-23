# System Design: evidenced

## Architecture Overview

`evidenced` uses a pipeline-oriented CLI architecture with deterministic processing and artifact output for verification traceability.

```mermaid
flowchart TB
    subgraph cliLayer [CLI_Layer]
        clapParser[ClapParser]
        commandHandlers[CommandHandlers]
    end

    subgraph orchestration [Orchestration]
        runCoordinator[RunCoordinator]
        checkpointManager[CheckpointManager]
        policyEngine[PolicyEngine]
    end

    subgraph validation [Validation]
        claimValidator[ClaimValidator]
        adapterRegistry[AdapterRegistry]
        decisionEngine[DecisionEngine]
    end

    subgraph artifacts [Artifacts]
        evidenceWriter[EvidenceWriter]
        manifestSigner[ManifestSigner]
        reportExporter[ReportExporter]
    end

    subgraph storage [Storage]
        artifactStore[ArtifactStore]
        cacheStore[CacheStore]
    end

    clapParser --> commandHandlers
    commandHandlers --> runCoordinator
    runCoordinator --> claimValidator
    runCoordinator --> checkpointManager
    claimValidator --> adapterRegistry
    adapterRegistry --> decisionEngine
    decisionEngine --> policyEngine
    policyEngine --> evidenceWriter
    evidenceWriter --> manifestSigner
    manifestSigner --> artifactStore
    reportExporter --> artifactStore
    adapterRegistry --> cacheStore
```

## Component Diagram

```mermaid
classDiagram
    class Claim {
        +string claim_id
        +string claim_type
        +string jurisdiction
        +string text
        +map metadata
    }

    class DecisionResult {
        +string claim_id
        +string decision
        +float confidence
        +string reason_code
        +[]EvidenceRef evidence
    }

    class EvidenceRef {
        +string source_url
        +string dataset_id
        +string record_id
        +string retrieved_at
        +string evidence_hash
    }

    class RunCoordinator {
        +run(inputPath) RunSummary
        +resume(runId) RunSummary
    }

    class Adapter {
        <<interface>>
        +fetch(query) SourceRecords
    }

    class ManifestSigner {
        +sign(manifest) SignedManifest
    }

    RunCoordinator --> Claim
    RunCoordinator --> DecisionResult
    DecisionResult --> EvidenceRef
    RunCoordinator --> Adapter
    RunCoordinator --> ManifestSigner
```

## File Structure

```
evidenced/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── cli.rs
│   ├── config.rs
│   ├── orchestrator/
│   │   ├── mod.rs
│   │   ├── run.rs
│   │   └── checkpoint.rs
│   ├── validation/
│   │   ├── mod.rs
│   │   ├── claim.rs
│   │   ├── decision.rs
│   │   └── policy.rs
│   ├── adapters/
│   │   ├── mod.rs
│   │   ├── texas_open_data.rs
│   │   └── texas_capitol_data.rs
│   ├── artifacts/
│   │   ├── evidence.rs
│   │   ├── manifest.rs
│   │   └── report.rs
│   └── storage/
│       ├── artifact_store.rs
│       └── cache_store.rs
└── tests/
    ├── cli_integration.rs
    ├── policy_tests.rs
    └── adapter_contracts.rs
```

## Technology Stack

- **Language**: Rust (stable)
- **CLI**: `clap`
- **Async HTTP**: `reqwest` + `tokio`
- **Serialization**: `serde`, `serde_json`
- **Logging**: `tracing`
- **Config**: `serde_yaml` + env overrides

## Error Handling

- Strongly typed error categories (`InputError`, `AdapterError`, `DecisionError`, `ArtifactError`)
- Partial-failure tracking for continue-on-error runs
- Structured stderr diagnostics with correlation IDs

## Testing Strategy

- Unit tests for decision/policy logic
- Integration tests for command flows (`run`, `resume`, `report`)
- Contract tests for adapter mappings
- Property tests for deterministic artifact generation
