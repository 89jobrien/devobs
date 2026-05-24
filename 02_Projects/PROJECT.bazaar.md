---
type: project
status: active
language_stack:
  - rust
domain:
  - plugins
  - marketplace
repo:
  url: git@github.com:89jobrien/bazaar.git
  path: ~/dev/bazaar
primary_machine: m5-max
tags:
  - project
  - rust
project_relationships:
  - type: consumes_from
    target: godmode
    notes:
      bazaar is a Claude Code plugin marketplace/showcase, and godmode is a Claude
      Code plugin that would naturally be listed or showcased there.
  - type: consumes_from
    target: atelier
    notes:
      atelier is a Claude Code plugin, making it a likely source of plugin metadata
      or examples for bazaar's marketplace/showcase role.
  - type: consumes_from
    target: orca-strait
    notes:
      orca-strait is another Claude Code plugin and fits bazaar's role as a marketplace
      and showcase for such plugins.
---

# bazaar

Claude Code plugin marketplace and showcase

## References

- Repo: `~/dev/bazaar`
