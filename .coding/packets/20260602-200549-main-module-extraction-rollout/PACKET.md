# Packet

## Goal

Incrementally decompose src/main.rs into focused Rust modules while preserving cflow CLI behavior.

## Scope

### In Scope

- Create a src/lib.rs boundary and thin binary entrypoint
- Move shared helpers and domains in later stories
- Keep existing command behavior and markdown output stable
- Verify each extraction slice with cargo tests and CLI smoke checks

### Out of Scope

- No broad command model rewrite
- No clap or new CLI framework adoption
- No persistent phase JSON files
- No removal of .coding/current compatibility

## Global Acceptance Criteria

- cargo test passes after each story
- cargo fmt --check passes after each story
- CLI command routing remains compatible
- Each story moves one coherent responsibility only

## Technical Constraints

- Do not edit .coding markdown artifacts directly
- Do not implement more than the current story
- Prefer behavior-neutral code movement
- Preserve existing workflow_vocab module

## Shared Data / Contracts

- CflowResult remains the common command result type
- run() remains the top-level CLI dispatcher exposed to the binary
- State remains stored in .coding/state.json with .coding/current compatibility

## Validation Strategy

- cargo fmt --check
- cargo test --quiet
- cargo build --quiet
- bin/cflow status
- story-level verify artifacts
