---
type: system
status: active
tags: [system, automation, crux]
---

# Crux Pipelines

Automated pipelines for vault maintenance, powered by the crux DSL.

## Pipeline Inventory

| Pipeline | File                           | Purpose                              |
| -------- | ------------------------------ | ------------------------------------ |
| Ingest   | `pipelines/devobs_ingest.crux` | Parse PROJECT notes into kgx graph   |
| Lint     | `pipelines/devobs_lint.crux`   | Schema validation, obfsck, agentlint |
| Enrich   | `pipelines/devobs_enrich.crux` | LLM-assisted relationship discovery  |

## Orchestration

The `Cruxfile` defines a target graph:

```
ci (default) ─┬─ lint
              └─ ingest
enrich ────────── ingest
```

`crux run Cruxfile` runs `ci` (lint + ingest in parallel).
`enrich` is invoked separately and requires `ingest` to complete first.

## Dependencies

- **crux** CLI (`~/dev/crux`)
- **kgx** CLI (`~/dev/kgx`) for graph storage
- **obfsck** for secret scanning during lint
- **agentlint** for CLAUDE.md quality gates
