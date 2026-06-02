# Coding Flow v0.4

Bộ workflow mỏng nhẹ cho coding task:

## Recommended flow

```bash
bin/cflow new "focus garden"

cat <<'JSON' | bin/cflow request --task current
{
  "summary": "Build Focus Garden MVP",
  "type": "new_feature",
  "planning_needed": true,
  "lane": "normal",
  "risk_flags": [],
  "hard_gates": [],
  "assumptions": [],
  "clarifying_questions": [],
  "next_action": "plan"
}
JSON

bin/cflow agent plan --task current
bin/cflow agent coding --task current

cat <<'JSON' | bin/cflow verify --task current
{
  "status": "passed",
  "checks": [],
  "manual_checks": ["Start/cancel/complete/reload tested"],
  "regressions_checked": [],
  "known_issues": [],
  "done_criteria_verified": ["Garden persists after reload"]
}
JSON

cat <<'JSON' | bin/cflow ship --task current --dry-run
{
  "ready": true,
  "commit": {
    "type": "feat",
    "scope": "focus-garden",
    "message": "add focus garden mvp",
    "body": []
  },
  "changed_files": [],
  "summary": ["Added focus session timer and garden history"],
  "verification": {
    "status": "passed",
    "source": ".coding/tasks/current/VERIFY.md"
  },
  "notes": []
}
JSON
```

## Manual fallback flow

```bash
bin/cflow new "focus garden"

cat request.json | bin/cflow request --task current
cat plan.json | bin/cflow plan --task current
cat coding.json | bin/cflow coding --task current
cat verify.json | bin/cflow verify --task current
cat ship.json | bin/cflow ship --task current --dry-run
```

## Quy tắc bắt buộc (Important Rules)

- JSON chỉ là transient input.
- CLI không lưu JSON.
- Markdown trong `.coding/tasks/<task-id>/` là artifact chính.
- Mỗi task có folder riêng nên không ghi đè task cũ.
- Agent oneshot giúp main context không phải giữ toàn bộ plan/coding detail.

## Cài đặt nhanh

Từ folder `coding-flow-v0`:

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

bin/cflow` commands like `request`, `plan`, `coding`, `verify`, and `ship` can use `--task` to resolve which folder to use.
- `--task current` (default): Uses the task specified in `.coding/current`.
- `--task <task-id>`: Uses `.coding/tasks/<task-id>`.
- `--task .coding/tasks/<task-id>`: Uses the absolute or relative path directly.
