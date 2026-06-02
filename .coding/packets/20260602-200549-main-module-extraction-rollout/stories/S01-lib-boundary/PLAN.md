# Code Plan

## 1. Objective

Introduce a Rust library boundary for cflow and make src/main.rs a thin binary wrapper.

## 2. Scope

### In Scope

- Create src/lib.rs containing the current cflow implementation
- Expose run() from the library
- Keep src/main.rs limited to calling cflow::run() and process exit handling
- Preserve workflow_vocab module availability from the library

### Out of Scope

- No helper/domain extraction beyond the library boundary
- No CLI behavior changes
- No test refactor beyond what is necessary to compile

## 3. Requirements

- cargo fmt --check passes
- cargo test --quiet passes
- cargo build --quiet passes
- bin/cflow status works after rebuild

## 4. Technical Approach

- Copy the existing src/main.rs implementation into src/lib.rs as the library root
- Remove binary-only main() from src/lib.rs and make run() public
- Replace src/main.rs with a minimal wrapper invoking cflow::run()
- Adjust imports so std::process is only needed by the binary if no longer used in lib
- Keep tests with the library implementation for this slice

## 5. Files to Change

- src/main.rs
- src/lib.rs

## 6. Implementation Steps

- [todo] Create src/lib.rs from the existing implementation
- [todo] Make run() public in src/lib.rs
- [todo] Replace src/main.rs with thin process wrapper
- [todo] Run formatting tests build and CLI smoke check

## 7. Test Plan

### Planned

- cargo fmt --check
- cargo test --quiet
- cargo build --quiet
- bin/cflow status

### Result

- _None_

## 8. Risks

- Binary crate name import must match Cargo package library naming
- Tests may need to live in lib.rs after moving implementation
- Stale target/debug/cflow can confuse smoke checks if build is not run

## 9. Done Criteria

### Criteria

- src/main.rs is thin
- src/lib.rs owns runtime implementation
- Existing tests pass
- CLI status still prints current workflow status

### Verified

- _None_
