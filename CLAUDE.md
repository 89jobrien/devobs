# CLAUDE.md

## What This Is

Public Obsidian vault for open-source project tracking. Only references
public repos under github.com/89jobrien that have local clones in ~/dev/.

## Automation

- **Crux pipelines** in `pipelines/` handle ingest, lint, and enrichment
- **kgx** stores the knowledge graph in `.kgx/`
- **JSON schemas** in `schemas/` define the frontmatter and ingest contracts

## Commands

```bash
crux run pipelines/devobs_ingest.crux    # Ingest projects into kgx
crux run pipelines/devobs_lint.crux      # Quality gates
crux run pipelines/devobs_enrich.crux    # LLM relationship discovery
crux run Cruxfile                        # Full CI (lint + ingest)
```

## Conventions

### PROJECT Notes

- One file per public repo: `02_Projects/PROJECT.<name>.md`
- YAML frontmatter validated against `schemas/project.schema.json`
- Required fields: type, status, language_stack, domain, repo, primary_machine, tags
- `repo.url` must be `https://github.com/89jobrien/<name>.git`
- Status: `active` (committed within 30 days), `inactive`, `archived`

### Relationship Types

Defined in `schemas/project.schema.json` under `project_relationships.type`:
depends_on, bootstraps, observes, consumes_from, feeds_into, adjacent_to,
sibling_of, replaces, replaced_by, complements, informed_by, informs,
indexes, indexed_by, aligns_schema_with, control_plane_for, provisioned_by.

### Scope Rules

- No private repos, no employer projects, no forks without local work
- No secrets, credentials, or internal IPs in any file
- All content must be safe for public GitHub hosting
