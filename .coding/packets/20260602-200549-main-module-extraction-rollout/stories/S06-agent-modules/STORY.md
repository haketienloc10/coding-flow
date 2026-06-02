# Story: Extract agent provider runner and JSON parsing

## Description

Move agent provider configuration, command execution, and agent JSON extraction into agent modules.

## Acceptance Criteria

- Agent config resolution is separated from command handlers
- Agent JSON extraction tests still pass
- Agent doctor/providers routing still works
- Tests pass

## Files to Change

- src/lib.rs
- src/agent/
