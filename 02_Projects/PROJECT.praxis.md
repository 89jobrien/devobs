---
type: project
status: active
language_stack:
  - rust
domain:
  - benchmarking
  - agents
repo:
  url: https://github.com/89jobrien/praxis.git
  path: ~/dev/praxis
primary_machine: m5-max
project_relationships:
  - type: depends_on
    target: crux
    notes: cruxx-improve crate via path dep (currently broken)
  - type: observes
    target: looprs
    notes:
      praxis is an agent benchmark harness, and looprs is a core agent framework
      whose runs could be benchmarked and analyzed.
  - type: observes
    target: braid
    notes:
      braid is a multi-agent orchestration platform, making it a natural target
      for praxis's run-cycle extraction and benchmark observation.
  - type: observes
    target: langchainx
    notes:
      langchainx implements chains, tools, and agents, which align well with praxis's
      role as a benchmark and run-analysis harness.
tags:
  - project
  - rust
---

# praxis

Agent benchmark harness with run-cycle extraction

## References

- Repo: `~/dev/praxis`
