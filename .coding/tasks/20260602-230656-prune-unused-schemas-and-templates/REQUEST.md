# Request Intake

## Summary

Prune unused schema and template files by keeping only agent output schemas and moving packet markdown rendering fully into Rust code.

## Type

maintenance

## Planning Needed

true

## Lane

tiny

## Risk Flags

- file_deletion
- rendering_change

## Hard Gates

- cargo test must pass
- no templates references remain

## Assumptions

- Only schemas/plan.schema.json and schemas/coding.schema.json are required by CLI agent commands.
- Markdown rendering should live consistently in Rust code rather than templates.

## Clarifying Questions

- _None_

## Next Action

plan
