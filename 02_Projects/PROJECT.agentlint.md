---
type: project
status: active
language_stack:
  - rust
domain:
  - agents
  - linting
repo:
  url: https://github.com/89jobrien/agentlint.git
  path: ~/dev/agentlint
primary_machine: m5-max
project_relationships:
  - type: complements
    target: obfsck
    notes: both are scanning/linting tools for code quality
  - type: complements
    target: sandbox
    notes:
      agentlint statically detects risky AI-agent patterns, while sandbox provides
      execution tracing and isolation; together they cover preventive and runtime safety.
tags:
  - project
  - rust
---

# agentlint

Linter for AI agent patterns — sshpass, sleep, frontmatter heuristics

## References

- Repo: `~/dev/agentlint`
