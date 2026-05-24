---
title: praxis
source_document: project_praxis
tags: [summary, project, rust]
---

# praxis

**Status:** active
**Languages:** rust
**Domains:** benchmarking, agents

Agent benchmark harness with run-cycle extraction

## Relationships

- [[crux]] (depends_on): cruxx-improve crate via path dep (currently broken)
- [[looprs]] (observes): praxis is an agent benchmark harness, and looprs is a core agent framework whose runs could be benchmarked and analyzed.
- [[braid]] (observes): braid is a multi-agent orchestration platform, making it a natural target for praxis's run-cycle extraction and benchmark observation.
- [[langchainx]] (observes): langchainx implements chains, tools, and agents, which align well with praxis's role as a benchmark and run-analysis harness.

## References

- Repo: https://github.com/89jobrien/praxis.git
- Source note: 02_Projects/PROJECT.praxis.md
