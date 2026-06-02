# Story: Extract problem and decision registries

## Description

Move problem and decision parsing, rendering, lifecycle, and commands into knowledge modules.

## Acceptance Criteria

- Problem registry code lives under src/knowledge/problems.rs
- Decision registry code lives under src/knowledge/decisions.rs
- CLI problem and decision commands still work
- Related tests pass

## Files to Change

- src/lib.rs
- src/knowledge/
