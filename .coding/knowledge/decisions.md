## D-0001: Store decisions beside problem knowledge log

Status: accepted
Date: 2026-06-02
Agent: codex
Related Problems: P004, P005

### Context
The repo stores durable workflow problems in .coding/knowledge/PROBLEMS.md rather than inside each packet/story folder, while the new decision workflow needs a markdown source of truth alongside the existing problem log.

### Decision
Store the decision log at .coding/knowledge/decisions.md and keep tradeoffs inside each decision entry.

### Options Considered
- Task-local decisions.md
- Knowledge-level decisions.md
- JSON decision state

### Tradeoffs
Pros:
- Matches existing durable problem-log convention
- Uses requested decisions.md filename
- Works across packet and story flows

Cons:
- Not scoped to only one task folder
- Uses markdown parsing instead of structured JSON

### Consequences
Decision commands read and update .coding/knowledge/decisions.md; future backlink work can still reference problem IDs.

### Supersedes
None

## D-0002: Fallback to CLI-rendered story plan for audit fixes

Status: accepted
Date: 2026-06-02
Agent: codex
Related Problems: P008

### Context
story agent plan failed before code edits because plan.schema.json was rejected by Codex response_format strict schema validation.

### Decision
Use bin/cflow story plan with explicit JSON for S01, and repair schema compatibility as part of the validation/template/DX story.

### Options Considered
- Retry same agent command
- Switch providers
- Use CLI-rendered manual plan

### Tradeoffs
Pros:
- Keeps workflow artifacts generated through cflow
- unblocks current story
- fixes the root schema issue in scope

Cons:
- Bypasses the intended planning subprocess for this story

### Consequences
P008 tracks the failure and S01 includes schema compatibility validation.

### Supersedes
None
