---
type: knowledge
generated: true
tags: [knowledge, relationships, map]
---

# Project Relationship Map

Extracted from `project_relationships` frontmatter across all
PROJECT notes. 27 of 37 projects declare at least one relationship.

## Dependency Chains (depends_on)

```
godmode ──> crux
godmode ──> slashcrux
crux ──> slashcrux
slashcrux ──> crux
minibox ──> crux
praxis ──> crux
coursers ──> prefixe
```

## Plugin Siblings (sibling_of)

```
godmode <──> atelier <──> orca-strait
braid <──> looprs <──> langchainx
```

## Complements

```
obfsck <──> agentlint    (scanning/linting)
obfsck <──> sanctum      (security)
agentlint <──> sandbox
propkit <──> sandbox
devkit <──> doob
mcpipe <──> rslm
bamlish <──> rslm
valerie <──> warpx
```

## Adjacent (adjacent_to)

```
coursers <──> hooklings <──> prefixe
sandbox <──> minibox
hj <──> doob
hj <──> warpx
hj <──> valerie
doob <──> valerie
```

## Data Flow (feeds_into / consumes_from)

```
harvestrs ──> kgx
warpx ──> doob
bazaar <── godmode, atelier, orca-strait
```

## Observation (observes)

```
praxis ──> looprs, braid, langchainx
```

## Replacement

```
notfiles ──replaces──> dotfiles
```

## Status Summary

| Status   | Count |
| -------- | ----- |
| active   | 28    |
| inactive | 8     |
| archived | 0     |

## Projects Without Relationships (9)

89jobrien.github.io, cannibalizer, dotfiles, kgx, mlrs,
rslm, rx, steve, sanctum (referenced by others but declares none)
