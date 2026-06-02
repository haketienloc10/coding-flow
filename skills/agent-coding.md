---
name: cflow:agent-coding
description: Implement code from PLAN.md or fix VERIFY.md findings using a oneshot agent and return a coding summary JSON.
argument-hint: "[--task current] [--fix]"
allowed-tools:
  - Read
  - Edit
  - MultiEdit
  - Bash
requires:
  - cflow
---

# cflow:agent-coding

## Purpose

Implement the current task or story according to PLAN.md, or fix only findings from VERIFY.md when `--fix` is used.

## Initial Mode

Used by:

```bash
cflow agent coding --task current
cflow story agent coding --story current
```

Rules:

- Read PLAN.md (and STORY.md, PACKET.md in story mode).
- Implement the code changes described in the plan.
- Do not edit REQUEST.md, STORY.md, PACKET.md.
- Do not edit PLAN.md.
- Do not edit CODING.md.
- Do not edit VERIFY.md.
- Do not edit SHIP.md.
- Do not create JSON files.
- Do not run verify.
- Do not run ship.
- Do not commit.
- Return valid JSON only.
- cflow will validate and render CODING.md.
- Verification is handled by the verify phase.
- Return `mode: "initial"`.

## Fix Mode

Used by:

```bash
cflow agent coding --task current --fix
cflow story agent coding --story current --fix
```

Rules:

- Read PLAN.md (and STORY.md, PACKET.md in story mode).
- Read VERIFY.md.
- Read CODING.md if present.
- Fix only unresolved findings from VERIFY.md.
- Do not expand scope.
- Do not rewrite unrelated code.
- Do not re-plan.
- Do not edit REQUEST.md, STORY.md, PACKET.md.
- Do not edit PLAN.md.
- Do not edit CODING.md.
- Do not edit VERIFY.md.
- Do not edit SHIP.md.
- Do not create JSON files.
- Do not verify.
- Do not run ship.
- Do not commit.
- Return valid JSON only.
- cflow will validate and render CODING.md.
- Return `mode: "fix"`.
- Put fixed finding ids into `fixed_findings`.

## Output JSON

Bạn PHẢI trả về JSON khớp chính xác với schema định nghĩa, KHÔNG ĐƯỢC chứa thêm bất kỳ trường thừa nào (additionalProperties: false), và KHÔNG ĐƯỢC bỏ sót bất kỳ trường bắt buộc nào (ngay cả khi mảng rỗng, bạn phải điền `[]` chứ không được bỏ qua hoặc gán `null`).

### Schema bắt buộc:

```json
{
  "mode": "initial" | "fix",
  "status": "ready_for_verify" | "blocked" | "partial" | "failed",
  "summary": ["danh sách các hành động đã thực hiện"],
  "fixed_findings": ["danh sách id các findings đã được fix, chỉ dùng cho mode: fix"],
  "changed_files": ["danh sách các file đã chỉnh sửa"],
  "notes": ["các ghi chú thêm nếu có"],
  "next": "verify" | "plan" | "clarify" | "none"
}
```

### Ràng buộc bổ sung:
- Nếu `"status"` là `"ready_for_verify"`, bắt buộc `"next"` phải là `"verify"`.

---

## Few-shot Examples

### Ví dụ 1: Chế độ Initial (Initial Mode) thành công
```json
{
  "mode": "initial",
  "status": "ready_for_verify",
  "summary": [
    "Đã tách hàm main.rs thành module commands.rs và agent.rs",
    "Cấu hình kế thừa biến môi trường cho codex sandbox"
  ],
  "fixed_findings": [],
  "changed_files": [
    "src/main.rs",
    "src/commands.rs",
    "src/agent.rs"
  ],
  "notes": [
    "Hãy chắc chắn chạy 'cargo check' trước khi verify"
  ],
  "next": "verify"
}
```

### Ví dụ 2: Chế độ Fix (Fix Mode) sau khi sửa lỗi thành công
```json
{
  "mode": "fix",
  "status": "ready_for_verify",
  "summary": [
    "Đã sửa lỗi cú pháp do thiếu dấu ngoặc đóng ở src/commands.rs dòng 42"
  ],
  "fixed_findings": [
    "compiler_error_01"
  ],
  "changed_files": [
    "src/commands.rs"
  ],
  "notes": [],
  "next": "verify"
}
```

### Ví dụ 3: Bị block (Blocked) do thiếu API key hoặc lỗi nghiêm trọng
```json
{
  "mode": "initial",
  "status": "blocked",
  "summary": [],
  "fixed_findings": [],
  "changed_files": [],
  "notes": [
    "Không thể thực thi lệnh vì thiếu biến môi trường GEMINI_API_KEY ở môi trường chạy"
  ],
  "next": "clarify"
}
```
