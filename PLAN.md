# devobs Knowledge Graph — Build Plan

## Status

| Layer          | State   | Numbers                                     |
| -------------- | ------- | ------------------------------------------- |
| PROJECT notes  | done    | 36 notes with validated frontmatter         |
| JSON schemas   | done    | project.schema.json, kgx-ingest.schema.json |
| Crux pipelines | partial | ingest works, lint/enrich untested          |
| kgx graph      | seeded  | 71 nodes, 97 edges (language + domain only) |
| kgx wiki       | empty   | no wiki pages yet                           |
| GitHub remote  | done    | 89jobrien/devobs                            |

## Phase 1: Cross-Project Relationships

**Goal:** Add `project_relationships` edges to PROJECT note frontmatter so
the graph captures how projects connect (depends_on, bootstraps, feeds_into,
etc.).

### Tasks

1. **Audit real dependencies** — for each of the 36 repos, check Cargo.toml
   / go.mod / pyproject.toml for cross-repo deps. Record as
   `project_relationships` in frontmatter.

   Known relationships (from code):
   - slashcrux depends_on crux
   - godmode depends_on crux (pipeline authoring)
   - atelier depends_on crux (pipeline authoring)
   - coursers depends_on obfsck (pre-commit scanning)
   - agentlint depends_on obfsck (pattern detection)
   - doob feeds_into hj (todo → handoff)
   - warpx feeds_into hj (handoff collapse)
   - valerie feeds_into hj (handoff reconciliation)
   - kgx bootstraps devobs (this vault)
   - praxis depends_on looprs (agent benchmarks)
   - braid depends_on looprs (orchestration)
   - langchainx depends_on looprs (agent framework)
   - bazaar bootstraps godmode, atelier (plugin marketplace)
   - prefixe depends_on coursers (hook resolution)

2. **Update ingest script** — extend `ingest_projects.py` to emit
   `project_relationships` as kgx relations.

3. **Re-run ingest** — `crux run pipelines/devobs_ingest.crux` to add
   relationship edges.

### Acceptance

- `kgx query "crux"` returns slashcrux, godmode, atelier as dependents
- `kgx query "hj"` returns doob, warpx, valerie as feeders
- Total edges > 120

## Phase 2: Wiki Layer

**Goal:** Populate kgx wiki with summary pages per project and synthesis
pages per domain cluster.

### Tasks

1. **Project summary pages** — one wiki page per project via
   `kgx wiki write --category summary`. Content: description, languages,
   domains, key relationships. Script: `scripts/write_wiki_summaries.py`.

2. **Domain cluster pages** — one wiki page per domain (agents, security,
   developer-tools, etc.) listing all projects in that domain and their
   inter-relationships. Script: `scripts/write_domain_pages.py`.

3. **Language ecosystem pages** — one wiki page per language summarizing
   all projects using it.

### Acceptance

- `kgx wiki search "agents"` returns the agents domain page
- `kgx wiki list --category summary` returns 36 pages
- `kgx wiki lint` reports 0 broken wikilinks

## Phase 3: Lint Pipeline

**Goal:** Get `crux run pipelines/devobs_lint.crux` working end-to-end.

### Tasks

1. **Validate frontmatter** — check each PROJECT note against
   `schemas/project.schema.json`. Report missing fields, invalid status
   values, broken repo URLs.

2. **obfsck scan** — run obfsck over all markdown files. Verify no secrets
   leak into the public vault.

3. **Schema validation for kgx payloads** — validate
   `.kgx/data/graph.json` against `schemas/kgx-ingest.schema.json`.

### Acceptance

- Pipeline exits 0 on clean vault
- Pipeline exits 1 and reports findings on intentionally broken note

## Phase 4: Enrich Pipeline (LLM-Assisted)

**Goal:** Use LLM to discover relationships not captured in Cargo.toml /
frontmatter — conceptual connections, shared patterns, architectural
similarities.

### Tasks

1. **Implement enrich pipeline** — feed project descriptions + existing
   graph edges to an LLM, ask for suggested new relationships with
   confidence scores.

2. **Review + commit** — human reviews suggestions before they become
   graph edges. Pipeline outputs candidates to a review file, not
   directly to kgx.

3. **Selective ingest** — approved candidates get added via
   `kgx graph add-edge`.

### Acceptance

- Enrich produces >= 10 candidate relationships
- At least 5 are accepted after review
- No hallucinated project names

## Phase 5: Ongoing Maintenance

1. **New project onboarding** — when a new repo is created, add a
   PROJECT note and re-run ingest.
2. **Periodic lint** — `crux run Cruxfile` in CI or as a pre-push hook.
3. **Wiki refresh** — after any bulk frontmatter change, re-run wiki
   scripts.
4. **Graph health** — monthly `kgx wiki lint` + `kgx stats` check.

## Order of Operations

```
Phase 1 (relationships) ─── must come first, everything else builds on edges
  │
  ├── Phase 2 (wiki) ────── can start after Phase 1
  │
  ├── Phase 3 (lint) ────── independent of Phase 2
  │
  └── Phase 4 (enrich) ──── depends on Phase 1 + Phase 2 for context
```

Phase 5 is ongoing after all phases complete.
