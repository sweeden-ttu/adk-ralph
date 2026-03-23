# Product Requirements Document

## Project Overview

**Project Name**: evidenced
**Language**: Rust
**Type**: CLI Application

`evidenced` is a command-line orchestration tool for the Trustworthy AI legal/government validator project. It runs verification jobs against authoritative sources, emits pass/fail decisions, and produces provenance artifacts for downstream indexing and audit workflows.

## Glossary

- **Claim**: Structured statement requiring verification (citation, official, election, law, document, or template)
- **Evidence Bundle**: Matched source records and metadata supporting a decision
- **Decision**: Verification outcome (`pass`, `fail`, `conflicting`, `insufficient_evidence`)
- **Provenance Manifest**: Signed JSON metadata for run traceability
- **Adapter**: Source-specific connector for authoritative datasets

## User Stories

### US-001: Run Validation Jobs

**Priority**: 1
**Status**: pending

**Description**: As a validation engineer, I want to execute verification runs from claim files, so that I can produce trustworthy decisions at scale.

**Acceptance Criteria**:
1. WHEN a user runs `evidenced run --input claims.ndjson`, THE system SHALL validate each claim and emit a decision
2. WHEN input schema is invalid, THE system SHALL reject the run with actionable errors
3. THE system SHALL support claim types for legal/government validation
4. THE system SHALL include machine-readable reason codes per decision

### US-002: Generate Provenance Artifacts

**Priority**: 1
**Status**: pending

**Description**: As an auditor, I want signed provenance output for every run, so that decisions can be traced and reproduced.

**Acceptance Criteria**:
1. THE system SHALL write run metadata and item-level evidence to JSON artifacts
2. THE system SHALL include source URLs, dataset identifiers, retrieval timestamps, and evidence hashes
3. THE system SHALL generate a signed provenance manifest per run
4. THE system SHALL include tool version and configuration fingerprint

### US-003: Resume Interrupted Runs

**Priority**: 2
**Status**: pending

**Description**: As an operator, I want checkpoint and resume support, so that long runs recover safely after failures.

**Acceptance Criteria**:
1. WHEN a run is interrupted, THE system SHALL persist a checkpoint
2. WHEN a user runs `evidenced resume <run-id>`, THE system SHALL continue remaining claims only
3. THE system SHALL preserve deterministic output ordering after resume

### US-004: Adapter and Cache Controls

**Priority**: 2
**Status**: pending

**Description**: As a platform engineer, I want configurable adapters and cache policies, so that source verification stays fast and reliable.

**Acceptance Criteria**:
1. THE system SHALL load adapter and cache settings from config files
2. THE system SHALL cache adapter responses with TTL and deterministic keys
3. THE system SHALL support offline replay from source snapshots

### US-005: Export and Reporting

**Priority**: 3
**Status**: pending

**Description**: As a project stakeholder, I want decision summaries in common formats, so that experiment and review pipelines can consume run outputs.

**Acceptance Criteria**:
1. WHEN a user runs `evidenced report --format json|csv`, THE system SHALL export decision summaries
2. THE system SHALL include counts by decision state and content type
3. THE system SHALL include run-level latency and error metrics
