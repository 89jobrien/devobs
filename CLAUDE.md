# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working
with code in this repository.

## What This Is

Public Obsidian vault for open-source project tracking. Only references
public repos under github.com/89jobrien that have local clones in ~/dev/.

## Structure

| Folder        | Purpose                                       |
| ------------- | --------------------------------------------- |
| `02_Projects` | One PROJECT note per public repo              |
| `schemas`     | JSON schemas for frontmatter and kgx payloads |
| `pipelines`   | Crux pipeline definitions                     |
| `.kgx/`       | kgx knowledge graph (generated)               |

## Commands

```bash
crux run pipelines/devobs_ingest.crux    # Ingest projects into kgx
crux run pipelines/devobs_lint.crux      # Quality gates
crux run pipelines/devobs_enrich.crux    # LLM relationship discovery
crux run Cruxfile                        # Full CI (lint + ingest)
```

### Target Dependencies (Cruxfile)

```
ci ─┬─ lint
    └─ ingest
enrich ── ingest
```

`crux run Cruxfile` runs the default `ci` target (lint + ingest in
parallel). `enrich` is separate and depends on `ingest` completing first.

## Godmode Workflow

Session context lives in `.ctx/`:

| File                   | Purpose                 |
| ---------------------- | ----------------------- |
| `GODMODE.session.json` | Current session state   |
| `GODMODE.trace.jsonl`  | Session trace log       |
| `GODMODE.tasks.yaml`   | Task graph (if present) |

Use `godmode task` CLI for task state transitions. Independent tasks can
run in parallel via `godmode:parallel-agents`.

## Conventions

### PROJECT Notes

- One file per public repo: `02_Projects/PROJECT.<name>.md`
- YAML frontmatter validated against `schemas/project.schema.json`
- Required fields: type, status, language_stack, domain, repo,
  primary_machine, tags
- `repo.url` must be `https://github.com/89jobrien/<name>.git`
- Status: `active` (committed within 30 days), `inactive`, `archived`

### Relationship Types

Defined in `schemas/project.schema.json` under
`project_relationships.type`: depends_on, bootstraps, observes,
consumes_from, feeds_into, adjacent_to, sibling_of, replaces,
replaced_by, complements, informed_by, informs, indexes, indexed_by,
aligns_schema_with, control_plane_for, provisioned_by.

### Scope Rules

- No private repos, no employer projects, no forks without local work
- No secrets, credentials, or internal IPs in any file
- All content must be safe for public GitHub hosting
