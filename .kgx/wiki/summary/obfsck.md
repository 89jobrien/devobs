---
title: obfsck
source_document: project_obfsck
tags: [summary, project, rust]
---

# obfsck

**Status:** active
**Languages:** rust
**Domains:** security, scanning

Secret scanner and redaction CLI

## Relationships

- [[agentlint]] (complements): both are scanning/linting tools for code quality
- [[sanctum]] (complements): obfsck handles secret scanning and redaction, while sanctum provides session security and auth guarding; both address complementary security concerns.

## Relationship Graph

```
┌────────┐   complements   ┌───────────┐
│ obfsck ├─┬──────────────>│ agentlint │
└────────┘ │               └───────────┘
           │
           │ complements   ┌─────────┐
           └──────────────>│ sanctum │
                           └─────────┘
```

## References

- Repo: git@github.com:89jobrien/obfsck.git
- Source note: 02_Projects/PROJECT.obfsck.md
