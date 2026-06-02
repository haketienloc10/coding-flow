## P001 - Agent output was invalid JSON

Status: resolved  
Severity: medium  
Area: agent-plan  
Detected by: codex / `cflow agent plan --provider codex`  
Phase: plan  
Detected at: 2026-06-02T12:49:43.288079035+07:00  

### Problem

Agent returned prose instead of JSON.

### Impact

PLAN.md could not be rendered.

### Fallback

Retried with stricter JSON-only prompt.

### Follow-up

Use schema-enforced provider mode when available.

### Links

- _None_

### Resolution

Schema-enforced output added.

Resolved at: 2026-06-02T12:49:53.875274657+07:00

## P002 - Old AGENTS.md note became irrelevant

Status: cancelled  
Severity: low  
Area: docs  
Detected by: codex / `manual verification`  
Phase: workflow  
Detected at: 2026-06-02T12:50:07.195646809+07:00  

### Problem

A stale workflow note was superseded by the new problem registry guidance.

### Impact

Future agents could follow outdated guidance.

### Fallback

Use the updated Problems section.

### Follow-up

Keep workflow guidance close to command behavior.

### Links

- `AGENTS.md`

### Cancellation

No longer relevant.

Cancelled at: 2026-06-02T12:50:12.291625781+07:00

### Update

Reopened for generic update verification.

Updated at: 2026-06-02T12:50:31.023213734+07:00

## P003 - Chạy agent coding tự động bị fail

Status: resolved  
Severity: high  
Area: agent_execution  
Detected by: antigravity / `cflow agent coding`  
Phase: coding  
Detected at: 2026-06-02T12:59:13.912973265+07:00  

### Problem

Lệnh `cflow agent coding` gọi đến agent bị thất bại (exit code 1) do thay đổi quá lớn, cấu trúc Rust phức tạp hoặc thiếu kết nối API/API key trong môi trường sandbox.

### Impact

Không thể chạy coding tự động bằng Agent cho các tác vụ lớn hoặc nâng cấp hệ thống, bắt buộc phải dùng manual coding flow.

### Fallback

Thực hiện Manual Fallback: viết lí do fallback vào CODING.md và tự tay sửa code trong main context.

### Follow-up

Tinh chỉnh skills/agent-coding.md để bổ sung few-shot examples, làm mượt các ràng buộc schema rỗng, và tích hợp compiler error logs vào VERIFY.md để Agent sửa code tự động.

### Links

- _None_

### Resolution

Cau hinh ke thua shell_environment_policy.inherit=all trong codex sandbox va tinh chinh skill prompts bo sung few-shot cung feedback loop logs

Resolved at: 2026-06-02T13:07:33.504452954+07:00

## P004 - Packet intake requires JSON input

Status: resolved  
Severity: low  
Area: workflow  
Detected by: codex / `bin/cflow packet intake --packet current`  
Phase: intake  
Detected at: 2026-06-02T13:24:23.184208410+07:00  

### Problem

Running packet intake without stdin or --input exits with: No input provided via stdin. Use --input or pipe JSON.

### Impact

The packet flow could not advance until an explicit intake JSON was provided.

### Fallback

Record the issue and rerun packet intake with JSON piped via stdin.

### Follow-up

Consider documenting packet intake input requirements near the packet command examples.

### Links

- _None_

### Resolution

Provided packet intake JSON via stdin and completed packet/story flow; existing README packet example already shows piped intake input.

Resolved at: 2026-06-02T13:33:37.998109832+07:00
## P005 - Smoke test used stale cflow binary

Status: resolved  
Severity: low  
Area: workflow  
Detected by: codex / `bin/cflow decision list`  
Phase: verify  
Detected at: 2026-06-02T13:30:46.445479607+07:00  

### Problem

The bin/cflow launcher preferred an existing target/debug/cflow binary that did not include the newly added decision namespace, so decision commands printed old usage output.

### Impact

CLI smoke checks failed until the debug binary was rebuilt.

### Fallback

Run cargo build before smoke testing new cflow commands through bin/cflow.

### Follow-up

Document or automate rebuilding before launcher-based smoke tests after CLI command changes.

### Links

- _None_

### Resolution

Actual stale launcher target was target/release/cflow; rebuilt release binary before rerunning decision CLI smoke tests.

Resolved at: 2026-06-02T13:31:21.015732888+07:00

## P006 - Implementation skipped story agent coding

Status: open  
Severity: medium  
Area: workflow  
Detected by: codex / `manual review after packet/story flow`  
Phase: coding  
Detected at: 2026-06-02T14:49:08.978862304+07:00  

### Problem

The implementation was done directly in the main context and then recorded with cflow story coding, instead of running cflow story agent coding --story current as expected by the repo workflow guidance.

### Impact

The story flow artifacts exist, but the implementation bypassed the intended agent coding subprocess boundary, reducing workflow fidelity and making the story coding artifact less representative of actual execution.

### Fallback

Recorded the issue in the durable problem log and kept verification evidence from cargo fmt --check, cargo test, cargo build --release, and CLI smoke tests.

### Follow-up

For future implementation stories, run cflow story agent coding --story current for the coding step, or explicitly record a decision/problem before using main-context implementation as a fallback.

### Links

- `.coding/packets/20260602-132403-add-decision-tradeoff-workflow/stories/S01-decision-workflow/CODING.md`

### Update

Repeated for S03-main-module-extraction-plan: implementation used story plan/coding in the main context instead of story agent plan/coding. Future stories must attempt story agent coding first and record fallback before manual implementation.

Updated at: 2026-06-02T16:41:40.373636622+07:00
## P007 - cflow command is not on PATH

Status: open  
Severity: low  
Area: workflow  
Detected by: codex / `cflow --help`  
Phase: intake  
Detected at: 2026-06-02T16:06:28.278380195+07:00  

### Problem

Running cflow from the repo shell failed with command not found.

### Impact

Workflow commands require an explicit bin/cflow fallback, which can confuse agents and repeat in future sessions.

### Fallback

Used the repo launcher bin/cflow for all workflow commands.

### Follow-up

Document the launcher expectation or add a PATH/bootstrap step for local agent sessions.

### Links

- `bin/cflow`

## P008 - Agent plan rejected plan schema

Status: resolved  
Severity: medium  
Area: agent-plan  
Detected by: codex / `bin/cflow story agent plan --story current`  
Phase: plan  
Detected at: 2026-06-02T16:08:09.125489396+07:00  

### Problem

The Codex response_format request rejected schemas/plan.schema.json because nested object properties such as test_plan do not declare a required array containing every property.

### Impact

The story agent plan subprocess cannot generate PLAN.md, blocking the intended story flow before implementation.

### Fallback

Use bin/cflow story plan with manually prepared JSON for this story, then include schema compatibility repair in the current validation story.

### Follow-up

Normalize nested object required declarations in schemas used with provider response_format.

### Links

- `schemas/plan.schema.json`

### Resolution

Updated plan.schema.json to require all declared object fields used by response_format, including test_plan.planned/result and root optional arrays.

Resolved at: 2026-06-02T16:14:53.249551933+07:00
## P009 - Agent coding rejected coding schema allOf

Status: resolved  
Severity: medium  
Area: agent-coding  
Detected by: codex / `bin/cflow story agent coding --story current`  
Phase: coding  
Detected at: 2026-06-02T16:08:52.510808040+07:00  

### Problem

The Codex response_format request rejected schemas/coding.schema.json because top-level allOf is not permitted.

### Impact

The story agent coding subprocess cannot run for S01, so implementation cannot proceed through the intended subprocess boundary until the schema is normalized.

### Fallback

Use the accepted D-0002 fallback to implement S01 in the main context and then record CODING.md through bin/cflow story coding.

### Follow-up

Remove unsupported allOf from coding.schema.json while preserving the ready_for_verify implies next=verify validation in local validation logic if needed.

### Links

- `schemas/coding.schema.json`
- `.coding/knowledge/decisions.md`

### Resolution

Updated coding.schema.json with draft 2020-12 metadata and removed provider-rejected allOf; Rust validate_coding still enforces ready_for_verify -> verify.

Resolved at: 2026-06-02T16:14:53.247139388+07:00
## P010 - Cargo tests mutate workflow state

Status: resolved  
Severity: medium  
Area: test  
Detected by: codex / `cargo test`  
Phase: verify  
Detected at: 2026-06-02T16:12:23.563357752+07:00  

### Problem

Running cargo test changed .coding/state.json and .coding/current, and created .coding/packets/PKT-0001 from packet creation tests.

### Impact

The active story context was lost after tests, causing bin/cflow story status to report no current story selected.

### Fallback

Restore the intended current packet/story after tests and remove the untracked test packet artifact generated by this run.

### Follow-up

Isolate command_packet_create tests from repository .coding state or inject a temp state root for tests.

### Links

- `src/main.rs`
- `.coding/state.json`

### Resolution

Added test workflow state guard for packet-create tests and confirmed cargo test no longer leaves .coding/packets/PKT-0001 or changes current story.

Resolved at: 2026-06-02T16:14:53.420272648+07:00

## P011 - Story ship requires JSON input

Status: open  
Severity: low  
Area: workflow  
Detected by: codex / `bin/cflow story ship --story current --dry-run`  
Phase: ship  
Detected at: 2026-06-02T16:15:09.395232342+07:00  

### Problem

Running story ship dry-run without stdin or --input exits with SHIP.md is missing; provide ship JSON via stdin or --input.

### Impact

The story flow cannot complete from the obvious dry-run command unless the caller knows to pipe ship JSON.

### Fallback

Rerun story ship with explicit ship JSON piped on stdin.

### Follow-up

Document story ship input requirements next to story flow examples or improve the error with an example.

### Links

- `README.md`

## P012 - Ship commit leaves post-commit state dirty

Status: open  
Severity: medium  
Area: ship  
Detected by: codex / `bin/cflow story ship --story current --commit`  
Phase: ship  
Detected at: 2026-06-02T16:17:15.000380049+07:00  

### Problem

The ship command runs git add/commit before updating .coding/state.json to committed, leaving state.json modified immediately after a successful commit.

### Impact

A shipped story can leave the worktree dirty and the committed state can still show commit_pending instead of committed.

### Fallback

Amend the generated commit with the post-commit state update and problem record.

### Follow-up

Move ship state updates before git add/commit or stage state.json again before committing.

### Links

- `src/main.rs`
- `.coding/state.json`

## P013 - Story status cannot resolve current packet stories

Status: open  
Severity: low  
Area: workflow  
Detected by: codex / gpt-5 / `./bin/cflow story status --story current`  
Phase: workflow  
Detected at: 2026-06-02T16:21:02.920284915+07:00  

### Problem

The story status command returned `stories.md not found` while `packet status --packet current` could read the current packet and list its stories.

### Impact

Agents cannot rely on `story status --story current` to inspect the active story and must use packet status or state inspection as a fallback.

### Fallback

Used `./bin/cflow packet status --packet current` and `.coding/state.json` inspection to determine current workflow state.

### Follow-up

Fix story status path resolution for the current packet/story context or document the correct command form.

### Links

- `src/main.rs`
- `.coding/state.json`

## P014 - story switch reads wrong story index path

Status: resolved  
Severity: medium  
Area: story workflow state  
Detected by: codex / gpt-5 / `./bin/cflow story switch S02-state-consistency-repair`  
Phase: workflow  
Detected at: 2026-06-02T16:22:50.468149793+07:00  

### Problem

story switch failed with stories.md not found at the previous story directory instead of using the packet root STORIES.md or state packet story metadata.

### Impact

Agents cannot switch to an existing packet story through the documented story flow, so current_story_id remains stale and story commands fail.

### Fallback

Read packet/story artifacts directly and keep implementation scoped to S02 until the CLI repair is implemented.

### Follow-up

Fix story resolution and state repair so packet-root stories are discoverable consistently.

### Links

- `.coding/packets/20260602-160700-workflow-audit-priority-fixes/stories/S02-state-consistency-repair/STORY.md`

### Resolution

Fixed in S02 by resolving story switch/list/status from packet state and extending state repair for packet stories.

Resolved at: 2026-06-02T16:31:38.688764421+07:00

## P015 - Intake command without JSON blocks plan-only requests

Status: open  
Severity: low  
Area: workflow/intake  
Detected by: codex / gpt-5 / `bin/cflow request --task current`  
Phase: intake  
Detected at: 2026-06-02T20:02:33.236828316+07:00  

### Problem

Running intake via `bin/cflow request --task current` for a plan-only conversation fails because the command requires JSON via stdin or --input.

### Impact

Agent cannot satisfy the run-intake-before-planning rule through the normal command path for conversational plan-only requests without creating a separate JSON artifact or ad hoc payload.

### Fallback

Performed manual intake by inspecting repository structure and src/main.rs before producing the plan.

### Follow-up

Consider adding an interactive/no-input intake mode or documented minimal JSON invocation for plan-only requests.

### Links

- `src/main.rs`
- `AGENTS.md`

## P016 - Invalid git status ignored-mode invocation

Status: open  
Severity: low  
Area: agent/verification  
Detected by: codex / gpt-5 / `git status --short --ignored=.coding/knowledge/PROBLEMS.md`  
Phase: verify  
Detected at: 2026-06-02T20:02:53.444556702+07:00  

### Problem

Agent attempted to pass a path to git status --ignored mode, which expects a mode value such as traditional, matching, or no.

### Impact

The intended worktree status check failed and had to be rerun with a valid command.

### Fallback

Use plain `git status --short` for status checks unless an ignored-mode value is explicitly needed.

### Follow-up

Avoid combining --ignored mode syntax with path filtering; use standard pathspec position after options if needed.

### Links

- _None_

## P017 - Story plan JSON payload failed before schema-correct retry

Status: open  
Severity: low  
Area: workflow/story-plan  
Detected by: codex / gpt-5 / `bin/cflow story plan --story current`  
Phase: plan  
Detected at: 2026-06-02T20:10:02.208180522+07:00  

### Problem

The initial story plan command failed first with empty stdin from a combined switch-and-pipe shell command, then with a payload that did not satisfy the current plan schema shape.

### Impact

PLAN.md creation required retrying with a separately piped JSON payload using done_criteria.items and object-shaped implementation_steps.

### Fallback

Ran story switch separately and retried story plan with schema-compatible JSON stdin.

### Follow-up

Prefer separate shell invocations for state-changing workflow commands followed by piped JSON commands; keep a small valid plan JSON fixture handy.

### Links

- `.coding/packets/20260602-200549-main-module-extraction-rollout/stories/S01-lib-boundary/PLAN.md`
