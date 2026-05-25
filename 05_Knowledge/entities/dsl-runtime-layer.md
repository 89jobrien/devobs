---
id: "5a58e609-f919-44e8-8ac3-a6674c8588f2"
type: architecture
source_docs:
  - "infra_agent_runtime_stack"
---

# dsl-runtime-layer

DSL/Runtime: crux (DSL + trace model), slashcrux

## Relations

- **depends_on** -> [[entities/execution-layer|execution-layer]] (confidence: 0.90)
- **depends_on** <- [[entities/plugin-layer|plugin-layer]] (confidence: 1.00)
- **member_of** <- [[entities/crux|crux]] (confidence: 1.00)
- **member_of** <- [[entities/slashcrux|slashcrux]] (confidence: 1.00)

## Source Documents

- [[documents/infra_agent_runtime_stack|Agent Runtime Stack]]

## Relevant Chunks

> The agentic infrastructure spans four layers, each built in Rust. Plugins: godmode, atelier, orca-strait, sanctum. Marketplace: bazaar. DSL/Runtime: crux, slashcrux. Execution: minibox, sandbox. godmode and minibox both depend on crux. slashcrux bridges slash with crux. Distributed via bazaar.
> -- from [[documents/infra_agent_runtime_stack|Agent Runtime Stack]], chunk 0

## Wiki Pages

- [[wiki/entity/dsl-runtime-layer|dsl-runtime-layer (wiki)]]
