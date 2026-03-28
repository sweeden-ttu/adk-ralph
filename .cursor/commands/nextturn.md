# nextturn

Continue the Ralph multi-agent pipeline for the active project. Treat **`design.md`** and **`tasks.json`** as the source of truth for what to build next; align implementation with the design and task breakdown.

## 1. Locate the project

- Prefer `RALPH_PROJECT_PATH` (or the path the user indicates). Otherwise use the workspace root or the directory that contains `tasks.json` / `design.md`.
- Typical layout: each generated app lives under the configured project folder (often a subfolder per project with `prd.md`, `design.md`, `tasks.json`, `progress.json`).

## 2. Read state (before acting)

Open and skim:

- `design.md` — architecture, components, constraints
- `tasks.json` — pending / in-progress tasks, priorities, dependencies
- `progress.json` if present — prior learnings and blockers
- `prd.md` if you need requirements context or design/tasks are missing

## 3. Run the right Ralph phase (terminal)

From the **adk-ralph** repo root (with `.env` loaded), use the `ralph` binary so each agent does its real work:

| Situation | Command |
|-----------|---------|
| No `prd.md` yet; user has a product idea | `ralph run "<prompt>"` or `ralph resume --phase requirements "<prompt>"` |
| `prd.md` exists but not `design.md` / `tasks.json` | `ralph resume --phase design ""` |
| `design.md` + `tasks.json` exist; continue implementation | `ralph resume --phase implementation ""` |

Use the global flag `-p` / `--project-path <dir>` when the generated project is not the default `RALPH_PROJECT_PATH`, or set `RALPH_PROJECT_PATH` before running.

## 4. What “next turn” means for each agent

- **PRD Agent** — Only if requirements are missing or the user explicitly wants them refreshed; output must feed later phases.
- **Architect Agent** — Regenerate or refine `design.md` and `tasks.json` from `prd.md` when design/tasks are absent or stale.
- **Ralph Loop Agent** — Pick the next actionable tasks from `tasks.json`, implement against `design.md`, update task status and `progress.json`, run tests before commits (per Ralph’s normal rules).

## 5. After commands run

Summarize what changed (files touched, tasks completed or started, blockers). If the CLI errored, show the error and suggest env fixes (API keys, `RALPH_*` / `OLLAMA_*`).

This command is invoked as **/nextturn** in chat.
