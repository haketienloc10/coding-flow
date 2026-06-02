# Request Intake

## Summary

Remove the examples directory completely and update code/tests/docs so the repository no longer depends on examples/*.

## Type

maintenance

## Planning Needed

true

## Lane

tiny

## Risk Flags

- test_fixture_removal
- documentation_update

## Hard Gates

- cargo test must pass

## Assumptions

- Example files are not part of runtime CLI behavior.
- Tests should use inline fixtures instead of examples/* files.

## Clarifying Questions

- _None_

## Next Action

plan
