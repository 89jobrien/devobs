---
title: Domain - security
tags: [topic, domain, security]
---

# security

2 projects in this domain (1 active, 1 inactive/archived).

## Active Projects

- **[[obfsck]]** -- Secret scanner and redaction CLI

## Inactive / Archived

- [[sanctum]] -- Session security and auth guard.

## Internal Relationships

- [[obfsck]] --complements--> [[sanctum]]: obfsck handles secret scanning and redaction, while sanctum provides session security and auth guarding; both address complementary security concerns.

## Cluster Graph

```
┌────────┐
│ obfsck │
└────┬───┘
     │ complements
     │
     v
┌─────────┐
│ sanctum │
└─────────┘
```
