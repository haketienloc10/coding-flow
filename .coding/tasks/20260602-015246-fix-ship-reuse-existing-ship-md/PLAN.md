# Code Plan

## 1. Objective

Allow cflow ship to reuse an existing SHIP.md when no JSON input is available, while keeping verification gates and git actions explicit.

## 2. Scope

### In Scope

- Update ship command input handling
- Gate dry-run and commit on VERIFY.md status and findings
- Parse commit subject from SHIP.md
- Add focused unit tests and CLI verification

### Out of Scope

- Changing request/plan/coding/verify rendering formats
- Creating git commits

## 3. Requirements

- No-flag ship command must not block on stdin when no JSON is provided
- Existing SHIP.md should produce dry-run and commit guidance
- Dry-run should reuse SHIP.md if present and run git status --short
- Commit should reuse SHIP.md if present and run git add . plus git commit -m with parsed subject

## 4. Technical Approach

- Add optional JSON input reading that returns None for empty non-interactive stdin
- Add small markdown section helpers for VERIFY.md and SHIP.md parsing
- Refactor command_ship around explicit dry-run and commit modes
- Keep JSON rendering path for callers that still pipe or pass --input

## 5. Files to Change

- src/main.rs

## 6. Implementation Steps

- [done] Add optional JSON input helper
- [done] Add VERIFY.md gate helper
- [done] Add SHIP.md commit message parser
- [done] Refactor command_ship mode handling
- [done] Add focused unit tests

## 7. Test Plan

### Planned

- cargo fmt --check
- cargo test
- Manual cflow ship no-input guidance checks
- Manual cflow ship --dry-run reuse check

### Result

- All planned checks passed

## 8. Risks

- Markdown parsing is intentionally narrow and depends on current cflow-rendered headings

## 9. Done Criteria

### Criteria

- No-input ship no longer fails with invalid JSON
- Existing SHIP.md can be reused for dry-run
- Commit subject is parsed from SHIP.md
- VERIFY.md status and findings are checked before git actions

### Verified

- Unit tests cover commit message and findings parsing
- CLI dry-run reuse was verified manually
