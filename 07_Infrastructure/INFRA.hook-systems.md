---
type: infrastructure
tags: [infrastructure, hooks, ci]
---

# Hook Systems

Pre-commit, runtime, and agent hook infrastructure across projects.

## Projects

| Project   | Role                                         |
| --------- | -------------------------------------------- |
| coursers  | CLI shortcut learning from shell history     |
| hooklings | Hook framework for agent/tool events         |
| prefixe   | Prefix resolution logic consumed by coursers |
| obfsck    | Secret scanning in pre-commit hooks          |
| agentlint | CLAUDE.md and agent config quality gates     |

## Dependency Chain

```
coursers ──depends_on──> prefixe
coursers <──adjacent──> hooklings <──adjacent──> prefixe
```

coursers learns from shell history and depends on prefixe for
prefix resolution. hooklings provides a complementary hook
framework. obfsck and agentlint run as lint steps in CI and
pre-commit pipelines.
