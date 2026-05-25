---
title: prefixe
source_document: project_prefixe
tags: [summary, project, rust]
---

# prefixe

**Status:** active
**Languages:** rust
**Domains:** cli, resolution

Post-hook probing resolution CLI with confirm/cycle/exhaust

## Relationships

- [[coursers]] (adjacent_to): prefixe provides resolution logic consumed by coursers
- [[hooklings]] (adjacent_to): both are hook-related tools

## Relationship Graph

```
┌─────────┐   adjacent_to   ┌──────────┐
│ prefixe ├─┬──────────────>│ coursers │
└─────────┘ │               └──────────┘
            │
            │ adjacent_to   ┌───────────┐
            └──────────────>│ hooklings │
                            └───────────┘
```

## References

- Repo: https://github.com/89jobrien/prefixe.git
- Source note: 02_Projects/PROJECT.prefixe.md
