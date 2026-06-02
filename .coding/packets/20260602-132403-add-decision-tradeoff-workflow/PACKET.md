# Packet

## Goal

Implement cflow decision workflow with markdown-backed decisions.md, lifecycle commands, filters, tests, and docs.

## Scope

### In Scope

- Decision markdown parser and renderer
- cflow decision add/list/show/accept/reject/supersede
- Lifecycle validation
- Related problem filtering
- README and agent guidance updates
- Rust tests for required cases

### Out of Scope

- Separate tradeoff entity or command
- JSON source of truth
- Problem backlink migration
- Complex editor flow

## Global Acceptance Criteria

- decision add creates decisions.md and increments D-xxxx IDs
- list/show/status transition commands work
- status/agent/related filters work
- tradeoffs are embedded inside decision entries
- no tradeoffs.md or cflow tradeoff command is added
- tests pass
- docs explain when agents should record decisions

## Technical Constraints

- Preserve existing problems.md workflow
- Preserve markdown body formatting when updating entries
- Use existing Rust CLI patterns

## Shared Data / Contracts

- Decision IDs use D-0001 format
- Decision entries are separated by ## D-xxxx: headings
- Required sections include Context, Decision, Options Considered, Tradeoffs, Consequences, Supersedes

## Validation Strategy

- cargo test
- targeted CLI smoke commands if needed
