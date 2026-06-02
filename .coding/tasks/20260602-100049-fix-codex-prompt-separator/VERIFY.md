# Verify

## Status

passed

## Automated Checks

- cargo fmt --check: passed
- cargo test: passed (15 tests)
- cargo build --release: passed
- bin/cflow agent doctor --provider codex: passed; plan and coding commands include -- before "<PROMPT>"

## Manual Checks

- Reviewed src/main.rs diff to confirm separator is only added to built-in Codex args
- Confirmed prompt_mode remains arg and shared run_agent stdout/JSON behavior is unchanged

## Acceptance Criteria Checked

- Built-in Codex plan command resolves with -- before the prompt argument
- Built-in Codex coding command resolves with -- before the prompt argument
- Existing stdout capture, JSON parsing, validation, and rendering behavior is preserved
- Focused tests verify the separator is present

## Findings

- _None_

## Known Issues

- _None_

## Done Criteria Verified

- Codex plan args end with --
- Codex coding args end with --
- Tests pass
- Doctor output shows -- before "<PROMPT>" for Codex
