---
title: Domain - testing
tags: [topic, domain, testing]
---

# testing

2 projects in this domain (2 active, 0 inactive/archived).

## Active Projects

- **[[propkit]]** -- Property test analyzer, generator, and scaffold.
- **[[sandbox]]** -- Execution tracing, subshell isolation, rustqual refactor

## Internal Relationships

- [[propkit]] --complements--> [[sandbox]]: propkit generates and analyzes property tests, while sandbox can provide isolated execution and tracing for running those tests safely.

## Cluster Graph

```
┌─────────┐
│ propkit │
└────┬────┘
     │ complements
     │
     v
┌─────────┐
│ sandbox │
└─────────┘
```
