# Citation and Official Record Verification API - Python

Create a FastAPI service in Python called `verdict-api`.

## Purpose

A verification API that receives legal/government claims and citations, validates them against authoritative datasets (Texas Open Data, Texas Capitol Data, and configured official sources), and returns pass/fail decisions with provenance.

## Features

- **Verification Endpoints**
  - `POST /verify/citation` for legal citation checks
  - `POST /verify/official` for elected official and term validation
  - `POST /verify/election` for race, candidate, and result checks
  - `POST /verify/document` for court/legal document metadata verification
  - Batch verification endpoint for bulk claim submissions

- **Decisioning and Provenance**
  - Standardized decision states: `verified`, `unverified`, `conflicting`, `insufficient_evidence`
  - Return supporting source records, matched fields, and confidence scores
  - Include provenance metadata (source URL, dataset ID, retrieval time, evidence hash)
  - Explainable failure reasons with remediation hints

- **Security and Governance**
  - API key authentication and role-based scopes
  - Request schema enforcement and input sanitization
  - Audit log for every decision request
  - Signed response option for downstream trust checks

- **Operations and Observability**
  - Health and readiness endpoints
  - OpenTelemetry traces and structured logs
  - Configurable rate limits per client
  - Async job queue support for large batch runs

## Technical Requirements

- FastAPI + Pydantic v2
- SQLAlchemy for audit/provenance persistence
- Redis for cache and background job coordination
- HTTP client layer with retries/backoff for external datasets
- OpenAPI 3.1 documentation with reusable schemas
- Container-friendly configuration via environment variables

## API Design

- Versioned routes (`/v1/...`)
- RFC 7807 problem detail errors
- Idempotency keys for batch submissions
- Cursor-based pagination for decision history

## Testing

- `pytest` for unit and integration tests
- Async endpoint tests with `httpx`
- Contract tests for source-adapter responses
- Security tests for auth, input validation, and rate-limit behavior
