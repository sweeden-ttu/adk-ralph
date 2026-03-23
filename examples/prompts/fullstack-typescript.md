# Verification Review Workbench - TypeScript

Create a full-stack TypeScript application called `review-workbench`.

## Purpose

A human-in-the-loop review interface where analysts inspect validator decisions, compare source evidence, resolve conflicts, and approve content for downstream retrieval.

## Features

- **Case Queue and Triage**
  - Prioritized queue of verification cases (`verified`, `conflicting`, `unverified`)
  - Filters by content type, jurisdiction, risk score, and source family
  - Assignment and ownership workflow for analysts
  - SLA timers and escalation flags

- **Evidence Review**
  - Side-by-side claim vs. source evidence comparison
  - Provenance timeline showing fetch time, source IDs, and decision history
  - Diff view for conflicting records
  - Inline notes and adjudication comments

- **Decision Workflow**
  - Approve/reject/review-needed outcomes with reason codes
  - Policy override with mandatory justification and audit trail
  - Bulk actions for low-risk cases
  - Re-run validation from UI for updated sources

- **Reporting and Audit**
  - Dashboard for hallucination rate, conflict rate, and analyst throughput
  - Exportable audit logs and decision summaries
  - Alert center for source outages or anomaly spikes

## Technical Requirements

### Backend
- Node.js + Express (TypeScript)
- PostgreSQL + Prisma
- Redis for queue state and caching
- WebSocket channel for live queue updates

### Frontend
- React 18 + TypeScript
- TanStack Query for server state
- Zustand for local UI state
- Tailwind CSS for styling

### Infrastructure
- Docker Compose for local development
- Environment-based configuration
- OpenTelemetry instrumentation

## Testing

- Vitest for unit tests
- Playwright for analyst workflow E2E tests
- Contract tests for API schema stability
