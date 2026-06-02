# Code Plan

## 1. Objective

Add Antigravity as a first-class cflow agent provider for planning, coding, coding fixes, provider inspection, doctor checks, and documentation.

## 2. Scope

### In Scope

- Add antigravity to the built-in provider list and resolution path
- Configure built-in plan and coding commands as agy --prompt with prompt_mode arg
- Ensure existing stdout capture, JSON parse, validation, and markdown rendering paths apply to Antigravity without writing JSON files
- Update provider listing and doctor output to include Antigravity and print agy --prompt "<PROMPT>"
- Update README examples and AGENTS.md provider note
- Add or update focused tests for provider normalization, command resolution, provider list/doctor-relevant command shape, and docs behavior where practical

### Out of Scope

- Changing schemas for plan or coding artifacts
- Adding persistent JSON output files
- Changing unrelated provider behavior beyond what is necessary for consistency

## 3. Requirements

- cflow agent plan --provider antigravity --task current must run agy --prompt <prompt>
- cflow agent coding --provider antigravity --task current must run agy --prompt <prompt>
- cflow agent coding --provider antigravity --task current --fix must run agy --prompt <prompt>
- stdout must be captured, parsed as JSON, validated using existing plan/coding structs/schema, and rendered to PLAN.md or CODING.md
- JSON output must remain transient and must not be saved
- full stdout must remain hidden unless --verbose
- cflow agent providers must show antigravity
- cflow agent doctor --provider antigravity must check agy in PATH and print agy --prompt "<PROMPT>"
- README provider examples and AGENTS.md provider note must mention Antigravity

## 4. Technical Approach

- Inspect src/main.rs provider constants, command resolution, built-in command mapping, doctor, providers output, and tests.
- Add antigravity to AGENT_PROVIDERS or the provider enum/list equivalent so normalize_agent_provider accepts it and providers output iterates over it.
- Extend builtin_agent_command for both AgentPhase::Plan and AgentPhase::Coding with cmd agy, args [--prompt], prompt_mode arg.
- Reuse existing run_agent_command flow so stdout capture, verbose handling, JSON deserialization, schema validation, and markdown rendering remain unchanged.
- Verify doctor output already derives from resolved AgentCommand; adjust only if needed so antigravity checks agy and displays the expected command shape.
- Update README provider examples and AGENTS.md provider note with antigravity usage and config snippet/availability note.
- Run cargo fmt and cargo test; run cflow provider/doctor smoke commands where possible. If agy is not installed, verify doctor reports the missing executable while still printing resolved command.

## 5. Files to Change

- src/main.rs
- README.md
- AGENTS.md

## 6. Implementation Steps

- [todo] Read provider-related code paths and tests in src/main.rs.
- [todo] Add antigravity to built-in provider resolution and command configuration for plan/coding.
- [todo] Update or add focused tests for antigravity provider normalization/resolution and command shape.
- [todo] Update README provider examples and AGENTS.md provider note.
- [todo] Run formatting, tests, and cflow provider/doctor smoke checks.

## 7. Test Plan

### Planned

- cargo fmt --check or cargo fmt
- cargo test
- bin/cflow agent providers
- bin/cflow agent doctor --provider antigravity

### Result

- _None_

## 8. Risks

- Existing provider command execution behavior may need a small adjustment if prompt_mode arg is not compatible with agy args ordering.
- Doctor command may fail on machines without agy installed; expected behavior should still make the missing PATH check explicit.
- Tests may need to avoid depending on agy being installed.

## 9. Done Criteria

### Criteria

- Antigravity is accepted as --provider antigravity and via normal provider resolution inputs.
- Plan and coding phases resolve to agy --prompt with prompt passed as an argument.
- Existing transient JSON parse/validate/render behavior is preserved for Antigravity.
- Provider list and doctor support Antigravity, with doctor showing agy --prompt "<PROMPT>".
- README and AGENTS.md mention Antigravity provider usage.
- Focused automated/smoke verification is completed or limitations are recorded.

### Verified

- _None_
