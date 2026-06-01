# Coding Flow v0

Bộ workflow mỏng nhẹ cho coding task nhỏ:

```text
request -> plan -> verify -> ship(commit)
```

Nguyên tắc chính:

```text
LLM = phân tích + sinh JSON
CLI = validate + render markdown + ship/commit
```

LLM không sửa trực tiếp markdown artifact. LLM chỉ tạo JSON input tương ứng từng phase.

---

## Cấu trúc

```text
coding-flow-v0/
├── Cargo.lock
├── Cargo.toml
├── bin/
│   └── cflow
├── src/
│   └── main.rs
├── skills/
│   ├── request.md
│   ├── plan.md
│   ├── verify.md
│   └── ship.md
├── templates/
│   ├── request.md
│   ├── plan.md
│   ├── verify.md
│   └── ship.md
├── schemas/
│   ├── request.schema.json
│   ├── plan.schema.json
│   ├── verify.schema.json
│   └── ship.schema.json
├── examples/
│   ├── request.json
│   ├── plan.json
│   ├── verify.json
│   └── ship.json
└── package.json
```

Khi chạy trong repo thật, CLI sẽ tạo:

```text
.coding/
├── REQUEST.md
├── PLAN.md
├── VERIFY.md
└── SHIP.md
```

---

## Cài đặt nhanh

Từ folder `coding-flow-v0`:

```bash
cargo install --path .
```

Sau đó dùng được command:

```bash
cflow
```

Không muốn install thì chạy trực tiếp:

```bash
cargo run --bin cflow --
```

Hoặc dùng launcher tương thích trong workspace:

```bash
./bin/cflow
```

---

## Flow dùng thực tế

### 1. Request intake

LLM sinh `.coding/request.json`, sau đó CLI render markdown:

```bash
cflow request --input .coding/request.json
```

Hoặc thử bằng example:

```bash
cflow request --input examples/request.json
```

### 2. Plan

LLM sinh `.coding/plan.json`:

```bash
cflow plan --input .coding/plan.json
```

Hoặc thử:

```bash
cflow plan --input examples/plan.json
```

### 3. Verify

Sau implement, LLM/dev sinh `.coding/verify.json`:

```bash
cflow verify --input .coding/verify.json
```

Hoặc thử:

```bash
cflow verify --input examples/verify.json
```

### 4. Ship

LLM/dev sinh `.coding/ship.json`, rồi dry-run:

```bash
cflow ship --input .coding/ship.json --dry-run
```

Commit thật:

```bash
cflow ship --input .coding/ship.json --commit
```

---

## Quy tắc v0

### Request

`request.json` quyết định request có cần plan không.

Allowed `type`:

```text
question
unclear
investigation
new_feature
bug_fix
refactor
maintenance
documentation
test_only
```

Allowed `lane`:

```text
none
needs_clarification
tiny
normal
high_risk
```

Allowed `next_action`:

```text
answer
clarify
investigate
plan
none
```

### Plan

`plan.json` giữ đúng 9 mục:

1. Objective
2. Scope
3. Requirements
4. Technical Approach
5. Files to Change
6. Implementation Steps
7. Test Plan
8. Risks
9. Done Criteria

Bắt buộc có từ đầu:

```text
objective
scope
requirements
technical_approach
done_criteria.items
```

### Verify

`verify.status` phải là `passed` thì mới ship được.

Allowed `verify.status`:

```text
passed
failed
partial
skipped
```

### Ship

`cflow ship --commit` sẽ reject nếu:

- `ship.ready != true`
- `verification.status != passed`

---

## Hướng phát triển v1

| Version | Hướng nâng cấp |
|---|---|
| v0.1 | Validate schema chặt bằng `ajv` |
| v0.2 | Thêm `cflow status` |
| v0.3 | Auto lấy changed files bằng `git diff --name-only` |
| v0.4 | Block ship nếu `known_issues` không rỗng |
| v0.5 | Link `done_criteria.items` với `verify.done_criteria_verified` |
| v1.0 | Hỗ trợ nhiều task song song bằng task id |
