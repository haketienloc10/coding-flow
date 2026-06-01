# Coding Flow v0.3

Bộ workflow mỏng nhẹ cho coding task:

```text
request -> plan -> verify -> ship(commit)
```

Nguyên tắc chính:

```text
LLM = phân tích + sinh JSON
CLI = validate + render markdown + ship/commit
```

LLM không sửa trực tiếp markdown artifact. LLM chỉ tạo JSON input tương ứng từng phase và truyền qua stdin.

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

## Flow dùng thực tế

Always create/select a task first:

```bash
./bin/cflow new "focus garden"
```

### 1. Request intake

```bash
cat <<'JSON' | ./bin/cflow request --task current
{ ... }
JSON
```

### 2. Plan

```bash
cat <<'JSON' | ./bin/cflow plan --task current
{ ... }
JSON
```

### 3. Verify

```bash
cat <<'JSON' | ./bin/cflow verify --task current
{ ... }
JSON
```

### 4. Ship

Dry-run:

```bash
cat <<'JSON' | ./bin/cflow ship --task current --dry-run
{ ... }
JSON
```

Commit:

```bash
cat <<'JSON' | ./bin/cflow ship --task current --commit
{ ... }
JSON
```

## Quy tắc bắt buộc (Important Rules)

- JSON chỉ là transient input.
- CLI không lưu JSON.
- Markdown trong .coding/tasks/<task-id>/ là artifact chính.
- Mỗi task có folder riêng nên không ghi đè task cũ.

## Task resolution

`cflow` commands like `request`, `plan`, `verify`, and `ship` can use `--task` to resolve which folder to use.
- `--task current` (default): Uses the task specified in `.coding/current`.
- `--task <task-id>`: Uses `.coding/tasks/<task-id>`.
- `--task .coding/tasks/<task-id>`: Uses the absolute or relative path directly.
