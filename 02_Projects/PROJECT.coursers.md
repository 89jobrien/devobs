---
type: project
status: active
language_stack: [rust]
domain: [hooks, ci]
repo:
  url: https://github.com/89jobrien/coursers.git
  path: ~/dev/coursers
primary_machine: m5-max
project_relationships:
  - type: depends_on
    target: prefixe
    notes: "prefixe crate via crates.io"
  - type: adjacent_to
    target: hooklings
    notes: "both are hook frameworks"
tags: [project, rust]
---

# coursers

Claude Code hook runner — pre/post tool-use guards

## References

- Repo: `~/dev/coursers`
