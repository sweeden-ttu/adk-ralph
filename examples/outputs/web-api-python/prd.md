# Product Requirements Document

## Project Overview

**Project Name**: verdict-api
**Language**: Python
**Type**: REST API

`verdict-api` is the verification service for legal/government claims in the CS5374 Trustworthy AI project. It validates citations, officials, election claims, and document metadata against authoritative datasets and returns explainable decisions with provenance.

## Glossary

- **Verification Request**: Structured payload containing a claim and context metadata
- **Decision State**: `verified`, `unverified`, `conflicting`, `insufficient_evidence`
- **Provenance Record**: Source dataset IDs, URLs, timestamps, and evidence hashes used in a decision
- **Adapter**: Component that maps authoritative source responses into a canonical schema

## User Stories

### US-001: Verify Citations

**Priority**: 1  
**Status**: pending

**Description**: As a researcher, I want to submit legal citations for verification, so that I can avoid surfacing hallucinated references.

**Acceptance Criteria**:
1. WHEN a client sends `POST /v1/verify/citation`, THE system SHALL return a decision state with reason codes
2. THE system SHALL include matched source records when available
3. THE system SHALL return `insufficient_evidence` when authoritative records are unavailable

### US-002: Verify Officials and Elections

**Priority**: 1  
**Status**: pending

**Description**: As an analyst, I want to verify official names, offices, terms, and election outcomes, so that generated outputs remain grounded in authoritative data.

**Acceptance Criteria**:
1. WHEN a client sends `POST /v1/verify/official`, THE system SHALL validate name/office/term claims
2. WHEN a client sends `POST /v1/verify/election`, THE system SHALL validate race/candidate/result claims
3. THE system SHALL include source dataset identifiers in responses

### US-003: Batch Verification

**Priority**: 2  
**Status**: pending

**Description**: As a pipeline operator, I want to submit batches of claims, so that high-volume validation jobs can run efficiently.

**Acceptance Criteria**:
1. WHEN a client sends `POST /v1/verify/batch`, THE system SHALL accept and process multiple claims
2. THE system SHALL provide per-item outcomes and aggregate summary stats
3. THE system SHALL support idempotency keys for safe retries

### US-004: Secure Access and Auditability

**Priority**: 1  
**Status**: pending

**Description**: As a platform owner, I want authenticated and auditable verification APIs, so that access is controlled and decisions are traceable.

**Acceptance Criteria**:
1. THE system SHALL require API keys and scope checks for protected endpoints
2. THE system SHALL log immutable audit records for every request
3. THE system SHALL sanitize and validate all payloads before adapter execution

### US-005: Provenance and Decision History

**Priority**: 2  
**Status**: pending

**Description**: As an auditor, I want decision history and provenance export, so that model outputs can be reviewed and reproduced.

**Acceptance Criteria**:
1. WHEN a client requests decision history, THE system SHALL return paginated records
2. THE system SHALL expose evidence metadata for each decision
3. THE system SHALL support JSON export for downstream reporting
