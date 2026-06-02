# Story: Introduce library boundary and thin main

## Description

Move the cflow runtime entrypoint into src/lib.rs and reduce src/main.rs to process exit handling only.

## Acceptance Criteria

- src/lib.rs exposes run() for the binary
- src/main.rs is a thin wrapper around cflow::run()
- Cargo tests and build pass
- CLI status smoke check still works

## Files to Change

- src/main.rs
- src/lib.rs
