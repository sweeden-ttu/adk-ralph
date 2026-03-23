# Validation Routing Microservice - Java

Create a Spring Boot microservice called `validator-router`.

## Purpose

An event-driven routing service that receives extracted claims, invokes specialized validators, enforces pass/fail/retry policy, and emits workflow events for downstream indexing and human review.

## Features

- **Claim Intake**
  - Ingest claim events from queue/topic
  - Validate event schema and deduplicate by idempotency key
  - Classify claim type (citation, official, election, law, document, template)

- **Routing and Policy**
  - Route claims to validator adapters based on type and jurisdiction
  - Policy states: `PASS`, `FAIL`, `RETRY`, `ESCALATE`
  - Configurable retry budget, backoff, and dead-letter handling
  - Escalate low-confidence or conflicting outcomes to analyst queue

- **Event Publishing**
  - Publish lifecycle events: `ClaimReceived`, `ValidationPassed`, `ValidationFailed`, `EscalationCreated`
  - Ensure idempotent publish semantics
  - Support replay mode for incident recovery

- **Operations**
  - Administrative endpoints for replay and policy reload
  - Health/readiness checks and queue lag metrics
  - Correlation IDs across inbound/outbound events

## Technical Requirements

- Spring Boot 3.x with Java 21
- Spring Kafka for intake/publish workflows
- Spring Data JPA + PostgreSQL for run metadata
- Spring Retry for retry orchestration
- Flyway for migrations
- OpenAPI docs for admin/control APIs

## API Design

- REST admin endpoints for replay, status, and policy inspection
- RFC 7807 problem details for errors
- Bean Validation for request payloads

## Observability

- Spring Actuator health and metrics
- Micrometer dashboards for routing outcomes
- Distributed tracing with correlation IDs

## Testing

- JUnit 5 unit tests for routing and policy engine
- Testcontainers integration tests (Kafka + PostgreSQL)
- Contract tests for validator adapter interfaces
