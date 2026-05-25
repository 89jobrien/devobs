---
title: State Management System
tags: [entity, architecture, state, handoff]
---

# State Management System

Session state, task tracking, and handoff continuity across
agent sessions.

## Members

- **[[hj]]** — Central handoff state management CLI
- **[[doob]]** — Todo/task tracker with GitHub sync
- **[[valerie]]** — Task lifecycle reconciliation
- **[[warpx]]** — Agentic dev environment (collapses handoff state)
- **[[devkit]]** — Shared CLI toolkit modules

## Data Flow

`warpx -> doob` (feeds_into)
`hj <-> doob` (todo data feeds handoff state)
`hj <-> warpx` (warpx collapses handoff state)
`hj <-> valerie` (valerie reconciles handoff state)

hj is the central node in this system.

## Source

- `07_Infrastructure/INFRA.state-management.md`
