---
type: project
status: active
language_stack:
  - rust
domain:
  - security
  - scanning
repo:
  url: git@github.com:89jobrien/obfsck.git
  path: ~/dev/obfsck
primary_machine: m5-max
project_relationships:
  - type: complements
    target: agentlint
    notes: both are scanning/linting tools for code quality
  - type: complements
    target: sanctum
    notes:
      obfsck handles secret scanning and redaction, while sanctum provides session
      security and auth guarding; both address complementary security concerns.
tags:
  - project
  - rust
---

# obfsck

Secret scanner and redaction CLI

## References

- Repo: `~/dev/obfsck`
