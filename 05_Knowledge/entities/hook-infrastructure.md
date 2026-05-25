---
id: "ba5a1112-a67b-4fb6-83a4-8eabdd9121f1"
type: architecture
source_docs:
  - "infra_hook_systems"
---

# hook-infrastructure

Pre-commit, runtime, and agent hook infrastructure across projects

## Relations

- **member_of** <- [[entities/coursers|coursers]] (confidence: 1.00)
- **member_of** <- [[entities/hooklings|hooklings]] (confidence: 1.00)
- **member_of** <- [[entities/prefixe|prefixe]] (confidence: 1.00)
- **member_of** <- [[entities/obfsck|obfsck]] (confidence: 1.00)
- **member_of** <- [[entities/agentlint|agentlint]] (confidence: 1.00)

## Source Documents

- [[documents/infra_hook_systems|Hook Systems]]

## Relevant Chunks

> Pre-commit, runtime, and agent hook infrastructure. coursers depends on prefixe for prefix resolution. hooklings provides a complementary hook framework. obfsck and agentlint run as lint steps in CI and pre-commit pipelines.
> -- from [[documents/infra_hook_systems|Hook Systems]], chunk 0

## Wiki Pages

- [[wiki/entity/hook-infrastructure|hook-infrastructure (wiki)]]
