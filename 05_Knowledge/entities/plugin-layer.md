---
id: "8f2dc440-e420-4693-b885-0819cec27066"
type: architecture
source_docs:
  - "infra_agent_runtime_stack"
---

# plugin-layer

Plugins: godmode, atelier, orca-strait, sanctum

## Relations

- **distributed_via** -> [[entities/marketplace-layer|marketplace-layer]] (confidence: 1.00)
- **depends_on** -> [[entities/dsl-runtime-layer|dsl-runtime-layer]] (confidence: 1.00)
- **member_of** <- [[entities/godmode|godmode]] (confidence: 1.00)
- **member_of** <- [[entities/atelier|atelier]] (confidence: 1.00)
- **member_of** <- [[entities/orca-strait|orca-strait]] (confidence: 1.00)
- **member_of** <- [[entities/sanctum|sanctum]] (confidence: 1.00)

## Source Documents

- [[documents/infra_agent_runtime_stack|Agent Runtime Stack]]

## Relevant Chunks

> The agentic infrastructure spans four layers, each built in Rust. Plugins: godmode, atelier, orca-strait, sanctum. Marketplace: bazaar. DSL/Runtime: crux, slashcrux. Execution: minibox, sandbox. godmode and minibox both depend on crux. slashcrux bridges slash with crux. Distributed via bazaar.
> -- from [[documents/infra_agent_runtime_stack|Agent Runtime Stack]], chunk 0

## Wiki Pages

- [[wiki/entity/plugin-layer|plugin-layer (wiki)]]
