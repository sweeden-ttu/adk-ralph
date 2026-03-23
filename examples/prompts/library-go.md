# Source Trust and Anti-Spoof Library - Go

Create a Go library called `trustscore` for source reliability scoring and spoof detection in legal/government validation pipelines.

## Purpose

A reusable library that computes trust scores for sources and evidence records, detects source spoofing signals, and produces policy-based allow/review/block decisions.

## Features

- **Trust Scoring**
  - Weighted scoring based on domain reputation, authority type, freshness, and citation consistency
  - Configurable scoring profiles by content type (news, officials, elections, court docs, templates)
  - Confidence penalties for contradictory evidence
  - Deterministic score explanations for auditability

- **Anti-Spoof Detection**
  - Domain normalization and lookalike detection
  - TLS/certificate metadata checks (when provided by caller)
  - URL/path anomaly heuristics for phishing-like patterns
  - Cross-source mismatch checks (same claim, conflicting authority records)

- **Decision Engine**
  - Policy thresholds for `allow`, `review`, and `block`
  - Reason codes and human-readable explanations
  - Override hooks for analyst-approved exceptions
  - Support for both single-claim and batch evaluation

- **Observability**
  - Export scoring telemetry and decision counters
  - Trace-friendly structured events
  - Optional Prometheus metrics adapter

## Technical Requirements

- Go 1.21+
- Context-aware public APIs
- Thread-safe internals
- Minimal dependencies in core package
- Optional adapters for metrics and distributed caches

## API Design

```go
type Evaluator interface {
    Score(ctx context.Context, req ScoreRequest) (ScoreResult, error)
    Decide(ctx context.Context, req DecisionRequest) (DecisionResult, error)
    BatchDecide(ctx context.Context, reqs []DecisionRequest) ([]DecisionResult, error)
}

type Decision string

const (
    DecisionAllow  Decision = "allow"
    DecisionReview Decision = "review"
    DecisionBlock  Decision = "block"
)
```

## Testing

- Table-driven tests for scoring and policy behavior
- Fuzz tests for URL normalization and spoof heuristics
- Race tests (`go test -race`) for concurrent batch scoring
- Benchmark suite for single vs. batch throughput
