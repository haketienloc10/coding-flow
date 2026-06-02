# Coding

## Mode

initial

## Status

ready_for_verify

## Summary

- Đã ghi decision D-0005 về migration path: .coding/state.json là canonical state, .coding/current chỉ là legacy compatibility pointer.
- Đã cập nhật README.md, AGENTS.md, GEMINI.md để mô tả unified task/packet/story model, policy deprecate .coding/current, và các follow-up migration stories.
- Đã thêm helper write_legacy_current_pointer trong src/main.rs và dùng cho các điểm ghi .coding/current để làm rõ compatibility behavior mà không đổi CLI contract.

## Fixed Findings

- _None_

## Changed Files

- src/main.rs
- README.md
- AGENTS.md
- GEMINI.md
- .coding/knowledge/decisions.md

## Notes

- Đã chạy cargo fmt.
- Không chạy verify/test/ship theo instruction của phase coding.

## Next

verify
