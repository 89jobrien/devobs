---
title: DSL Runtime Layer
tags: [entity, architecture, layer]
---

# DSL Runtime Layer

Core runtime layer of the [[agent-runtime-stack]].

## Members

- [[crux]] — agentic Rust DSL and runtime trace model
- [[slashcrux]] — slash-command parser integrated with crux

Consumed by [[plugin-layer]] (godmode) and [[execution-layer]] (minibox).
