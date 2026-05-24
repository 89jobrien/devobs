---
type: infrastructure
tags: [infrastructure, agents, frameworks]
---

# Agent Frameworks

Three sibling Rust agent frameworks, each approaching multi-agent
orchestration from a different angle.

## Projects

| Project    | Focus                                |
| ---------- | ------------------------------------ |
| braid      | Multi-agent orchestration platform   |
| looprs     | Agent loop primitives                |
| langchainx | LangChain-style agent chains in Rust |

## Relationships

```
braid <──sibling──> looprs <──sibling──> langchainx
```

All three are active. praxis observes all three as reference
implementations for its own agent improvement work.

## Supporting Projects

| Project | Role                                           |
| ------- | ---------------------------------------------- |
| rslm    | LLM client abstractions                        |
| bamlish | BAML structured output                         |
| mcpipe  | MCP protocol proxy                             |
| praxis  | Agent quality improvement (observes all three) |

bamlish and mcpipe both complement rslm, providing structured
output and protocol bridging respectively.
