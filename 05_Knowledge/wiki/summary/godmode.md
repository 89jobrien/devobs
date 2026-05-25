---
slug: "godmode"
category: "summary"
---

# Summary

---

title: godmode
source_document: project_godmode
tags: [summary, project, rust]

---

# godmode

**Status:** active
**Languages:** rust
**Domains:** plugins, agents

Claude Code plugin — skills, agents, task graphs

## Relationships

- [[crux]] (depends_on): cruxx-core crate via crates.io
- [[slashcrux]] (depends_on): slashcrux crate via crates.io
- [[atelier]] (sibling_of): both are Claude Code plugins
- [[orca-strait]] (sibling_of): both are Claude Code plugins

## Relationship Graph

```
┌─────────┐   sibling_of   ┌─────────┐
│ godmode ├─┬─────────────>│ atelier │
└─────────┘ │              └─────────┘
            │
            │ depends_on   ┌──────┐
            ├─────────────>│ crux │
            │              └──────┘
            │
            │ sibling_of   ┌─────────────┐
            ├─────────────>│ orca-strait │
            │              └─────────────┘
            │
            │ depends_on   ┌───────────┐
            └─────────────>│ slashcrux │
                           └───────────┘
```

## References

- Repo: https://github.com/89jobrien/godmode.git
- Source note: 02_Projects/PROJECT.godmode.md

## Backlinks

- [[entities/godmode|godmode]]
