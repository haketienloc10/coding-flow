# Code Plan

## 1. Objective

Prevent Codex CLI from parsing cflow prompts that start with dashes as unexpected options by adding an explicit -- separator before the prompt argument.

## 2. Scope

### In Scope

- Update built-in Codex plan command args
- Update built-in Codex coding command args
- Add or update focused tests for the -- separator

### Out of Scope

- Changing non-Codex provider behavior
- Changing prompt_mode semantics
- Changing schemas or JSON parsing

## 3. Requirements

- Codex plan command must include -- after schema args and before the prompt
- Codex coding command must include -- after schema args and before the prompt
- Existing agent execution behavior must remain shared and unchanged otherwise

## 4. Technical Approach

- Edit builtin_agent_command in src/main.rs for codex plan and coding to append -- to args.
- Update focused unit tests to assert Codex commands contain the separator at the final args position.
- Run formatting, unit tests, and a doctor smoke check after rebuilding release so bin/cflow uses the updated binary.

## 5. Files to Change

- src/main.rs

## 6. Implementation Steps

- [todo] Add -- separator to Codex plan built-in args.
- [todo] Add -- separator to Codex coding built-in args.
- [todo] Update tests for separator placement.
- [todo] Run cargo fmt/test/build and cflow doctor smoke check.

## 7. Test Plan

### Planned

- cargo fmt --check
- cargo test
- cargo build --release
- bin/cflow agent doctor --provider codex

### Result

- _None_

## 8. Risks

- Running full agent plan/coding would invoke an external model; command shape can be verified by unit tests and doctor output without consuming an agent run.

## 9. Done Criteria

### Criteria

- Codex plan args end with --
- Codex coding args end with --
- Tests pass
- Doctor output shows -- before "<PROMPT>" for Codex

### Verified

- _None_
