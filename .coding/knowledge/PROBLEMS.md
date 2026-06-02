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

Status: open  
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
