---
title: Domain - infrastructure
tags: [topic, domain, infrastructure]
---

# infrastructure

2 projects in this domain (0 active, 2 inactive/archived).

## Active Projects

## Inactive / Archived

- [[dotfiles]] -- Machine bootstrap — Nix flakes, GNU Stow, mise.
- [[notfiles]] -- Rust replacement for dotfiles bootstrap.

## Internal Relationships

- [[notfiles]] --replaces--> [[dotfiles]]: notfiles is explicitly described as a Rust replacement for the dotfiles bootstrap workflow.

## Cluster Graph

```
┌──────────┐
│ notfiles │
└─────┬────┘
      │ replaces
      │
      v
┌──────────┐
│ dotfiles │
└──────────┘
```
