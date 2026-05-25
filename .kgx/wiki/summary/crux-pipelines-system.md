---
title: Crux Pipelines System
source_document: system_crux_pipelines
tags: [summary, system, crux, automation]
---

# Crux Pipelines System

Automated pipelines for devobs vault maintenance.

## Pipelines

- **Ingest** — parse PROJECT note frontmatter into [[kgx]] graph
- **Lint** — schema validation + [[obfsck]] + [[agentlint]]
- **Enrich** — LLM-assisted relationship discovery

## Orchestration

Cruxfile target graph: `ci = lint + ingest` (default), `enrich -> ingest`.

## Dependencies

- [[crux]] DSL runtime
- [[kgx]] knowledge graph
- [[obfsck]] secret scanning
- [[agentlint]] quality gates

## Source

- `03_Area-Systems/SYSTEM.crux-pipelines.md`
