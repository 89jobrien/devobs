---
title: Domain - developer-tools
tags: [topic, domain, developer-tools]
---

# developer-tools

9 projects in this domain (5 active, 4 inactive/archived).

## Active Projects

- **[[atelier]]** -- Claude Code plugin — skills, agents, forge/sentinel/herald.
- **[[cannibalizer]]** -- Codebase classifier with precedence rules.
- **[[devkit]]** -- Go dev workflow toolkit — standup, council, timeline.
- **[[doob]]** -- Todo and task tracker with sync adapters.
- **[[warpx]]** -- Handoff collapse and todo grouping tool.

## Inactive / Archived

- [[harvestrs]] -- Rust data harvester.
- [[hooklings]] -- Feature-complete hook framework.
- [[orca-strait]] -- Claude Code plugin.
- [[valerie]] -- Handoff reconciliation tool.

## Internal Relationships

- [[atelier]] --sibling_of--> [[orca-strait]]: both are Claude Code plugins
- [[devkit]] --complements--> [[doob]]: devkit supports standups, councils, and timelines, while doob manages todos and tasks; both support developer workflow coordination.
- [[doob]] --adjacent_to--> [[valerie]]: both manage task lifecycle
- [[orca-strait]] --sibling_of--> [[atelier]]: both are Claude Code plugins
- [[valerie]] --complements--> [[warpx]]: valerie reconciles handoff state, while warpx collapses handoffs and groups todos; both operate on adjacent parts of the handoff-management workflow.
- [[warpx]] --feeds_into--> [[doob]]: warpx groups todos from handoff context, which could feed into doob's task-tracking and sync workflow.

## Cluster Graph

```
atelier -> orca-strait [sibling_of]
orca-strait -> atelier [sibling_of]

devkit -> doob [complements]
doob -> valerie [adjacent_to]
valerie -> warpx [complements]
warpx -> doob [feeds_into]
```
