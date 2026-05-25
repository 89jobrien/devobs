---
title: bamlish
source_document: project_bamlish
tags: [summary, project, rust]
---

# bamlish

**Status:** active
**Languages:** rust
**Domains:** ai, structured-output

BAML-related Rust tooling for structured LLM output.

## Relationships

- [[rslm]] (complements): bamlish focuses on structured LLM output tooling, while rslm provides Rust LLM integration; structured output is a natural layer on top of LLM integration.

## Relationship Graph

```
┌─────────┐
│ bamlish │
└────┬────┘
     │ complements
     │
     v
 ┌──────┐
 │ rslm │
 └──────┘
```

## References

- Repo: https://github.com/89jobrien/bamlish.git
- Source note: 02_Projects/PROJECT.bamlish.md
