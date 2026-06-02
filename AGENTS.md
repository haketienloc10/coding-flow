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
bin/cflow problem add
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
bin/cflow problem list --status open
bin/cflow problem show P001
bin/cflow problem resolve P001 --note "..."
bin/cflow problem cancel P001 --note "..."
```

## Decision Log

Record non-trivial workflow, architecture, implementation, or fallback decisions in:

```text
.coding/knowledge/decisions.md
```

Use:

```bash
bin/cflow decision add --title "<title>" --status proposed --agent "<agent-name>"
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
request -> intake -> packet -> stories -> story loop (agent plan -> agent coding -> verify -> fix loop -> ship) -> packet verify -> packet ship
```

## Unified State Model

- `.coding/state.json` is the canonical workflow state for current task, current packet, current story, and metadata.
- `.coding/current` is a legacy compatibility pointer only; keep writing it during migration, but read `.coding/state.json` first in new command logic.
- Task commands must remain backward compatible with `--task current` and legacy `.coding/current` fallback.
- Story commands should resolve from `current_packet_id` and `current_story_id`; packet commands should resolve from `current_packet_id`.
- Do not remove `.coding/current` until a dedicated removal story updates compatibility, deprecation messaging, and tests.

### 1. Tiny Flow

Dùng cho task nhỏ hoặc story-level.
- `bin/cflow new "<task-name>"`
- `bin/cflow request --task current`
- `bin/cflow agent plan --task current`
- `bin/cflow agent coding --task current`
- `bin/cflow verify --task current`
- `bin/cflow ship --task current --dry-run`

### 2. Packet Flow

Dùng cho thay đổi trung bình/lớn/nguy cơ cao.
- `bin/cflow packet new "<title>"`
- `bin/cflow packet intake --packet current`
- `bin/cflow packet brief --packet current`
- `bin/cflow packet split --packet current`
- `bin/cflow story list`
- `bin/cflow story switch <story-id>`
- `bin/cflow story agent plan --story current`
- `bin/cflow story agent coding --story current`
- `bin/cflow story verify --story current`
- `bin/cflow story ship --story current --dry-run`
- `bin/cflow packet verify --packet current`
- `bin/cflow packet ship --packet current --dry-run`

## Story and Packet Granularity

Stories are small requirement or implementation slices.

Packets are execution or handoff bundles. A packet may contain multiple stories and should only be created intentionally.

Rules:

- Creating a story must not create a packet.
- Request planning may create stories, but must not create packets automatically.
- Use `bin/cflow packet create --stories S-0001,S-0002` to create a packet explicitly.
- Use `bin/cflow packet create --from-ready` to bundle all ready stories.
- Single-story packets require `--force`.

CLI Examples:
```bash
bin/cflow story add --title "Implement problem list filters"
bin/cflow story update S-0001 --status ready
bin/cflow story add --title "Implement decision log"
bin/cflow story update S-0002 --status ready

bin/cflow packet create --from-ready
bin/cflow packet list
bin/cflow packet show PKT-0001
```
