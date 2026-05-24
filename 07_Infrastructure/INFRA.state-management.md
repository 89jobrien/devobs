---
type: infrastructure
tags: [infrastructure, state, handoff]
---

# State Management

Session state, task tracking, and handoff continuity across
agent sessions.

## Projects

| Project | Role                                |
| ------- | ----------------------------------- |
| hj      | Handoff state management CLI        |
| doob    | Todo/task tracker with GitHub sync  |
| valerie | Task lifecycle reconciliation       |
| warpx   | Agentic dev environment (Warp fork) |
| devkit  | Shared CLI toolkit modules          |

## Data Flow

```
warpx ──feeds_into──> doob
hj <──adjacent──> doob (todo data feeds into handoff state)
hj <──adjacent──> warpx (warpx collapses handoff state)
hj <──adjacent──> valerie (valerie reconciles handoff state)
doob <──adjacent──> valerie (both manage task lifecycle)
devkit <──complements──> doob
valerie <──complements──> warpx
```

hj is the central node: it consumes task data from doob, receives
collapsed state from warpx, and valerie reconciles the results.
