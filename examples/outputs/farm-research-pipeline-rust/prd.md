# Product Requirements Document

## Project Overview

**Project Name**: research-pipeline  
**Language**: Rust  
**Type**: CLI + ADK-Rust multi-agent graph + persistence + static site generation  

A command-line pipeline that uses **Gemini** with **Google Search grounding** to discover farm and rural land listings in three geographic focus areas, researches **plant-only** agricultural retail competition, persists **state, long-term memory, embeddings, and retrievable context** in a database, writes **JSON/markdown artifacts**, and **feeds** a **GitHub Jekyll Pages** publishing agent and a **presentation expert** that produce an elegant **portfolio** (business proposal, biography, contract draft with price analysis per property).

## Glossary

- **Region**: One of `central_tx` (Killeen–Austin corridor), `lubbock_tx`, or `speedwell_tn`.
- **Listing**: A candidate property (farm, ranch, acreage, auction) with URLs and unstructured price or acreage hints from web search.
- **Viability score**: Qualitative rating `low`, `medium`, or `high` for operating a plant-focused nursery/greenhouse near candidate locations—not a financial model.
- **Competitor**: Nursery, garden center, greenhouse, or wholesale plant grower within relevant drive distance of the search area.
- **Artifact run**: A single execution that writes files under `runs/<region>/<YYYY-MM-DD>/`.
- **Memory store**: Database node persisting graph state snapshots, deduplicated listing history, embedding vectors, and retrieval context for downstream agents.
- **Portfolio**: Jekyll-based static site (GitHub Pages) presenting properties with proposals, biographies, and contract drafts.
- **Presentation expert**: Agent that authors narrative, layout copy, and structured sections for the portfolio from DB + artifacts + retrieved embeddings.

## User Stories

### US-001: Central Texas property search

**Priority**: 1  
**Status**: pending  

**Description**: As an operator, I want the **central_tx** agent to search the Killeen–Austin corridor using region-aware queries (I-35 land brokers, Land.com, Lands of Texas, county sources, auctions), so that I get current farm and acreage leads with citations.

**Acceptance Criteria**:
1. WHEN the user runs with `--region central_tx`, THE pipeline SHALL invoke a property agent trained for Central Texas sources.
2. THE property agent SHALL use the `google_search` tool for discovery.
3. THE output SHALL include listing URLs or explicit source URLs where possible.

### US-002: Lubbock property search

**Priority**: 1  
**Status**: pending  

**Description**: As an operator, I want the **lubbock_tx** agent to emphasize South Plains land specialists, regional MLS/land portals, and West Texas auction patterns.

**Acceptance Criteria**:
1. WHEN `--region lubbock_tx`, THE pipeline SHALL use the Lubbock-specific property instructions.
2. Search behavior SHALL exclude livestock-focused business ideas in downstream analysis.

### US-003: Speedwell TN property search

**Priority**: 1  
**Status**: pending  

**Description**: As an operator, I want the **speedwell_tn** agent to focus on East Tennessee / Claiborne County area listings and Knoxville spillover markets.

**Acceptance Criteria**:
1. WHEN `--region speedwell_tn`, THE pipeline SHALL use Speedwell / East Tennessee–specific instructions.
2. Results SHALL reference regional land listing patterns appropriate to Appalachia / East TN.

### US-004: Business viability analysis

**Priority**: 1  
**Status**: pending  

**Description**: As an operator, I want a **shared business agent** to research nearby plant-sector competitors and market access given property findings.

**Acceptance Criteria**:
1. THE business agent SHALL use `google_search`.
2. THE scope SHALL be limited to **plants** (saplings, landscape, vegetables, organic plant products)—not livestock.
3. THE output SHALL include competitor narrative, market access, demand signals per product category, qualitative viability, risks, and **sources**.

### US-005: Structured artifacts

**Priority**: 1  
**Status**: pending  

**Description**: As an operator, I want each run to write **JSON** aligned with shipped schemas plus **summary.md** and **manifest.json**.

**Acceptance Criteria**:
1. EVERY successful run SHALL create the run directory and write `manifest.json`.
2. WHEN synthesis returns parseable JSON, THE system SHALL write `property_candidates.json` and `business_viability.json`.
3. WHEN JSON parsing fails, THE system SHALL write `synthesis_raw.txt` with the model output and still write `summary.md`.

### US-006: CLI and dry run

**Priority**: 2  
**Status**: pending  

**Description**: As an operator, I want `--dry-run` to validate configuration without consuming API quota.

**Acceptance Criteria**:
1. WHEN `--dry-run` is set, THE binary SHALL NOT invoke the LLM graph.
2. THE binary SHALL print the resolved output directory and region.

### US-007: Scheduling documentation

**Priority**: 2  
**Status**: pending  

**Description**: As an operator, I want README examples for **cron** and **systemd** to run the binary on a schedule per region.

**Acceptance Criteria**:
1. THE README SHALL include at least one cron example and one systemd timer example.
2. THE README SHALL mention extension nodes (farm-site analysis, demographic growth).

### US-008: Database, embeddings, and long-term memory

**Priority**: 1  
**Status**: pending  

**Description**: As an operator, I want a **memory_store** step after artifacts that persists graph/run state, listing history, embeddings over research text, and retrievable context chunks so later agents and runs can deduplicate and reason over time.

**Acceptance Criteria**:
1. THE system SHALL persist **run metadata** (region, timestamps, artifact paths, manifest).
2. THE system SHALL persist **long-term memory** records (e.g. listings, viability summaries, operator tags) with stable IDs suitable for deduplication.
3. THE system SHALL store **embedding vectors** and metadata (model name, dimension, source chunk) and support **similarity search** for downstream agents.
4. THE system SHALL expose a documented **query API** (Rust functions or SQL views) used by the Jekyll and presentation pipeline.

### US-009: Jekyll GitHub Pages and presentation portfolio

**Priority**: 1  
**Status**: pending  

**Description**: As a stakeholder, I want a **GitHub Pages (Jekyll)** site that presents each property as a polished portfolio page including **farm business proposal**, **biography**, and **sales contract with price analysis**, generated from artifacts + database + retrieved embeddings and learnings.

**Acceptance Criteria**:
1. A **Jekyll Pages agent** SHALL read artifacts, DB rows, and vector-retrieved context and emit a valid Jekyll repo structure for GitHub Pages.
2. A **presentation expert** agent SHALL produce elegant copy and structure for each property: proposal, biography, and contract template with **price analysis** (comparables, assumptions, disclaimers).
3. THE site SHALL be **navigable** (index + per-property pages) and use coherent **design** (layout, typography, assets).
4. ALL legal/financial content SHALL include **disclaimers** (not legal or investment advice; attorney review required for contracts).

## Non-Functional Requirements

- Default model: **gemini-2.5-flash** for tool-heavy agents; document embedding model choice.
- API keys via `GOOGLE_API_KEY` or `GEMINI_API_KEY` and optional `.env`.
- Database: documented choice (e.g. SQLite + vector extension, or embedded alternative); migrations or schema versioned in repo.
- Publishing: document GitHub Actions or manual `git push` to `gh-pages` / Pages branch.

## Out of Scope

- Livestock, poultry, or animal product businesses.
- Automated HTML scraping beyond search grounding.
- Guaranteed legal enforceability of generated contracts (templates only).
