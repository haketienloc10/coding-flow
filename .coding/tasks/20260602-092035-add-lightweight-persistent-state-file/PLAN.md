# Code Plan

## 1. Objective

Add a lightweight persistent state file to cflow at .coding/state.json

## 2. Scope

### In Scope

- Implement State and TaskMetadata structs in Rust.
- Support reading and writing .coding/state.json.
- Update state after command execution (new, request, plan, coding, verify, ship).
- Support tasks, status, switch, and state repair commands.
- Prefer state.json.current_task_id as source of truth for current task.

### Out of Scope

- No other JSON files in .coding except state.json.
- No full markdown content stored in state.json.

## 3. Requirements

- State file must be human-readable and lightweight JSON.
- State file and filesystem mismatch must trigger warnings in status.

## 4. Technical Approach

- Define State and TaskMetadata serialize/deserialize structures.
- Modify resolve_task to use state.json current_task_id.
- Implement cflow tasks, switch, status, and state repair command routing.
- Integrate state updates into all commands.

## 5. Files to Change

- src/main.rs

## 6. Implementation Steps

- [todo] Define State structures in src/main.rs.
- [todo] Implement State load/save and update helpers.
- [todo] Refactor resolve_task to read current task from state.json.
- [todo] Integrate state updates after cflow commands: new, request, plan, coding, verify, ship.
- [todo] Implement cflow tasks command.
- [todo] Implement cflow switch command.
- [todo] Implement cflow status command with state vs filesystem verification.
- [todo] Implement cflow state repair command.

## 7. Test Plan

### Planned

- Verify that creating a task updates state.json.
- Verify that status output parses state.json correctly.
- Verify repair command rebuilds state.json if it is deleted or corrupted.

### Result

- _None_

## 8. Risks

- Breaking backward compatibility with .coding/current if not handled correctly.

## 9. Done Criteria

### Criteria

- state.json is the source of truth for current_task_id.
- All required metadata is stored and updated.
- New status, tasks, switch, and state repair commands work correctly.

### Verified

- _None_
