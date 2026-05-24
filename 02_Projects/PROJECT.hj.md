---
type: project
status: active
language_stack: [rust]
domain: [handoff, state-management]
repo:
  url: git@github.com:89jobrien/hj.git
  path: ~/dev/hj
primary_machine: m5-max
project_relationships:
  - type: adjacent_to
    target: doob
    notes: "handoff state consumes todo/task data"
  - type: adjacent_to
    target: warpx
    notes: "warpx collapses handoff state"
  - type: adjacent_to
    target: valerie
    notes: "valerie reconciles handoff state"
tags: [project, rust]
---

# hj

Handoff CLI — YAML state management for session continuity

## References

- Repo: `~/dev/hj`
