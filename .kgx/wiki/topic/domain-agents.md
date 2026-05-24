---
title: Domain - agents
tags: [topic, domain, agents]
---

# agents

9 projects in this domain (8 active, 1 inactive/archived).

## Active Projects

- **[[agentlint]]** — Linter for AI agent patterns — sshpass, sleep, frontmatter heuristics
- **[[braid]]** — Multi-agent orchestration platform.
- **[[crux]]** — Agentic Rust DSL and runtime trace model
- **[[godmode]]** — Claude Code plugin — skills, agents, task graphs
- **[[langchainx]]** — Rust port of LangChain — tools, chains, agents
- **[[looprs]]** — Core Rust agent framework — workspace v0.3.1.
- **[[praxis]]** — Agent benchmark harness with run-cycle extraction
- **[[slashcrux]]** — Slash + Crux integration workspace.

## Inactive / Archived

- [[steve]] — Python agent framework with skills and commands.

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
- [[slashcrux]] --depends_on--> [[crux]]: integrates crux DSL with slash
