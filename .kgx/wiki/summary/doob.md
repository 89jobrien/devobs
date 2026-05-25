---
title: doob
source_document: project_doob
tags: [summary, project, rust]
---

# doob

**Status:** active
**Languages:** rust
**Domains:** developer-tools, task-management

Todo and task tracker with sync adapters.

## Relationships

- [[hj]] (adjacent_to): todo data feeds into handoff state
- [[valerie]] (adjacent_to): both manage task lifecycle

## Relationship Graph

```
┌──────┐   adjacent_to   ┌────┐
│ doob ├─┬──────────────>│ hj │
└──────┘ │               └────┘
         │
         │ adjacent_to   ┌─────────┐
         └──────────────>│ valerie │
                         └─────────┘
```

## References

- Repo: https://github.com/89jobrien/doob.git
- Source note: 02_Projects/PROJECT.doob.md
