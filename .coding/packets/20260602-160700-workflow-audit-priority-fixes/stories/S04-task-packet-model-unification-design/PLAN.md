# Code Plan

## 1. Objective

Xác định và ghi lại hướng migration để giảm nhầm lẫn giữa task/packet model, bao gồm vai trò mới của .coding/state.json và chính sách deprecate .coding/current theo cách giữ backward compatibility cho các lệnh task hiện có.

## 2. Scope

### In Scope

- Đánh giá luồng hiện tại liên quan đến task, packet, story, current task và .coding/current trong src/main.rs.
- Ghi decision bằng cflow decision add về migration path, tradeoffs, và các phương án bị loại.
- Cập nhật tài liệu README.md, AGENTS.md, GEMINI.md để mô tả model thống nhất và trạng thái deprecation của .coding/current.
- Xác định các story implementation tiếp theo cho phần code migration lớn hơn.

### Out of Scope

- Không thực hiện full task/packet model unification trong story này.
- Không split hoặc modularize toàn bộ src/main.rs.
- Không xóa .coding/current ngay lập tức.
- Không phá behavior của các lệnh task legacy hoặc thay đổi CLI contract ngoài phạm vi migration design.

## 3. Requirements

- Decision phải ghi rõ migration path được chọn, tradeoffs, hậu quả, và các follow-up liên quan.
- CLI task commands phải tiếp tục backward compatible trong giai đoạn chuyển tiếp.
- .coding/state.json được định nghĩa là canonical state dài hạn; .coding/current chỉ là legacy compatibility pointer nếu còn cần.
- Follow-up implementation stories phải đủ rõ để thực hiện từng bước mà không nhập chung architecture work lớn vào story hiện tại.
- Không edit trực tiếp .coding markdown artifacts; dùng cflow commands cho decision/story artifacts.

## 4. Technical Approach

- Đọc src/main.rs để xác định nơi task, packet, story, current state, và .coding/current đang được đọc/ghi.
- Chọn migration strategy thận trọng: state.json là source of truth mới; .coding/current được giữ làm fallback/compatibility shim trong một giai đoạn, không xóa ngay.
- Dùng cflow decision add để ghi lại quyết định và tradeoffs thay vì sửa .coding/knowledge/decisions.md trực tiếp.
- Cập nhật README.md, AGENTS.md, GEMINI.md để đồng bộ terminology: request/task/story/packet, cách chọn current item, và quy tắc không tạo packet tự động.
- Tạo hoặc đề xuất follow-up stories cho các bước implementation: compatibility shim, state normalization, command deprecation messaging, và eventual removal plan.
- Chạy kiểm tra build/test phù hợp để bảo đảm thay đổi tài liệu/nhỏ trong src/main.rs không làm hỏng CLI.

## 5. Files to Change

- src/main.rs
- README.md
- AGENTS.md
- GEMINI.md
- .coding/knowledge/decisions.md (via cflow decision add only)
- .coding/state.json (via cflow story/decision commands only if needed)

## 6. Implementation Steps

- [todo] Inspect current task/packet/story state handling in src/main.rs and existing documentation terminology.
- [todo] Define the migration path and compatibility policy for .coding/state.json versus .coding/current.
- [todo] Record the migration decision with cflow decision add, including options considered and tradeoffs.
- [todo] Update README.md, AGENTS.md, and GEMINI.md with the chosen model and transition guidance.
- [todo] Add narrowly scoped src/main.rs changes only if needed to preserve or clarify backward-compatible behavior for existing task commands.
- [todo] Create clear follow-up implementation stories using cflow story commands for the remaining migration work.
- [todo] Run cargo fmt --check, cargo test, and targeted CLI smoke checks for current task/story/packet behavior.

## 7. Test Plan

### Planned

- cargo fmt --check
- cargo test
- cargo build --release if src/main.rs changes are made
- CLI smoke: cflow --help or relevant task/story/packet current commands
- Manual documentation review for README.md, AGENTS.md, GEMINI.md consistency

### Result

- _None_

## 8. Risks

- src/main.rs may already conflate task and packet state in ways that make even small code changes risky; keep implementation minimal in this story.
- Deprecating .coding/current too aggressively could break existing user workflows or scripts.
- Documentation could get ahead of implementation if follow-up stories are not explicit.
- cflow decision/story commands may modify workflow artifacts; these changes must be CLI-managed and not manually edited.

## 9. Done Criteria

### Criteria

- A decision exists that records the chosen task/packet migration path, tradeoffs, and rejected options.
- Documentation describes the unified model, backward compatibility stance, and .coding/current deprecation policy.
- Existing task CLI behavior remains backward compatible after the story changes.
- Follow-up implementation stories are present and actionable for the larger migration steps.
- Required verification commands pass or any failure is documented with cause and next action.

### Verified

- _None_
