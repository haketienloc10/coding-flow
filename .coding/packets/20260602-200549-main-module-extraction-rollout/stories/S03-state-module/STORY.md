# Story: Extract workflow state module

## Description

Move state loading, saving, current pointer, task/story/packet resolution, and state sync logic into src/state.rs.

## Acceptance Criteria

- State APIs are isolated in src/state.rs
- .coding/state.json remains canonical
- .coding/current compatibility is preserved
- State repair and story switch tests pass

## Files to Change

- src/lib.rs
- src/state.rs
