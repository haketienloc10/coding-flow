# Coding Flow

`bin/cflow` là CLI workflow cho coding task. CLI lưu artifact dạng Markdown trong `.coding/` và dùng `.coding/state.json` làm state chính. JSON chỉ là input tạm thời qua `--input file` hoặc stdin.

## Workflow chính

Dùng cho thay đổi trung bình/lớn/nguy cơ cao. Luồng này đi qua `packet -> stories -> story loop -> packet verify -> packet ship`.

```bash
bin/cflow packet new "<title>"
cat intake.json | bin/cflow packet intake --packet current
cat packet.json | bin/cflow packet brief --packet current
cat stories.json | bin/cflow packet split --packet current

bin/cflow story list
bin/cflow story switch <story-id>
bin/cflow story agent plan --story current
bin/cflow story agent coding --story current
cat verify.json | bin/cflow story verify --story current
cat ship.json | bin/cflow story ship --story current --dry-run

cat packet_verify.json | bin/cflow packet verify --packet current
cat packet_ship.json | bin/cflow packet ship --packet current --dry-run
```

| Lệnh | Đầu vào |
| --- | --- |
| `bin/cflow packet new "<title>"` | Chuỗi title. Tạo packet mới và set current packet. |
| `bin/cflow packet intake --packet current [--input file]` | JSON có `request_summary`, `input_type`, `lane`, `next_action`. Optional: `risk_flags`, `hard_gates`, `split_required`, `reason`, `assumptions`, `clarifying_questions`. |
| `bin/cflow packet brief --packet current [--input file]` | JSON có `goal`, `scope.in`, `scope.out`, `global_acceptance_criteria`. Optional: `technical_constraints`, `shared_data_contracts`, `validation_strategy`. |
| `bin/cflow packet split --packet current [--input file]` | JSON có `stories[]`; mỗi story cần `id`, `title`, `description`, `acceptance_criteria`. Optional: `files_to_change`. |
| `bin/cflow story list` | Không cần input. Đọc current packet trong `.coding/state.json`. |
| `bin/cflow story switch <story-id>` | Story id hoặc prefix khớp, ví dụ `S01-lib-boundary`. |
| `bin/cflow story agent plan --story current [--provider name] [--verbose]` | Artifact của current story và packet context. Agent phải trả JSON theo `schemas/plan.schema.json`. |
| `bin/cflow story agent coding --story current [--provider name] [--fix] [--verbose]` | Artifact của current story. Dùng `--fix` khi verify fail. Agent phải trả JSON theo `schemas/coding.schema.json`. |
| `bin/cflow story verify --story current [--input file]` | JSON có `status`, `checks`, `manual_checks`, `acceptance_criteria_checked`, `findings`, `known_issues`, `done_criteria_verified`. |
| `bin/cflow story ship --story current [--input file] [--dry-run\|--commit]` | JSON có `ready=true`, `commit.type`, `commit.message`, `verification.status=passed`. Cần `VERIFY.md` passed và không có findings. |
| `bin/cflow packet verify --packet current [--input file]` | JSON có `status` là `passed` hoặc `failed`. Optional: `goal_achieved`, `regressions_checked`, `findings`. |
| `bin/cflow packet ship --packet current [--input file] [--dry-run\|--commit]` | JSON có `commit_message`. Optional: `changelog`. Cần `PACKET_VERIFY.md` passed. |

## Tiny/task flow

Dùng cho task nhỏ, doc-only, fix gọn, hoặc việc không cần chia story.

```bash
bin/cflow new "<task-name>"
cat request.json | bin/cflow request --task current
bin/cflow agent plan --task current
bin/cflow agent coding --task current
cat verify.json | bin/cflow verify --task current
cat ship.json | bin/cflow ship --task current --dry-run
```

| Lệnh | Đầu vào |
| --- | --- |
| `bin/cflow new "<task-name>"` | Chuỗi task name. Tạo task mới và set current task. |
| `bin/cflow request --task current [--input file]` | JSON có `summary`, `type`, `lane`, `next_action`. Optional: `planning_needed`, `risk_flags`, `hard_gates`, `assumptions`, `clarifying_questions`. |
| `bin/cflow agent plan --task current [--provider name] [--verbose]` | Artifact của current task. Agent phải trả JSON theo `schemas/plan.schema.json`. |
| `bin/cflow agent coding --task current [--provider name] [--fix] [--verbose]` | Artifact của current task. Dùng `--fix` khi verify fail. Agent phải trả JSON theo `schemas/coding.schema.json`. |
| `bin/cflow verify --task current [--input file]` | JSON có `status`, `checks`, `manual_checks`, `acceptance_criteria_checked`, `findings`, `known_issues`, `done_criteria_verified`. |
| `bin/cflow ship --task current [--input file] [--dry-run\|--commit]` | JSON có `ready=true`, `commit.type`, `commit.message`, `verification.status=passed`. Có thể dùng existing `SHIP.md` với `--dry-run` hoặc `--commit`. |

## Lệnh điều phối/tiện ích

| Lệnh | Đầu vào |
| --- | --- |
| `bin/cflow status` | Không cần input. In current task/packet/story, artifact status và next command. |
| `bin/cflow next --task current` | Optional `--task`. In next command và reason. |
| `bin/cflow run --task current` | Optional `--task`. Tự chạy các bước agent/task có thể tự động; dừng khi cần input người hoặc task xong. |
| `bin/cflow tasks` | Không cần input. Liệt kê task trong state. |
| `bin/cflow switch <task-id>` | Task id. Set current task và clear current packet/story. |
| `bin/cflow state repair` | Không cần input. Đồng bộ `.coding/state.json` từ artifact trên filesystem. |
| `bin/cflow agent providers` | Không cần input. Liệt kê provider và command đã resolve. |
| `bin/cflow agent doctor [--provider name]` | Optional provider. Kiểm tra command agent có sẵn sàng chạy không. |
| `bin/cflow story add --title "<title>" [--agent name]` | Tạo story draft trong task legacy, không tạo packet. |
| `bin/cflow story update <story-id> --status <status>` | Cập nhật status story trong task legacy. |
| `bin/cflow packet create --stories S-0001,S-0002 [--title "..."] [--agent name] [--force]` | Tạo packet từ story đã có. Cần `--force` nếu chỉ có 1 story. |
| `bin/cflow packet create --from-ready [--force]` | Tạo packet từ tất cả story status `ready`. |
| `bin/cflow packet list [--status s] [--story id] [--agent name]` | Liệt kê packet legacy, có filter optional. |
| `bin/cflow packet show <packet-id>` | In packet entry legacy. |
| `bin/cflow problem add` | JSON qua stdin/`--input`. Ghi vào `.coding/knowledge/PROBLEMS.md`. |
| `bin/cflow problem list [--status open\|resolved\|cancelled]` | Optional status filter. |
| `bin/cflow problem show <id>` | Problem id, ví dụ `P001`. |
| `bin/cflow problem resolve <id> --note "<text>"` | Problem id và note bắt buộc. |
| `bin/cflow problem cancel <id> --note "<text>"` | Problem id và note bắt buộc. |
| `bin/cflow problem update <id> --status <open\|resolved\|cancelled> --note "<text>"` | Cập nhật status problem. |
| `bin/cflow decision add --title "<title>" --agent "<agent>" [--status proposed]` | Tạo decision trong `.coding/knowledge/decisions.md`. Optional: `--related`, `--context`, `--decision`, `--options`, `--pros`, `--cons`, `--consequences`. |
| `bin/cflow decision list [--status s] [--agent name] [--related id]` | Liệt kê/filter decision. |
| `bin/cflow decision show <id>` | Decision id, ví dụ `D-0001`. |
| `bin/cflow decision accept <id>` | Chuyển decision `proposed` sang `accepted`. |
| `bin/cflow decision reject <id>` | Chuyển decision `proposed` sang `rejected`. |
| `bin/cflow decision supersede <id> --by <decision-id>` | Chuyển decision `accepted` sang `superseded`. |

Có thể truyền JSON bằng file:

```bash
bin/cflow packet intake --packet current --input intake.json
```

Hoặc qua stdin:

```bash
cat intake.json | bin/cflow packet intake --packet current
```
