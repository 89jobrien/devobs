---
title: Domain - agents
tags: [topic, domain, agents]
---

# agents

9 projects in this domain (8 active, 1 inactive/archived).

## Active Projects

- **[[agentlint]]** -- Linter for AI agent patterns — sshpass, sleep, frontmatter heuristics
- **[[braid]]** -- Multi-agent orchestration platform.
- **[[crux]]** -- Agentic Rust DSL and runtime trace model
- **[[godmode]]** -- Claude Code plugin — skills, agents, task graphs
- **[[langchainx]]** -- Rust port of LangChain — tools, chains, agents
- **[[looprs]]** -- Core Rust agent framework — workspace v0.3.1.
- **[[praxis]]** -- Agent benchmark harness with run-cycle extraction
- **[[slashcrux]]** -- Slash + Crux integration workspace.

## Inactive / Archived

- [[steve]] -- Python agent framework with skills and commands.

## Internal Relationships

- [[braid]] --sibling_of--> [[looprs]]: both are Rust agent frameworks
- [[braid]] --sibling_of--> [[langchainx]]: both are Rust agent frameworks
- [[crux]] --depends_on--> [[slashcrux]]: slashcrux crate via crates.io
- [[godmode]] --depends_on--> [[crux]]: cruxx-core crate via crates.io
- [[godmode]] --depends_on--> [[slashcrux]]: slashcrux crate via crates.io
- [[langchainx]] --sibling_of--> [[looprs]]: both are Rust agent frameworks
- [[langchainx]] --sibling_of--> [[braid]]: both are Rust agent frameworks
- [[looprs]] --sibling_of--> [[braid]]: both are Rust agent frameworks
- [[looprs]] --sibling_of--> [[langchainx]]: both are Rust agent frameworks
- [[praxis]] --depends_on--> [[crux]]: cruxx-improve crate via path dep (currently broken)
- [[praxis]] --observes--> [[looprs]]: praxis is an agent benchmark harness, and looprs is a core agent framework whose runs could be benchmarked and analyzed.
- [[praxis]] --observes--> [[braid]]: braid is a multi-agent orchestration platform, making it a natural target for praxis's run-cycle extraction and benchmark observation.
- [[praxis]] --observes--> [[langchainx]]: langchainx implements chains, tools, and agents, which align well with praxis's role as a benchmark and run-analysis harness.
- [[slashcrux]] --depends_on--> [[crux]]: integrates crux DSL with slash

## Cluster Graph

```
braid -> looprs [sibling_of]
braid -> langchainx [sibling_of]
crux -> slashcrux [depends_on]
godmode -> crux [depends_on]
godmode -> slashcrux [depends_on]
langchainx -> looprs [sibling_of]
langchainx -> braid [sibling_of]
looprs -> braid [sibling_of]
looprs -> langchainx [sibling_of]
praxis -> crux [depends_on]
praxis -> looprs [observes]
praxis -> braid [observes]
praxis -> langchainx [observes]
slashcrux -> crux [depends_on]
```
