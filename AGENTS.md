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
