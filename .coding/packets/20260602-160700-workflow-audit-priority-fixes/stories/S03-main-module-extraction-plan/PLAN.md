# Code Plan

## 1. Objective

Create the first conservative module extraction slice by moving workflow vocabulary constants out of src/main.rs.

## 2. Scope

### In Scope

- Extract low-risk workflow vocabulary/path constants from src/main.rs into a new Rust module.
- Keep validator and renderer behavior unchanged while using the extracted public constants.
- Run formatting, tests, build, and story-level CLI smoke checks.

### Out of Scope

- No full src/main.rs decomposition.
- No task/packet model unification.
- No CLI behavior changes beyond compile-equivalent module extraction.

## 3. Requirements

- Extract a small low-risk domain from src/main.rs into src/.
- Public boundaries must be clear and compile cleanly.
- Existing tests and CLI smoke checks must still pass.

## 4. Technical Approach

- Add a workflow_vocab module containing the CLI vocabulary arrays and knowledge path constants currently defined at the top of src/main.rs.
- Import the extracted constants in src/main.rs through an explicit use list so dependencies remain visible.
- Avoid logic movement in this story to keep the slice behavior-neutral.

## 5. Files to Change

- src/main.rs
- src/workflow_vocab.rs

## 6. Implementation Steps

- [todo] Create workflow_vocab module with public constants for request, coding, verify, problem, and decision vocabularies.
- [todo] Replace the constants removed from src/main.rs with an explicit module import list.
- [todo] Run cargo fmt --check, cargo test, cargo build --release, and story status/list smoke checks.

## 7. Test Plan

### Planned

- cargo fmt --check
- cargo test
- cargo build --release
- ./bin/cflow story status
- ./bin/cflow story list

### Result

- _None_

## 8. Risks

- Moving constants can break compile if any import is missed.
- This foundation slice reduces monolith size only slightly and intentionally avoids deeper behavior-bearing extraction.

## 9. Done Criteria

### Criteria

- A small low-risk domain is extracted from src/main.rs into a module.
- Public constants are exposed through a clear module boundary.
- Existing tests and CLI smoke checks pass.

### Verified

- _None_
