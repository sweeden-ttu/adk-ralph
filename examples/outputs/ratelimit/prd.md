# Product Requirements Document

## Project Overview

**Project Name**: trustscore
**Language**: Go
**Type**: Library

`trustscore` is a reusable Go library for computing source trust scores, detecting spoofing indicators, and producing policy decisions (`allow`, `review`, `block`) for legal/government content validation pipelines.

## Glossary

- **Trust Score**: Weighted numeric reliability score for a source-evidence pair
- **Spoof Signal**: Heuristic indication of source impersonation or manipulation risk
- **Decision Policy**: Threshold-based mapping of score and signals to operational decisions
- **Reason Code**: Stable machine-readable explanation for why a decision was made

## User Stories

### US-001: Score Source Reliability

**Priority**: 1  
**Status**: pending

**Description**: As a validator developer, I want deterministic source trust scores, so that I can apply consistent verification decisions.

**Acceptance Criteria**:
1. WHEN a caller submits a `ScoreRequest`, THE library SHALL return a numeric score and factor breakdown
2. THE library SHALL support configurable weighting profiles by content type
3. THE library SHALL emit deterministic results for identical inputs and config

### US-002: Detect Spoofing Risk

**Priority**: 1  
**Status**: pending

**Description**: As a security engineer, I want spoof detection heuristics, so that suspicious sources are flagged before content is trusted.

**Acceptance Criteria**:
1. THE library SHALL detect normalized-domain mismatches and lookalike patterns
2. THE library SHALL evaluate URL/path anomalies and confidence penalties
3. THE library SHALL expose spoof reason codes in decision output

### US-003: Policy-Based Decisioning

**Priority**: 1  
**Status**: pending

**Description**: As a platform service, I want a decision engine, so that score and spoof signals map to clear operational actions.

**Acceptance Criteria**:
1. WHEN policy thresholds are applied, THE library SHALL return `allow`, `review`, or `block`
2. THE library SHALL include explanation details with every decision
3. THE library SHALL support policy overrides for analyst-approved exceptions

### US-004: Batch Throughput

**Priority**: 2  
**Status**: pending

**Description**: As an ingestion pipeline, I want batch evaluation support, so that high-volume claims can be processed efficiently.

**Acceptance Criteria**:
1. THE library SHALL provide a batch decision API
2. THE library SHALL be safe for concurrent use
3. THE library SHALL expose per-item decision details in batch responses

### US-005: Observability Integration

**Priority**: 3  
**Status**: pending

**Description**: As an operator, I want score and decision telemetry, so that quality and drift can be monitored.

**Acceptance Criteria**:
1. THE library SHALL expose counters for decision classes
2. THE library SHALL provide optional hooks for trace/metrics emission
3. THE library SHALL allow callers to attach correlation IDs

