# Request Intake

## Summary

Update AGENTS.md request command examples to pipe inline JSON via stdin using the same pattern as skills/request.md instead of referencing request.json files.

## Type

documentation

## Planning Needed

true

## Lane

tiny

## Risk Flags

- documentation_update

## Hard Gates

- AGENTS.md should no longer contain cat request.json examples

## Assumptions

- Inline heredoc JSON is the preferred agent instruction pattern for request intake.
- Other phase examples can remain as named JSON placeholders for now unless they are specifically changed.

## Clarifying Questions

- _None_

## Next Action

plan
