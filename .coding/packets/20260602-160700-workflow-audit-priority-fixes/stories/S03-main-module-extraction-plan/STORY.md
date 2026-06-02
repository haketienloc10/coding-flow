# Story: Main module extraction foundation

## Description

Create a conservative first slice toward decomposing the src/main.rs monolith without broad behavior changes.

## Acceptance Criteria

- A small low-risk domain is extracted from src/main.rs into a module
- Public boundaries are clear and compile cleanly
- Existing tests and CLI smoke checks still pass

## Files to Change

- src/main.rs
- src/
