# ASCII Relationship Graphs in Wiki Pages

**Date:** 2026-05-25
**Status:** Approved

## Goal

Add ASCII box-and-arrow relationship graphs to the wiki pages generated
by `devobs.rs wiki-summaries` (per-project ego graphs) and
`devobs.rs wiki-domains` (per-cluster graphs). Pure Rust, no external
dependencies or runtime tools.

## Architecture

### Affected code

- `scripts/devobs.rs` — all changes are in this single file

### New functions

```
render_graph(nodes, edges) -> String    // entry point, picks LR vs TD
layout_lr(nodes, edges) -> LayoutPlan   // left-to-right layer assignment
layout_td(nodes, edges) -> LayoutPlan   // top-down layer assignment
draw_canvas(plan) -> String             // paint boxes + connectors
render_fallback(nodes, edges) -> String // flat list for >15 nodes
```

### Data flow

1. Existing subcommand collects nodes and edges from frontmatter
2. `render_graph()` called with node names and typed edges
3. If >15 nodes, `render_fallback()` returns adjacency list
4. Otherwise, pick orientation:
   - Compute max_depth (longest path) and max_width (widest layer)
   - `max_depth >= max_width` -> TD, else LR
5. Layout assigns (col, row) to each node box
6. `draw_canvas()` paints onto a `Vec<Vec<char>>` grid and returns
   the joined string

### Graph types

**Ego graph** (wiki-summaries):

- Center node = current project
- All direct relationship targets (outbound)
- All projects that target this project (inbound)
- Up to ~8 nodes including inbound edges; many projects have fewer

**Cluster graph** (wiki-domains):

- All projects in the domain
- Only edges where both endpoints are in the cluster
- Capped at 15 nodes; above that, flat fallback

### LR layout

```
┌──────────┐              ┌─────────┐
│ atelier  ├─┬─sibling───>│ godmode │
└──────────┘ │            └─────────┘
             │ consumes   ┌────────────┐
             └───────────>│ orca-strait│
                          └────────────┘
```

- Source box `├` on middle row of right edge
- Vertical rail: `┬` for non-last branches, `└` for last
- Horizontal `─` with label inline, `>` at terminus
- Stacked targets separated by 2 rows

### TD layout

```
       ┌─────────┐
       │ atelier │
       └────┬────┘
            │
     ┌──────┴──────────┐
     │ sibling_of      │ consumes_from
     v                 v
┌─────────┐      ┌────────────┐
│ godmode │      │ orca-strait│
└─────────┘      └────────────┘
```

- Vertical `│` from source, horizontal `─` rail with drop points
- Each drop: `│` down with label beside, `v` at terminus
- Layers spaced 4 rows apart (box=3 + connector=1)

### Fallback (>15 nodes)

```
atelier -> godmode [sibling_of]
atelier -> orca-strait [sibling_of]
```

### Integration points

**wiki-summaries:** new `## Relationship Graph` section after the
existing `## Relationships` bullet list.

**wiki-domains:** new `## Cluster Graph` section after the existing
`## Internal Relationships` bullet list.

## Tech decisions

- **No external deps** — `petgraph` would be cleaner but adds a dep
  to a rust-script. The graphs are small enough that hand-rolled
  topo-sort + layer assignment is trivial.
- **LR vs TD heuristic** — depth >= width means the graph is tall,
  so TD. Otherwise LR to avoid excessive vertical space.
- **15-node cap** — box-and-arrow layout degrades above this. The
  flat fallback is always readable.
- **Edge label truncation** — cap at 18 chars to fit longest
  relationship types (`aligns_schema_with`, `control_plane_for`).

## Out of scope

- Interactive or HTML graph rendering
- Graphviz/DOT output
- Bidirectional edge merging (A->B and B->A shown separately)
- Changes to devobs.yaml config
- Changes to ingest, lint, enrich, or apply subcommands
