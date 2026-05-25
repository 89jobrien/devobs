---
id: "12486ab2-59c3-40eb-b98c-409f3fbfb39a"
type: architecture
source_docs:
  - "infra_agent_runtime_stack"
---

# execution-layer

Execution: minibox (container runtime), sandbox

## Relations

- **depends_on** <- [[entities/dsl-runtime-layer|dsl-runtime-layer]] (confidence: 0.90)
- **member_of** <- [[entities/minibox|minibox]] (confidence: 1.00)
- **member_of** <- [[entities/sandbox|sandbox]] (confidence: 1.00)

## Source Documents

- [[documents/infra_agent_runtime_stack|Agent Runtime Stack]]

## Relevant Chunks

> The agentic infrastructure spans four layers, each built in Rust. Plugins: godmode, atelier, orca-strait, sanctum. Marketplace: bazaar. DSL/Runtime: crux, slashcrux. Execution: minibox, sandbox. godmode and minibox both depend on crux. slashcrux bridges slash with crux. Distributed via bazaar.
> -- from [[documents/infra_agent_runtime_stack|Agent Runtime Stack]], chunk 0

## Wiki Pages

- [[wiki/entity/execution-layer|execution-layer (wiki)]]
