---
type: project
status: active
language_stack: [rust]
domain: [containers, runtime]
repo:
  url: git@github.com:89jobrien/minibox.git
  path: ~/dev/minibox
primary_machine: m5-max
project_relationships:
  - type: depends_on
    target: crux
    notes: "crux-plugin crate via git rev pin"
tags: [project, rust]
---

# minibox

Docker-like container runtime in Rust with hexagonal architecture

## References

- Repo: `~/dev/minibox`
