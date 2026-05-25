---
slug: "kgx-knowledge-graph-system"
category: "summary"
---

# Summary

---

title: kgx Knowledge Graph System
source_document: system_kgx_knowledge_graph
tags: [summary, system, kgx, knowledge-graphs]

---

# kgx Knowledge Graph System

Project metadata and relationships stored in `.kgx/` at vault root.

## How It Works

1. `devobs_ingest.crux` parses YAML frontmatter from PROJECT notes
2. Creates nodes per project, edges from `project_relationships`
3. Queryable via `kgx search`, `kgx bfs`, `kgx wiki`

## Schema

- 17 relationship edge types defined in `schemas/project.schema.json`
- Ingest payload format: `schemas/kgx-ingest.schema.json`

## Source

- `03_Area-Systems/SYSTEM.kgx-knowledge-graph.md`
