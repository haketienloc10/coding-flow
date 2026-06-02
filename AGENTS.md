# cflow Agent Rules

- Do not treat every request as one long coding task.
- For Tiny Flow, run `request` before planning.
- For Packet Flow, run `packet intake` before packet brief/split/story planning.
- Use packet flow when request is normal/high_risk or split_required.
- Use story flow for implementation.
- Do not implement more than the current story.
- If verify fails, do not fix in main context.
- Use `bin/cflow story agent coding --story current --fix`.
- Do not edit workflow markdown artifacts in `.coding` directly; use `bin/cflow ...` commands to create/update them.
- Do not create persistent phase JSON files.
- The only persistent JSON is .coding/state.json.

## Problems

Record durable workflow or agent problems in:

```text
.coding/knowledge/PROBLEMS.md
```

Use:

```bash
cat problem.json | bin/cflow problem add
```

Record a problem when an agent command fails, JSON output is invalid, fallback is required, workflow routing is wrong, or the issue is likely to repeat.

A problem JSON input must include:

* status
* severity
* area
* detected_by.agent
* detected_by.provider
* detected_by.command
* phase
* problem
* impact
* fallback
* follow_up
* links

Do not store long logs, full diffs, or noisy terminal output.

Example:

```json
{
  "status": "open",
  "title": "Agent output was invalid JSON",
  "severity": "medium",
  "area": "agent-plan",
  "detected_by": {
    "agent": "codex",
    "provider": "codex",
    "command": "bin/cflow story agent plan --story current"
  },
  "phase": "plan",
  "problem": "Agent returned prose instead of JSON.",
  "impact": "PLAN.md could not be rendered.",
  "fallback": "Retried with stricter JSON-only prompt.",
  "follow_up": "Use schema-enforced provider mode when available.",
  "links": []
}
```

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
bin/cflow decision add \
  --title "<title>" \
  --status proposed \
  --agent "<agent-name>" \
  --related "P001" \
  --context "<why this decision is needed>" \
  --decision "<chosen approach>" \
  --options "<option A>,<option B>" \
  --pros "<main benefits>" \
  --cons "<main costs>" \
  --consequences "<what this changes later>"
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
tiny: request -> agent plan -> agent coding -> verify -> fix loop -> ship
packet: packet new -> request -> packet intake -> packet brief -> packet split -> story loop (agent plan -> agent coding -> verify -> fix loop -> ship) -> packet verify -> packet ship
```

Phase commands that are not `agent ...` require JSON via stdin heredoc using the matching file in `skills/`. Do not create persistent phase JSON files.

## Unified State Model

- `.coding/state.json` is the canonical workflow state for current task, current packet, current story, and metadata.
- `.coding/current` is a legacy compatibility pointer only; keep writing it during migration, but read `.coding/state.json` first in new command logic.
- Task commands must remain backward compatible with `--task current` and legacy `.coding/current` fallback.
- Story commands should resolve from `current_packet_id` and `current_story_id`; packet commands should resolve from `current_packet_id`.
- Do not remove `.coding/current` until a dedicated removal story updates compatibility, deprecation messaging, and tests.

### 1. Tiny Flow

Dùng cho task nhỏ độc lập. Tiny Flow dùng `request`; không chạy `packet intake`.
- `bin/cflow new "<task-name>"`
- Run request using `skills/request.md`.
- `bin/cflow agent plan --task current`
- `bin/cflow agent coding --task current`
- Run verify using `skills/verify.md`.
- Run ship using `skills/ship.md`.

Use stdin heredoc only for phase JSON.

### 2. Packet Flow

Dùng cho thay đổi trung bình/lớn/nguy cơ cao. Packet Flow tạo `REQUEST.md` trong current packet trước `packet intake`.
- `bin/cflow packet new "<title>"`
- Run request using `skills/request.md`.
- Run packet intake using `skills/intake.md`.
- Run packet brief using `skills/packet-brief.md`.
- Run packet split using `skills/packet-split.md`.
- `bin/cflow story list`
- `bin/cflow story switch <story-id>`
- `bin/cflow story agent plan --story current`
- `bin/cflow story agent coding --story current`
- Run story verify using `skills/verify.md`.
- Run story ship using `skills/ship.md`.
- Run packet verify using `skills/packet-verify.md`.
- Run packet ship using `skills/packet-ship.md`.

Use stdin heredoc only for phase JSON.

## Story and Packet Granularity

Stories are small requirement or implementation slices.

Packets are execution or handoff bundles. A packet may contain multiple stories and should only be created intentionally.

Preferred packet flow for new normal/high_risk work:

- `bin/cflow packet new "<title>"`
- Run request using `skills/request.md`.
- Run packet intake using `skills/intake.md`.
- Run packet brief using `skills/packet-brief.md`.
- Run packet split using `skills/packet-split.md`.

Legacy task-bundling flow:

- Use `story add`, `story update`, and `packet create` only when stories already exist in a task-level `stories.md`.
- Do not use legacy task-bundling commands inside an active packet created by `packet new`.

Rules:

- Creating a story must not create a packet.
- Request planning may create stories, but must not create packets automatically.
- In legacy task-bundling flow, use `bin/cflow packet create --stories S-0001,S-0002` to create a packet explicitly.
- In legacy task-bundling flow, use `bin/cflow packet create --from-ready` to bundle all ready stories.
- In legacy task-bundling flow, single-story packets require `--force`.

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
