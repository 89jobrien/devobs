---
title: Hook Infrastructure
tags: [entity, architecture, hooks]
---

# Hook Infrastructure

Pre-commit, runtime, and agent hook systems spanning five projects.

## Members

- **[[coursers]]** — CLI shortcut learning from shell history
- **[[hooklings]]** — Hook framework for agent/tool events
- **[[prefixe]]** — Prefix resolution logic (consumed by coursers)
- **[[obfsck]]** — Secret scanning in pre-commit hooks
- **[[agentlint]]** — CLAUDE.md and agent config quality gates

## Dependency Chain

`coursers -> prefixe` (depends_on)
`coursers <-> hooklings <-> prefixe` (adjacent)

## Source

- `07_Infrastructure/INFRA.hook-systems.md`
