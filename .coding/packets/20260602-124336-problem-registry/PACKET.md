# Packet

## Goal

Add markdown-backed problem recording to cflow with add/list/show/status update commands and concise workflow documentation.

## Scope

### In Scope

- Create .coding/knowledge/PROBLEMS.md automatically when adding problems
- Add cflow problem add/list/open/resolved/cancelled/show/resolve/cancel/update commands
- Validate allowed status, severity, and phase values
- Parse problem entries line-by-line for listing and showing
- Update AGENTS.md and README.md with problem usage guidance

### Out of Scope

- Persistent problem JSON storage
- Database or complex markdown parser
- Optional state.json problem counters unless trivial
- Large workflow refactors beyond problem commands

## Global Acceptance Criteria

- cflow problem add reads transient JSON from stdin and appends rendered markdown with generated P### ids
- list/open/resolved/cancelled/show commands work from PROBLEMS.md
- resolve/cancel/update mutate status and append resolution/cancellation/reopen notes as appropriate
- Invalid statuses fail clearly
- README.md and AGENTS.md document problem recording rules

## Technical Constraints

- Keep implementation small in src/main.rs
- Do not store input JSON anywhere
- Only persistent JSON remains .coding/state.json
- Use simple line-based markdown parsing

## Shared Data / Contracts

- Problem JSON input excludes id and detected_at
- Problem markdown entries start with ## P001 - Title
- Allowed status values: open, resolved, cancelled
- Allowed severity values: low, medium, high, blocking
- Allowed phase values: request, intake, plan, coding, coding_fix, verify, ship, state, agent, workflow, unknown

## Validation Strategy

- cargo check
- Manual add/list/show/resolve/cancel/invalid-status commands from requested tests
- Inspect PROBLEMS.md for expected markdown sections
