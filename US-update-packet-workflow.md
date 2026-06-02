Bạn đang sửa repo `coding-flow`.

Mục tiêu: nâng cấp `cflow` để hỗ trợ workflow lớn hơn task đơn, nhưng **không dùng khái niệm project** vì dễ nhầm với repository/product project.

Dùng terminology chính thức:

```text
request -> intake -> packet -> stories -> story loop -> packet verify -> packet ship
```

## 0. Nguyên tắc bắt buộc

Không được tự đổi vocabulary.

Bắt buộc dùng:

* `request`: yêu cầu gốc của user
* `intake`: bước phân loại yêu cầu
* `packet`: một gói công việc bounded được tạo từ request
* `story`: lát implementation nhỏ bên trong packet
* `lane`: mức xử lý `tiny`, `normal`, `high_risk`

Không dùng:

* `project`
* `initiative`
* `epic`

Trừ khi chỉ nhắc trong docs để nói “không dùng các từ này”.

---

# 1. Flow cần implement

## Flow tổng quát

```text
REQUEST
  ↓
INTAKE
  ↓
Lane?
  ├─ tiny
  │    └─ task flow:
  │        plan -> coding -> verify -> fix loop -> ship
  │
  ├─ normal
  │    └─ packet flow:
  │        PACKET.md -> STORIES.md
  │        each story:
  │          STORY.md -> PLAN.md -> CODING.md -> VERIFY.md
  │          failed? -> agent coding --fix -> VERIFY.md lại
  │          passed? -> SHIP.md
  │        packet verify -> packet ship
  │
  └─ high_risk
       └─ packet flow bắt buộc split stories
          stronger validation
          human confirmation nếu ambiguous
```

Flow task cũ không bị bỏ. Nó trở thành shortcut cho `tiny` hoặc story-level flow.

---

# 2. Intake model

Thêm hoặc cập nhật intake để classify request.

Intake output cần có các field logic sau:

```json
{
  "request_summary": "",
  "input_type": "new_spec",
  "lane": "normal",
  "risk_flags": [],
  "hard_gates": [],
  "split_required": true,
  "reason": "",
  "next_action": "packet_split",
  "assumptions": [],
  "clarifying_questions": []
}
```

## Allowed `input_type`

```text
new_spec
spec_slice
change_request
new_feature
bug_fix
refactor
maintenance
workflow_improvement
documentation
test_only
question
unclear
```

## Allowed `lane`

```text
tiny
normal
high_risk
needs_clarification
none
```

## Allowed `next_action`

```text
answer_directly
clarify
task_flow
packet_brief
packet_split
story_flow
none
```

---

# 3. Risk model

Implement risk model theo rule sau.

## Risk flags

```text
auth
authorization
data_model
audit_security
external_system
public_contract
cross_platform
existing_behavior
weak_proof
multi_domain
```

Ý nghĩa:

* `auth`: login, logout, session, JWT, OAuth, password, refresh token
* `authorization`: role, permission, tenant/company/workspace scope
* `data_model`: schema, migration, uniqueness, deletion, retention, backfill
* `audit_security`: audit log, privacy, sensitive data, access log, encryption
* `external_system`: payment, email, cloud SDK, queue, webhook, third-party API
* `public_contract`: API shape, response envelope, status code, config schema, CLI contract
* `cross_platform`: web/mobile/desktop/native/deep link/multiple runtimes
* `existing_behavior`: thay đổi behavior đã tồn tại hoặc đã có test
* `weak_proof`: thiếu test, khó verify, unclear validation
* `multi_domain`: chạm nhiều product/technical domain

## Hard gates

Bất kỳ hard gate nào thì mặc định `lane = high_risk`:

```text
auth
authorization
data loss
data migration
audit/security
external provider behavior
removing validation
weakening validation
payment behavior
```

## Classification rule

```text
0-1 risk flags:
  tiny hoặc normal tùy code impact

2-3 risk flags:
  normal, cần validation rõ hơn

4+ risk flags:
  high_risk

any hard gate:
  high_risk unless user explicitly narrows scope

too ambiguous:
  needs_clarification
```

## Split rule

```text
tiny:
  split_required = false

normal:
  split_required = true nếu:
    - nhiều vertical slices
    - nhiều acceptance criteria
    - UI + state + persistence + validation
    - nhiều module chính
    - agent dễ miss requirement nếu làm một mạch

high_risk:
  split_required = true bắt buộc
```

---

# 4. Artifact layout

Implement layout mới:

```text
.coding/
├── state.json
└── packets/
    └── <packet-id>/
        ├── REQUEST.md
        ├── INTAKE.md
        ├── PACKET.md
        ├── STORIES.md
        ├── PACKET_VERIFY.md
        ├── PACKET_SHIP.md
        └── stories/
            ├── S01-storage/
            │   ├── STORY.md
            │   ├── PLAN.md
            │   ├── CODING.md
            │   ├── VERIFY.md
            │   └── SHIP.md
            ├── S02-create-form/
            └── S03-lock-unlock/
```

Với `tiny`, có thể dùng shortcut:

```text
.coding/packets/<packet-id>/
├── REQUEST.md
├── INTAKE.md
├── PLAN.md
├── CODING.md
├── VERIFY.md
└── SHIP.md
```

Không tạo các file JSON phase:

```text
request.json
intake.json
packet.json
stories.json
plan.json
coding.json
verify.json
ship.json
```

Chỉ được giữ **một file JSON persistent**:

```text
.coding/state.json
```

JSON phase vẫn chỉ là transient stdin/stdout.

---

# 5. CLI commands cần thêm

## Packet commands

Thêm:

```bash
cflow packet new "<title>"
cflow packet intake --packet current
cflow packet brief --packet current
cflow packet split --packet current
cflow packet status
cflow packet verify --packet current
cflow packet ship --packet current --dry-run
```

Ý nghĩa:

### `cflow packet new "<title>"`

Tạo packet folder:

```text
.coding/packets/<packet-id>/
```

Tạo/cập nhật `.coding/state.json`.

Set:

```text
current_packet_id = <packet-id>
```

### `cflow packet intake`

Đọc JSON transient từ stdin hoặc gọi agent nếu đã có agent mode.

Render:

```text
INTAKE.md
```

Update state:

```text
lane
risk_flags
hard_gates
split_required
next_action
```

### `cflow packet brief`

Tạo:

```text
PACKET.md
```

PACKET.md là brief ngắn cho toàn packet, không quá dài.

Sections:

```md
# Packet

## Goal

## Scope

## Out of Scope

## Global Acceptance Criteria

## Technical Constraints

## Shared Data / Contracts

## Validation Strategy
```

### `cflow packet split`

Tạo:

```text
STORIES.md
stories/<story-id>/STORY.md
```

Chỉ chạy khi:

```text
split_required = true
```

Hoặc user gọi rõ để split.

### `cflow packet verify`

Verify toàn packet sau khi các stories xong.

Render:

```text
PACKET_VERIFY.md
```

### `cflow packet ship`

Ship packet sau khi packet verify passed.

Render:

```text
PACKET_SHIP.md
```

Không commit nếu user chưa yêu cầu rõ.

---

## Story commands

Thêm:

```bash
cflow story list
cflow story switch <story-id>
cflow story status
cflow story plan --story current
cflow story coding --story current
cflow story coding --story current --fix
cflow story verify --story current
cflow story ship --story current --dry-run
```

Story loop dùng lại logic task hiện tại.

Nếu đã có `agent plan/coding`, hỗ trợ:

```bash
cflow story agent plan --story current
cflow story agent coding --story current
cflow story agent coding --story current --fix
```

Hoặc nếu codebase đang dùng form:

```bash
cflow agent plan --story current
cflow agent coding --story current
```

thì giữ form đó, miễn là story-level đúng.

---

# 6. State v2

Update `.coding/state.json` lên version 2.

Shape đề xuất:

```json
{
  "version": 2,
  "current_packet_id": "20260602-153012-time-capsule-notes",
  "current_story_id": "S02-create-form",
  "packets": {
    "20260602-153012-time-capsule-notes": {
      "title": "Time Capsule Notes",
      "status": "in_progress",
      "phase": "story_in_progress",
      "lane": "normal",
      "split_required": true,
      "risk_flags": ["existing_behavior", "weak_proof"],
      "hard_gates": [],
      "next_action": "story_verify",
      "created_at": "2026-06-02T15:30:12+07:00",
      "updated_at": "2026-06-02T16:10:45+07:00",
      "artifacts": {
        "request": true,
        "intake": true,
        "packet": true,
        "stories": true,
        "packet_verify": false,
        "packet_ship": false
      },
      "stories": {
        "S01-storage": {
          "title": "Storage and data model",
          "status": "done",
          "phase": "shipped",
          "findings_count": 0
        },
        "S02-create-form": {
          "title": "Create capsule form and validation",
          "status": "in_progress",
          "phase": "verify_failed",
          "findings_count": 2
        }
      }
    }
  }
}
```

Phases packet-level:

```text
new
requested
intake_done
packet_briefed
split_done
story_in_progress
packet_verify_pending
packet_verify_failed
packet_verify_passed
packet_ship_ready
committed
blocked
```

Phases story-level:

```text
new
planned
coding_done
verify_pending
verify_failed
verify_passed
ship_ready
committed
blocked
```

---

# 7. Status / next behavior

Update:

```bash
cflow status
cflow next
```

## `cflow status`

Nếu đang ở packet có stories, output ngắn:

```text
Current packet: 20260602-153012-time-capsule-notes
Title: Time Capsule Notes
Lane: normal
Phase: story_in_progress
Split required: true

Current story: S02-create-form
Story phase: verify_failed
Findings: 2

Next:
cflow story coding --story current --fix
```

## `cflow next`

Decision rules:

```text
REQUEST.md missing:
  next = packet intake or request creation

INTAKE.md missing:
  next = cflow packet intake --packet current

lane = tiny and PLAN.md missing:
  next = cflow agent plan --packet current

lane = tiny and CODING.md missing:
  next = cflow agent coding --packet current

split_required = true and PACKET.md missing:
  next = cflow packet brief --packet current

split_required = true and STORIES.md missing:
  next = cflow packet split --packet current

current story missing:
  next = cflow story list / cflow story switch <story-id>

current story PLAN.md missing:
  next = cflow story agent plan --story current

current story CODING.md missing:
  next = cflow story agent coding --story current

current story VERIFY.md missing:
  next = cflow story verify --story current

story VERIFY.md failed/partial:
  next = cflow story agent coding --story current --fix

all stories verified/shipped:
  next = cflow packet verify --packet current

PACKET_VERIFY.md passed and PACKET_SHIP.md missing:
  next = cflow packet ship --packet current --dry-run
```

Never suggest commit automatically.

---

# 8. Story splitting rules

`cflow packet split` / agent split phải tạo stories theo rule:

Một story tốt:

```text
- 1 behavior chính
- 2-5 acceptance criteria
- 1-3 nhóm file chính
- có verify rõ
- có thể commit độc lập
- không cần đọc toàn bộ chat
```

Không tạo story quá lớn.

Không tạo story kiểu technical-only nếu không cần.

Ưu tiên vertical slice:

```text
storage + model
create form + validation
display behavior
delete + empty state
final regression/polish
```

Ví dụ Time Capsule Notes:

```text
S01-storage
S02-create-form-validation
S03-locked-unlocked-display
S04-delete-empty-state
S05-final-regression-polish
```

---

# 9. Agent prompt constraints

Update skills/agent prompts để tránh lệch plan.

## Agent plan

Agent plan chỉ được đọc:

```text
REQUEST.md
INTAKE.md
PACKET.md nếu có
STORY.md nếu đang plan story
```

Agent plan không được implement.

Output JSON only.

## Agent coding

Agent coding chỉ được implement story hiện tại.

Nếu đang trong story:

Bắt buộc đọc:

```text
PACKET.md
STORY.md
PLAN.md
```

Có thể đọc:

```text
VERIFY.md nếu --fix
CODING.md nếu --fix
```

Không được sửa:

```text
.coding/**/*.md
.coding/state.json
```

Không được mở rộng scope sang story khác.

Nếu phát hiện story phụ thuộc story chưa xong:

* không tự làm story khác
* return status `blocked`
* ghi note rõ

## Agent verify

Verify chỉ kiểm tra.

Không được sửa code.

Verify phải check against:

```text
REQUEST.md
PACKET.md
STORY.md
PLAN.md
CODING.md
```

Nếu fail, phải tạo finding cụ thể.

---

# 10. Ship behavior

Story ship:

```text
cflow story ship
```

Chỉ ship story nếu story verify passed.

Packet ship:

```text
cflow packet ship
```

Chỉ ship packet nếu:

```text
all required stories are verified/shipped
PACKET_VERIFY.md passed
no open findings
```

Không commit tự động nếu không có `--commit`.

Nếu có commit:

* story commit message có thể là:

```text
feat(time-capsule): add capsule storage
```

* packet commit message có thể là merge/final:

```text
feat(time-capsule): complete time capsule notes mvp
```

Tùy implementation hiện tại, không bắt buộc implement complex commit logic ở version đầu.

---

# 11. Backward compatibility

Không được phá flow task hiện tại.

Các command cũ vẫn nên hoạt động:

```bash
cflow new
cflow request
cflow plan
cflow coding
cflow verify
cflow ship
```

Nhưng docs nên nói:

```text
Task flow = dùng cho tiny work hoặc story-level work.
Packet flow = dùng cho request vừa/lớn/risky.
```

---

# 12. Docs cần cập nhật

Update:

```text
README.md
AGENTS.md
skills/*
schemas/plan.schema.json
schemas/coding.schema.json
```

## README cần có 2 flow

### Tiny flow

```bash
cflow new "rename button"
cflow request
cflow agent plan
cflow agent coding
cflow verify
cflow ship --dry-run
```

### Packet flow

```bash
cflow packet new "time capsule notes"
cflow packet intake --packet current
cflow packet brief --packet current
cflow packet split --packet current

cflow story list
cflow story switch S01-storage
cflow story agent plan --story current
cflow story agent coding --story current
cflow story verify --story current
cflow story ship --story current --dry-run

cflow story switch S02-create-form
...

cflow packet verify --packet current
cflow packet ship --packet current --dry-run
```

## AGENTS.md cần nói rõ

```md
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
```

---

# 13. Tests / manual verification

Sau khi implement, chạy manual test.

## Test 1: tiny flow vẫn chạy

```bash
rm -rf .coding

cflow new "rename button"
# dùng JSON stdin hoặc existing helper để tạo REQUEST/PLAN/CODING/VERIFY/SHIP
cflow status
cflow next
```

Expected:

```text
Không crash.
Task flow cũ vẫn usable.
```

## Test 2: packet flow tạo artifacts

```bash
rm -rf .coding

cflow packet new "time capsule notes"
cflow packet status
```

Expected:

```text
.coding/state.json exists
.coding/packets/<packet-id>/ exists
current_packet_id set
```

## Test 3: split stories

Dùng sample intake normal + split_required true.

Expected:

```text
PACKET.md exists
STORIES.md exists
stories/S01-*/STORY.md exists
stories/S02-*/STORY.md exists
state.json has stories map
```

## Test 4: story loop

Chạy một story:

```bash
cflow story switch S01-storage
cflow story status
cflow story plan
cflow story coding
cflow story verify
```

Expected:

```text
story PLAN.md/CODING.md/VERIFY.md đúng folder story
state current_story_id đúng
```

## Test 5: verify failed loop

Tạo VERIFY.md failed có findings.

Run:

```bash
cflow next
```

Expected:

```text
Next: cflow story agent coding --story current --fix
```

Không được suggest main sửa code trực tiếp.

---

# 14. Deliverable

Sau khi sửa xong, báo cáo ngắn:

```text
- Files changed
- Commands added
- State changes
- Artifact layout
- Risk classification behavior
- Manual tests run
- Known limitations
```

Không thêm dependency lớn nếu chưa cần.
