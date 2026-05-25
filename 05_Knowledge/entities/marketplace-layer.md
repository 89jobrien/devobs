---
id: "c8596916-67f3-498e-8a97-8e8123ede205"
type: architecture
source_docs:
  - "infra_agent_runtime_stack"
---

# marketplace-layer

Marketplace: bazaar (distribution + showcase)

## Relations

- **distributed_via** <- [[entities/plugin-layer|plugin-layer]] (confidence: 1.00)
- **member_of** <- [[entities/bazaar|bazaar]] (confidence: 1.00)

## Source Documents

- [[documents/infra_agent_runtime_stack|Agent Runtime Stack]]

## Relevant Chunks

> The agentic infrastructure spans four layers, each built in Rust. Plugins: godmode, atelier, orca-strait, sanctum. Marketplace: bazaar. DSL/Runtime: crux, slashcrux. Execution: minibox, sandbox. godmode and minibox both depend on crux. slashcrux bridges slash with crux. Distributed via bazaar.
> -- from [[documents/infra_agent_runtime_stack|Agent Runtime Stack]], chunk 0

## Wiki Pages

- [[wiki/entity/marketplace-layer|marketplace-layer (wiki)]]
