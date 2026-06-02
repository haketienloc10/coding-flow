# Coding Flow v0.5

Bộ workflow mỏng nhẹ cho coding task:

## Unified workflow model

`cflow` có một model trạng thái thống nhất:

- Request là input ban đầu để phân loại việc cần làm.
- Task là luồng nhỏ, độc lập, phù hợp với Tiny Flow.
- Packet là bundle thực thi hoặc handoff cho thay đổi trung bình/lớn/nguy cơ cao.
- Story là lát cắt implementation nhỏ nằm trong packet.

`.coding/state.json` là source of truth dài hạn cho current task, current packet, current story, metadata, và artifact presence. `.coding/current` chỉ còn là legacy compatibility pointer để các command hoặc script cũ vẫn resolve được `--task current` trong giai đoạn chuyển tiếp.

Chính sách migration:

- CLI tiếp tục ghi `.coding/current` khi switch task, packet, hoặc story để giữ backward compatibility.
- Các command mới nên đọc `.coding/state.json` trước, chỉ fallback sang `.coding/current` khi cần hỗ trợ task legacy.
- Không tạo packet tự động từ story hoặc request planning; packet chỉ được tạo khi gọi `packet new` hoặc `packet create`.
- Không xóa `.coding/current` cho đến khi các command task legacy và tài liệu deprecation đã có một chu kỳ migration riêng.

## Tiny Flow (Task Flow)

Dùng cho task nhỏ hoặc story-level.
```bash
bin/cflow new "rename button"
cat request.json | bin/cflow request --task current
bin/cflow agent plan --task current
bin/cflow agent coding --task current
bin/cflow verify --task current
bin/cflow ship --task current --dry-run
```

## Packet Flow

Dùng cho thay đổi trung bình/lớn/nguy cơ cao.
```bash
# Tạo packet
bin/cflow packet new "time capsule notes"

# Intake để phân tích rủi ro & phân lane
cat intake.json | bin/cflow packet intake --packet current

# Tạo brief cho packet
cat brief.json | bin/cflow packet brief --packet current

# Chia nhỏ thành các stories
cat split.json | bin/cflow packet split --packet current

# Xem danh sách stories
bin/cflow story list

# Switch sang story đầu tiên để implement
bin/cflow story switch S01-storage
bin/cflow story agent plan --story current
bin/cflow story agent coding --story current
bin/cflow story verify --story current
bin/cflow story ship --story current --dry-run

# Switch sang story tiếp theo...
# ...

# Verify và Ship toàn bộ packet
cat packet_verify.json | bin/cflow packet verify --packet current
cat packet_ship.json | bin/cflow packet ship --packet current --dry-run
```

## Story and Packet Granularity

Stories are small requirement or implementation slices.

Packets are execution or handoff bundles. A packet may contain multiple stories and should only be created intentionally.

Rules:

- Creating a story must not create a packet.
- Request planning may create stories, but must not create packets automatically.
- Use `cflow packet create --stories S-0001,S-0002` to create a packet explicitly.
- Use `cflow packet create --from-ready` to bundle all ready stories.
- Single-story packets require `--force`.

CLI Examples:
```bash
cflow story add --title "Implement problem list filters"
cflow story update S-0001 --status ready
cflow story add --title "Implement decision log"
cflow story update S-0002 --status ready

cflow packet create --from-ready
cflow packet list
cflow packet show PKT-0001
```

## Quy tắc bắt buộc (Important Rules)

- JSON chỉ là transient input.
- CLI không lưu JSON.
- Markdown trong `.coding/tasks/<task-id>/` và `.coding/packets/<packet-id>/stories/<story-id>/` là artifact chính.
- `.coding/state.json` là persistent JSON duy nhất và là canonical workflow state.
- `.coding/current` là legacy pointer, không phải source of truth mới.
- Mỗi task có folder riêng nên không ghi đè task cũ.
- Agent oneshot giúp main context không phải giữ toàn bộ plan/coding detail.
- Agent stdout phải là JSON transient; `cflow` validate rồi render markdown.
- Không lưu JSON output vào `.coding/` ngoài `.coding/state.json`.

## Agent providers

`cflow agent plan` và `cflow agent coding` hỗ trợ chọn provider:

```bash
bin/cflow agent plan --task current --provider codex
bin/cflow agent coding --task current --provider codex

bin/cflow agent plan --task current --provider claude
bin/cflow agent coding --task current --provider claude

bin/cflow agent plan --task current --provider gemini
bin/cflow agent coding --task current --provider gemini

bin/cflow agent plan --task current --provider antigravity
bin/cflow agent coding --task current --provider antigravity

bin/cflow agent plan --task current --provider custom
bin/cflow agent coding --task current --provider custom
```

Provider resolution order:

1. `--provider`
2. `CFLOW_AGENT_PROVIDER`
3. `.coding/agent.toml` `default_provider`
4. fallback `codex`

Inspect local availability:

```bash
bin/cflow agent providers
bin/cflow agent doctor --provider codex
bin/cflow agent doctor --provider antigravity
```

Optional `.coding/agent.toml`:

```toml
default_provider = "codex"

[providers.custom.plan]
cmd = "my-agent"
args = ["plan", "--json"]
prompt_mode = "stdin"

[providers.custom.coding]
cmd = "my-agent"
args = ["coding", "--json"]
prompt_mode = "arg"
```

`prompt_mode = "arg"` passes the prompt as the final argument. `prompt_mode = "stdin"` writes the prompt to child stdin.

Built-in Antigravity uses:

```toml
[providers.antigravity.plan]
cmd = "agy"
args = ["--prompt"]
prompt_mode = "arg"

[providers.antigravity.coding]
cmd = "agy"
args = ["--prompt"]
prompt_mode = "arg"
```

## Problems

`cflow` can store durable problems discovered during workflow execution.

Problems are stored in:

```text
.coding/knowledge/PROBLEMS.md
```

Add a problem:

```bash
cat <<'JSON' | bin/cflow problem add
{
  "title": "Agent output was invalid JSON",
  "severity": "medium",
  "area": "agent-plan",
  "detected_by": {
    "agent": "codex",
    "provider": "codex",
    "command": "cflow agent plan --provider codex"
  },
  "phase": "plan",
  "problem": "Agent returned prose instead of JSON.",
  "impact": "PLAN.md could not be rendered.",
  "fallback": "Retried with stricter JSON-only prompt.",
  "follow_up": "Use schema-enforced provider mode when available.",
  "links": []
}
JSON
```

List open problems:

```bash
bin/cflow problem list --status open
```

Resolve:

```bash
bin/cflow problem resolve P001 --note "Schema-enforced output added."
```

Cancel:

```bash
bin/cflow problem cancel P002 --note "No longer relevant."
```

## Decisions

`cflow` can store durable decisions that explain why an agent chose one approach over another.

Decisions are stored in:

```text
.coding/knowledge/decisions.md
```

Tradeoffs are part of each decision entry. Do not create `tradeoffs.md` or a separate `cflow tradeoff` workflow.

Add a decision:

```bash
bin/cflow decision add --title "Use markdown decision log" --status accepted --agent codex
```

Optional body flags can prefill the entry:

```bash
bin/cflow decision add \
  --title "Fallback to markdown parser" \
  --status accepted \
  --agent codex \
  --related P-0003,P-0004 \
  --context "Parser fallback was needed for markdown source of truth." \
  --decision "Use a minimal markdown parser." \
  --options "JSON source of truth,Markdown parser,Manual final report" \
  --pros "Readable by humans,Easy to diff" \
  --cons "Less queryable than JSON" \
  --consequences "CLI owns lifecycle updates."
```

List and filter:

```bash
bin/cflow decision list
bin/cflow decision list --status accepted
bin/cflow decision list --agent codex
bin/cflow decision list --related P-0003
```

Show and update lifecycle:

```bash
bin/cflow decision show D-0001
bin/cflow decision accept D-0002
bin/cflow decision reject D-0003
bin/cflow decision supersede D-0001 --by D-0005
```

Agents should record a decision when choosing between multiple implementation approaches, using an important fallback, changing workflow direction, accepting a technical tradeoff, rejecting a reasonable option, making a choice that affects later tasks, or changing direction because of a problem. Skip tiny renames, formatting, typo fixes, behavior-neutral refactors, and obvious choices without meaningful tradeoff.

## Cài đặt nhanh

Từ folder `coding-flow`:

```bash
cargo install --path .
```

Sau đó dùng được command:

```bash
cflow
```

Hoặc dùng launcher tương thích trong workspace:

```bash
./bin/cflow
```

## Task resolution

`bin/cflow` commands like `request`, `plan`, `coding`, `verify`, and `ship` can use `--task` to resolve which folder to use.
- `--task current` (default): Uses `current_task_id` from `.coding/state.json`; if that is missing, falls back to `.coding/current` for legacy compatibility.
- `--task <task-id>`: Uses `.coding/tasks/<task-id>`.
- `--task .coding/tasks/<task-id>`: Uses the absolute or relative path directly.

Story commands resolve through `current_packet_id` and `current_story_id` in `.coding/state.json`. Packet commands resolve through `current_packet_id`. `.coding/current` may still mirror these selections as `packets/<packet-id>` or `packets/<packet-id>/stories/<story-id>`, but new code should not treat that file as authoritative.

## Follow-up migration stories

Use these as the next implementation slices instead of folding the full architecture migration into one story:

- Add a current-selection compatibility shim that centralizes reads/writes for task, packet, story, and `.coding/current`.
- Normalize state repair and switch commands so `.coding/state.json` is always updated first and `.coding/current` is only mirrored after successful state persistence.
- Add deprecation messaging and docs for `.coding/current`, including the conditions for removing fallback reads.
- Add a packet-story append/update workflow so follow-up implementation stories can be created inside packet state without rerunning `packet split`.
