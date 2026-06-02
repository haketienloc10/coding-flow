# Story: Extract artifact validation and rendering modules

## Description

Move request, plan, coding, verify, ship, packet, and story artifact validation/rendering into src/artifacts modules.

## Acceptance Criteria

- Artifact modules own their validate/render functions
- Existing examples still validate
- Rendered markdown remains stable
- Tests pass

## Files to Change

- src/lib.rs
- src/artifacts/
