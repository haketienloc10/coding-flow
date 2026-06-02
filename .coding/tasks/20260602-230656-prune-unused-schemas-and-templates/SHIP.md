# Ship

## Ready

true

## Commit Message

```text
chore(repo): prune unused schemas and templates
```

## Commit Body

- Keep only agent output schemas under schemas/.
- Inline packet verify and packet ship rendering in Rust.
- Remove templates/ and unused schema files.

## Changed Files

- src/lib.rs
- US-update-packet-workflow.md
- schemas/
- templates/

## Summary

- Removed the templates directory.
- Deleted unused schema files, leaving plan.schema.json and coding.schema.json.
- Inlined packet verify and packet ship renderers.

## Verification

Status: passed

Source: .coding/tasks/20260602-230656-prune-unused-schemas-and-templates/VERIFY.md

## Notes

- Dry run only; no commit was created.
