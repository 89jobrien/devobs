---
type: infrastructure
tags: [infrastructure, agents, runtime]
---

# Agent Runtime Stack

The agentic infrastructure spans four layers, each built in Rust.

## Layer Map

```
  Plugins         godmode, atelier, orca-strait, sanctum
     |
  Marketplace     bazaar (distribution + showcase)
     |
  DSL / Runtime   crux (DSL + trace model)
                  slashcrux (slash + crux integration)
     |
  Execution       minibox (container runtime)
                  sandbox (execution tracing + isolation)
```

## Plugin Layer

Claude Code plugins that ship skills, agents, and workflows.

| Project     | Role                                    |
| ----------- | --------------------------------------- |
| godmode     | Task graphs, parallel agents, skills    |
| atelier     | Forge, sentinel, herald, handoff agents |
| orca-strait | Parallel TDD subagent orchestrator      |
| sanctum     | 1Password session security              |

Distributed via **bazaar**, a plugin marketplace that consumes
metadata from each plugin project.

## DSL / Runtime Layer

| Project   | Role                                      |
| --------- | ----------------------------------------- |
| crux      | Agentic Rust DSL and runtime trace model  |
| slashcrux | Slash-command parser integrated with crux |

godmode and minibox both depend on crux. slashcrux bridges the
slash command language with crux's pipeline model.

## Execution Layer

| Project | Role                                       |
| ------- | ------------------------------------------ |
| minibox | Container runtime (hexagonal architecture) |
| sandbox | Execution tracing, subshell isolation      |

minibox manages containers via Tailscale + SSH. sandbox provides
controlled execution environments adjacent to minibox.
