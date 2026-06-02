# Story: Extract common argument filesystem and markdown helpers

## Description

Move shared CLI argument, filesystem, JSON input, and markdown helper functions into focused common modules.

## Acceptance Criteria

- Helpers live outside src/main.rs/lib root
- Call sites use explicit module imports
- No behavior change in rendered artifacts or CLI errors
- Tests pass

## Files to Change

- src/lib.rs
- src/args.rs
- src/fs_util.rs
- src/markdown.rs
