---
type: system
status: active
tags: [system, knowledge-graphs, kgx]
---

# kgx Knowledge Graph

Project metadata and relationships are stored in a kgx graph
at `.kgx/` in the vault root.

## How It Works

1. `devobs_ingest.crux` parses YAML frontmatter from each
   `02_Projects/PROJECT.*.md` file
2. Nodes are created per project; edges from `project_relationships`
3. The graph is queryable via `kgx search`, `kgx bfs`, and
   `kgx wiki`

## Ingest Payload

The ingest payload schema is defined at
`schemas/kgx-ingest.schema.json`. Each node carries: name, status,
language_stack, domain, primary_machine, and tags.

## Relationship Types

17 edge types defined in `schemas/project.schema.json`:
depends_on, bootstraps, observes, consumes_from, feeds_into,
adjacent_to, sibling_of, replaces, replaced_by, complements,
informed_by, informs, indexes, indexed_by, aligns_schema_with,
control_plane_for, provisioned_by.
