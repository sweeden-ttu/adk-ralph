# Farm property and business research pipeline — Rust (ADK-Rust)

Create a Rust binary crate **research-pipeline** using **adk-graph**, **adk-agent** (`LlmAgentBuilder`), **adk-model** (`GeminiModel`, default `gemini-2.5-flash`), and **adk-tool** (`GoogleSearchTool`) for grounded web search.

## Purpose

Scheduled and on-demand **farm and rural land listing research** in three U.S. regions, followed by **business viability analysis** for a plant-focused nursery/greenhouse operation (saplings, landscape plants, vegetables, plant organics). **Exclude** livestock, poultry, and non-plant farm products.

The system must include a **database / memory node** and a **downstream publishing pipeline** (GitHub Pages + Jekyll and a presentation expert) that turns research into a **portfolio** suitable for stakeholders.

## Regions (one specialized property-search agent each)

1. **central_tx** — Corridor between Killeen and Austin, Texas. Instructions must emphasize: Central Texas MLS portals, Austin–Temple–Killeen land brokers, `land.com`, Lands of Texas–style aggregators, county tax/foreclosure notices, regional auction houses, Texas A&M AgriLife extension land resources, and major franchise plus independent land realtor names common to the I-35 corridor.
2. **lubbock_tx** — Lubbock and South Plains. Emphasize: West Texas land specialists, Plains MLS and regional land sites, farm auctions, USDA/FSA county office references, irrigation/water district context, and Lubbock-area ag brokers.
3. **speedwell_tn** — Speedwell and surrounding Claiborne / East Tennessee markets. Emphasize: East Tennessee land and farm listings, regional MLS and land portals, county chancery / auction sources, Knoxville-area market spillover, and local nursery/garden center ecosystems.

Each property agent uses **only** `google_search` (no scraping); prompts steer queries toward the above sources.

## Graph workflow (core research)

Linear pipeline through research and persistence:

`START → property_search → business_viability → synthesis → artifact_writer → memory_store → END`

- **property_search**: Region-specific `LlmAgent` + `GoogleSearchTool`. Find current or recent **farm, ranch, acreage, or rural land** listings for sale (or auction) in the selected region. Output structured narrative plus URLs (listings, auctions, broker pages).
- **business_viability**: Shared `LlmAgent` + `GoogleSearchTool`. Given property findings, research **nearby nurseries, garden centers, greenhouses, wholesale growers** (plants only). Assess competitor density, differentiation gaps, drive-time to population centers and farmers markets, and demand signals for saplings, landscape, vegetables, and organic **plant** products. Deliver qualitative **low | medium | high** viability and explicit **risks** and **sources**.
- **synthesis**: `LlmAgent` **without** search tools. Consumes property + business text; produces (1) a single JSON object matching project JSON Schemas (property run summary + listings array + viability object) and (2) a short executive markdown summary. Instruct the model to wrap JSON in a fenced `json` block and markdown in a fenced `markdown` block.
- **artifact_writer**: `FunctionNode` that writes under `runs/<region_slug>/<ISO-date>/`: `property_candidates.json`, `business_viability.json`, `summary.md`, and `manifest.json`. Create directories as needed. If JSON parsing fails, write raw model output to `synthesis_raw.txt` and still write `summary.md`.
- **memory_store** (database node): A `FunctionNode` or dedicated service boundary that **persists**:
  - **Graph / run state**: region, run_id, timestamps, paths to artifacts, manifest JSON, key state channel snapshots needed for resume or audit.
  - **Long-term memory**: structured records of listings seen, deduplication keys, viability conclusions, and operator notes or tags.
  - **Embeddings**: vector representations of listing summaries, business analysis chunks, and synthesis text for **semantic retrieval** (use an embedding API compatible with your stack, e.g. Gemini embeddings or a local model; store vectors in SQLite with `sqlite-vec` / `vec0`, or LanceDB, or pgvector—choose one and document it).
  - **Context packs**: rolling context for downstream agents (e.g. top-k retrieved chunks + citations for each property).

The database node must be **queryable** by later stages (SQL + vector search) so the Jekyll and presentation agents do not rely only on ephemeral files.

## Downstream: GitHub Jekyll Pages agent + presentation expert

After each run (or on a schedule), the **final pipeline** must **feed** two cooperating agents (implemented as separate binaries, library modules, or graph branches—document the choice):

1. **GitHub Jekyll Pages agent**  
   - Consumes: all **artifacts** from `runs/`, **database rows** (listings, viability, embeddings metadata), **learnings** (e.g. progress log, dedup decisions, search queries that worked), and **retrieved context** from the vector store.  
   - Produces: a **Jekyll** site repository layout (`_config.yml`, `_layouts/`, `_includes/`, `assets/`, collections or posts per property) ready for **GitHub Pages**.  
   - Each property page should link structured data (front matter) to the portfolio sections below.

2. **Presentation expert**  
   - Same inputs as the Jekyll agent plus style/tone guidelines.  
   - Produces **elegant, cohesive narrative and layout content** for a **portfolio** of farm properties. For **each** property (or top-N listings), generate:
     - **Farm business proposal** (plant-focused operation, market, differentiation, phased rollout—no livestock).
     - **Biography** (narrative “story” of the land, location, and opportunity—clearly labeled as research synthesis, not legal fact).
     - **Sales contract draft with price analysis** (template-style contract sections plus **comparable-driven price commentary** and assumptions; include disclaimers that a licensed attorney must review any real contract).

The Jekyll agent and presentation expert should share a **single source of truth** from the database and embeddings so the site stays consistent with stored research.

## CLI

Use **clap** with derived args:

- `--region central_tx | lubbock_tx | speedwell_tn` (required for research run)
- `--output-root <path>` default `./runs`
- `--dry-run` print planned run directory and exit without calling the API
- Optional: `--skip-publish` to run research + DB only without invoking Jekyll/presentation pipeline

Require `GOOGLE_API_KEY` or `GEMINI_API_KEY`; load `.env` via `dotenvy`.

## JSON Schemas

Ship **JSON Schema** files under `research-pipeline/schemas/`:

- `property_candidates.schema.json` — region, timestamp, listings (title, location, price/auction text, acreage hint, URLs, notes, queries used), and `sources[]`.
- `business_viability.schema.json` — region, property context, competitors, markets, demand breakdown (saplings, landscape, vegetables, organics_plants), assessment text, qualitative score, risks[], sources[].

Add schemas or tables (document in design) for **memory records**, **embedding metadata**, and **portfolio page front matter** as needed.

Load schema excerpts or full text into the **synthesis** system instruction so the model knows the exact shape.

## Scheduling

Document in **README.md**: example **cron** line and **systemd timer** snippet for research runs per region, and optionally a second timer for **site build + git push** to GitHub Pages. Note extension points: **farm-site suitability**, **demographics / growth**, **water/soil**.

## Testing

- `cargo test` for pure helpers (JSON fence extraction, path slugging, DB migrations smoke test).
- `cargo clippy` clean for the crate.

## Non-goals

- No livestock or animal-agriculture research.
- No automated HTML scraping beyond what Gemini `google_search` returns.
- Generated **contracts** are **templates for review only**—not legal advice.
