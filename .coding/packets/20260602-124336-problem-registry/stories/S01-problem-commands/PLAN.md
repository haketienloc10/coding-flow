# Code Plan

## 1. Objective

Add markdown-backed problem registry commands and docs for cflow.

## 2. Scope

### In Scope

- src/main.rs problem command implementation
- README.md problem section
- AGENTS.md problem rules

### Out of Scope

- state.json counters
- database/persistent problem JSON
- broad CLI refactor

## 3. Requirements

- Generate P### ids from existing PROBLEMS.md entries
- Read add input JSON from stdin and render markdown only
- Validate allowed status/severity/phase values
- Support list filters and status aliases
- Show and mutate matching markdown entries by id

## 4. Technical Approach

- Add constants for problem allowed values and PROBLEMS.md path
- Implement lightweight line/block parsing split on headings matching ## P### - Title
- Append rendered entries for add and rewrite the markdown file for updates
- Wire a new problem command branch into main dispatch and help text
- Document rules in README.md and AGENTS.md

## 5. Files to Change

- src/main.rs
- README.md
- AGENTS.md

## 6. Implementation Steps

- [in_progress] Inspect existing CLI helpers and command dispatch
- [todo] Implement problem parser/render/update commands
- [todo] Update docs
- [todo] Run cargo check and manual command tests

## 7. Test Plan

### Planned

- cargo check
- manual problem add/list/show/resolve/cancel/invalid-status tests

### Result

- _None_

## 8. Risks

- Markdown rewriting could accidentally modify wrong entry
- CLI usage strings may drift from README

## 9. Done Criteria

### Criteria

- Problem commands behave as requested
- Docs mention problem format and lifecycle commands
- No persistent JSON files are introduced
- Manual tests pass

### Verified

- _None_
