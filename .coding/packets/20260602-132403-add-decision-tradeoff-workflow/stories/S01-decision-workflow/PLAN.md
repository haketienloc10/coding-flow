# Code Plan

## 1. Objective

Add markdown-backed Decision + Tradeoff workflow to cflow.

## 2. Scope

### In Scope

- Decision parser/renderer backed by decisions.md
- cflow decision add/list/show/accept/reject/supersede
- Status lifecycle validation and filters
- Docs for decision logging guidance
- Tests for required command behavior

### Out of Scope

- Separate tradeoff entity
- tradeoffs.md
- cflow tradeoff namespace
- Problem backlink migration
- Editor-based add flow

## 3. Requirements

- Decision IDs use D-0001 format and increment from existing markdown
- Add appends without overwriting existing entries
- Updates preserve entry markdown as much as practical
- List tolerates missing file and missing fields
- Supersede validates replacement decision exists
- Tradeoffs remain a section in each decision entry with Pros and Cons

## 4. Technical Approach

- Reuse the existing problem command structure and markdown parser style
- Add a DecisionEntry model and helpers near ProblemEntry
- Use .coding/knowledge/DECISIONS.md as the source of truth alongside .coding/knowledge/PROBLEMS.md
- Add command dispatch under cflow decision
- Add focused Rust tests with temporary working directories

## 5. Files to Change

- src/main.rs
- README.md
- GEMINI.md
- AGENTS.md

## 6. Implementation Steps

- [done] Inspect existing CLI test style and constants
- [todo] Implement decision parser, renderer, commands, and dispatcher
- [todo] Add tests for required cases
- [todo] Update docs and agent guidance
- [todo] Run cargo test and CLI smoke checks

## 7. Test Plan

### Planned

- cargo test
- bin/cflow decision list in a temp/missing-file scenario if needed

### Result

- _None_

## 8. Risks

- The current problem log path uses uppercase PROBLEMS.md while the request names decisions.md; repo convention may require a matching knowledge path.

## 9. Done Criteria

### Criteria

- All requested decision commands work
- Required sections and lifecycle rules are implemented
- Filters by status, agent, and related problem work
- Tests pass
- Docs explain when to record decisions

### Verified

- _None_
