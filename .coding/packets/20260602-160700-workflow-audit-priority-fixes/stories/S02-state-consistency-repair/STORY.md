# Story: State consistency repair

## Description

Address audited state inconsistencies around committed/done packet and story state, ship_ready type, current pointer behavior, and state repair coverage without manual .coding markdown edits.

## Acceptance Criteria

- State repair handles packet/story artifacts consistently
- ship_ready is represented as a boolean where state is repaired or updated
- current pointer behavior is explicit and does not conflict silently with state.json
- Relevant CLI smoke tests cover repair or status output

## Files to Change

- src/main.rs
- .coding/state.json
