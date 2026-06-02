# cflow Agent Rules

- Do not treat every request as one long coding task.
- Run intake before planning.
- Use packet flow when request is normal/high_risk or split_required.
- Use story flow for implementation.
- Do not implement more than the current story.
- If verify fails, do not fix in main context.
- Use story coding --fix.
- Do not edit .coding markdown artifacts directly.
- Do not create persistent phase JSON files.
- The only persistent JSON is .coding/state.json.

## Problems

Record durable workflow or agent problems in:

```text
.coding/knowledge/PROBLEMS.md
```

Use:

```bash
cflow problem add
```

Record a problem when an agent command fails, JSON output is invalid, fallback is required, workflow routing is wrong, or the issue is likely to repeat.

A problem entry must include:

* status
* severity
* area
* detected agent/provider/command
* phase
* problem
* impact
* fallback
* follow-up
* links

Do not store long logs, full diffs, or noisy terminal output.

Use:

```bash
cflow problem list --status open
cflow problem show P001
cflow problem resolve P001 --note "..."
cflow problem cancel P001 --note "..."
```

## Decision Log

Record non-trivial workflow, architecture, implementation, or fallback decisions in:

```text
.coding/knowledge/decisions.md
```

Use:

```bash
cflow decision add --title "<title>" --status proposed --agent "<agent-name>"
```

Record a decision when the agent:

* chooses between multiple implementation approaches
* uses an important fallback
* changes workflow direction
* accepts a technical tradeoff
* rejects a reasonable option
* makes a choice that affects later tasks
* changes direction because of a problem

Do not record decisions for small renames, formatting, typo fixes, behavior-neutral refactors, or obvious choices without meaningful tradeoff.

A decision must capture context, decision, options considered, tradeoffs, consequences, and related problems if any. Tradeoffs belong inside the decision entry. Do not create a separate tradeoff log.

## Core Flow

```text
request -> intake -> packet -> stories -> story loop (plan -> coding -> verify -> fix loop -> ship) -> packet verify -> packet ship
```

### 1. Tiny Flow

Dùng cho task nhỏ hoặc story-level.
- `cflow new "<task-name>"`
- `cflow request --task current`
- `cflow agent plan --task current`
- `cflow agent coding --task current`
- `cflow verify --task current`
- `cflow ship --task current --dry-run`

### 2. Packet Flow

Dùng cho thay đổi trung bình/lớn/nguy cơ cao.
- `cflow packet new "<title>"`
- `cflow packet intake --packet current`
- `cflow packet brief --packet current`
- `cflow packet split --packet current`
- `cflow story list`
- `cflow story switch <story-id>`
- `cflow story agent plan --story current`
- `cflow story agent coding --story current`
- `cflow story verify --story current`
- `cflow story ship --story current --dry-run`
- `cflow packet verify --packet current`
- `cflow packet ship --packet current --dry-run`
