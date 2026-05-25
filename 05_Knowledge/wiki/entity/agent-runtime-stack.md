---
slug: "agent-runtime-stack"
category: "entity"
---

# Entity

---

title: Agent Runtime Stack
tags: [entity, architecture, agents]

---

# Agent Runtime Stack

Four-layer architecture for agentic infrastructure, all built in Rust.

## Layers

- **[[plugin-layer]]** — [[godmode]], [[atelier]], [[orca-strait]], [[sanctum]]
- **[[marketplace-layer]]** — [[bazaar]]
- **[[dsl-runtime-layer]]** — [[crux]], [[slashcrux]]
- **[[execution-layer]]** — [[minibox]], [[sandbox]]

## Key Dependencies

- Plugin layer depends on DSL/runtime layer (godmode depends on crux)
- Execution layer depends on DSL/runtime (minibox depends on crux)
- Plugins distributed via marketplace (bazaar)

## Source

- `07_Infrastructure/INFRA.agent-runtime-stack.md`
