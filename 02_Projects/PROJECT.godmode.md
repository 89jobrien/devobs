---
type: project
status: active
language_stack: [rust]
domain: [plugins, agents]
repo:
  url: https://github.com/89jobrien/godmode.git
  path: ~/dev/godmode
primary_machine: m5-max
project_relationships:
  - type: depends_on
    target: crux
    notes: "cruxx-core crate via crates.io"
  - type: depends_on
    target: slashcrux
    notes: "slashcrux crate via crates.io"
  - type: sibling_of
    target: atelier
    notes: "both are Claude Code plugins"
  - type: sibling_of
    target: orca-strait
    notes: "both are Claude Code plugins"
tags: [project, rust]
---

# godmode

Claude Code plugin — skills, agents, task graphs

## References

- Repo: `~/dev/godmode`
