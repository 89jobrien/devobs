---
title: Domain - ai
tags: [topic, domain, ai]
---

# ai

8 projects in this domain (7 active, 1 inactive/archived).

## Active Projects

- **[[atelier]]** -- Claude Code plugin — skills, agents, forge/sentinel/herald.
- **[[bamlish]]** -- BAML-related Rust tooling for structured LLM output.
- **[[braid]]** -- Multi-agent orchestration platform.
- **[[looprs]]** -- Core Rust agent framework — workspace v0.3.1.
- **[[mcpipe]]** -- MCP pipeline tool.
- **[[rslm]]** -- Rust LLM integration library.
- **[[slashcrux]]** -- Slash + Crux integration workspace.

## Inactive / Archived

- [[steve]] -- Python agent framework with skills and commands.

## Internal Relationships

- [[bamlish]] --complements--> [[rslm]]: bamlish focuses on structured LLM output tooling, while rslm provides Rust LLM integration; structured output is a natural layer on top of LLM integration.
- [[braid]] --sibling_of--> [[looprs]]: both are Rust agent frameworks
- [[looprs]] --sibling_of--> [[braid]]: both are Rust agent frameworks
- [[mcpipe]] --complements--> [[rslm]]: mcpipe provides MCP pipeline functionality, which complements an LLM integration library by enabling model/tool pipeline composition.

## Cluster Graph

```
bamlish -> rslm [complements]
mcpipe -> rslm [complements]

braid -> looprs [sibling_of]
looprs -> braid [sibling_of]
```
