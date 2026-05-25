---
id: "e164c043-43db-48e1-aff4-6d0d77279aad"
type: system
source_docs:
  - "system_crux_pipelines"
---

# devobs-pipeline-system

Automated pipelines for vault maintenance powered by crux DSL

## Relations

- **depends_on** -> [[entities/crux|crux]] (confidence: 1.00)
- **feeds_into** -> [[entities/kgx|kgx]] (confidence: 1.00)
- **depends_on** -> [[entities/obfsck|obfsck]] (confidence: 0.90)
- **depends_on** -> [[entities/agentlint|agentlint]] (confidence: 0.90)
- **feeds_into** -> [[entities/devobs-knowledge-graph|devobs-knowledge-graph]] (confidence: 1.00)

## Source Documents

- [[documents/system_crux_pipelines|Crux Pipelines]]

## Relevant Chunks

> Automated pipelines for vault maintenance powered by crux DSL. Three pipelines: ingest (PROJECT notes into kgx), lint (schema validation, obfsck, agentlint), enrich (LLM-assisted relationship discovery). Cruxfile orchestrates: ci = lint + ingest, enrich depends on ingest.
> -- from [[documents/system_crux_pipelines|Crux Pipelines]], chunk 0
