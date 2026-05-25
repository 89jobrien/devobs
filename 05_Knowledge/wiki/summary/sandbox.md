---
slug: "sandbox"
category: "summary"
---

# Summary

---

title: sandbox
source_document: project_sandbox
tags: [summary, project, rust]

---

# sandbox

**Status:** active
**Languages:** rust
**Domains:** execution, testing

Execution tracing, subshell isolation, rustqual refactor

## Relationships

- [[minibox]] (adjacent_to): sandbox focuses on execution tracing and isolation, while minibox is a container runtime; both address controlled execution environments from adjacent layers.

## Relationship Graph

```
┌─────────┐
│ sandbox │
└────┬────┘
     │ adjacent_to
     │
     v
┌─────────┐
│ minibox │
└─────────┘
```

## References

- Source note: 02_Projects/PROJECT.sandbox.md

## Backlinks

- [[entities/sandbox|sandbox]]
