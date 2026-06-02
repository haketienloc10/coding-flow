# Code Plan

## 1. Objective

Repair packet/story state consistency for S02.

## 2. Scope

### In Scope

- Make packet story switch/list/status resolve from packet state and artifacts.
- Extend state repair to packet and story artifacts.
- Normalize repaired or updated ship_ready values to booleans.
- Make current task/packet/story pointer behavior explicit.

### Out of Scope

- Do not modularize src/main.rs in this story.
- Do not unify the full task and packet data model in this story.
- Do not manually edit .coding markdown artifacts.

## 3. Requirements

- State repair must handle packet/story artifacts consistently.
- ship_ready must be boolean where state is repaired or updated.
- Story commands must work for current packet stories.
- Smoke tests must cover repair or status-related behavior.

## 4. Technical Approach

- Add helpers for story metadata sync, markdown boolean parsing, and packet story lookup.
- Update story command resolution to prefer packet state when a packet is current.
- Extend state repair to scan .coding/packets and reconcile .coding/current.
- Add focused unit tests for story switch and state repair behavior.

## 5. Files to Change

- src/main.rs
- .coding/state.json

## 6. Implementation Steps

- [done] Inspect current state, story, and repair commands.
- [done] Implement packet/story state repair and current pointer fixes.
- [done] Add smoke coverage for story switch and state repair.
- [done] Run format, tests, build, and CLI smoke checks.

## 7. Test Plan

### Planned

- cargo fmt --check
- cargo test
- cargo build
- cargo build --release
- CLI smoke: story switch/status/list and state repair

### Result

- _None_

## 8. Risks

- state repair touches durable .coding/state.json and must not downgrade committed story or packet state.
- bin/cflow may use stale target/debug/cflow unless cargo build refreshes it.

## 9. Done Criteria

### Criteria

- Existing S02 story can be selected through cflow story switch.
- cflow state repair syncs packet/story metadata without manual markdown edits.
- ship_ready is boolean in repaired or updated state.
- Relevant Rust tests and CLI smoke checks pass.

### Verified

- _None_
