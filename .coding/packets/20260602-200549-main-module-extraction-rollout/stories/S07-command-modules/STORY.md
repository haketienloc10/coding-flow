# Story: Extract command domain modules

## Description

Move task, packet, story, state, next, and ship command handlers into command modules behind the CLI dispatcher.

## Acceptance Criteria

- CLI dispatcher is small and delegates to command modules
- Task packet story commands remain compatible
- No unrelated behavior changes
- Full test suite passes

## Files to Change

- src/lib.rs
- src/commands/
