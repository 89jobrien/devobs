---
title: bazaar
source_document: project_bazaar
tags: [summary, project, rust]
---

# bazaar

**Status:** active
**Languages:** rust
**Domains:** plugins, marketplace

Claude Code plugin marketplace and showcase

## Relationships

- [[godmode]] (consumes_from): bazaar is a Claude Code plugin marketplace/showcase, and godmode is a Claude Code plugin that would naturally be listed or showcased there.
- [[atelier]] (consumes_from): atelier is a Claude Code plugin, making it a likely source of plugin metadata or examples for bazaar's marketplace/showcase role.
- [[orca-strait]] (consumes_from): orca-strait is another Claude Code plugin and fits bazaar's role as a marketplace and showcase for such plugins.

## Relationship Graph

```
┌────────┐   consumes_from   ┌─────────┐
│ bazaar ├─┬────────────────>│ atelier │
└────────┘ │                 └─────────┘
           │
           │ consumes_from   ┌─────────┐
           ├────────────────>│ godmode │
           │                 └─────────┘
           │
           │ consumes_from   ┌─────────────┐
           └────────────────>│ orca-strait │
                             └─────────────┘
```

## References

- Repo: git@github.com:89jobrien/bazaar.git
- Source note: 02_Projects/PROJECT.bazaar.md
