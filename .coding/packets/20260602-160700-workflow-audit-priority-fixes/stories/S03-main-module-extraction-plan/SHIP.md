# Ship

## Ready

true

## Commit Message

```text
refactor(workflow): extract workflow vocabulary module
```

## Commit Body

- Move workflow vocabulary and knowledge path constants out of src/main.rs.
- Keep behavior unchanged while establishing a small module boundary.

## Changed Files

- src/main.rs
- src/workflow_vocab.rs
- .coding/packets/20260602-160700-workflow-audit-priority-fixes/stories/S03-main-module-extraction-plan/PLAN.md
- .coding/packets/20260602-160700-workflow-audit-priority-fixes/stories/S03-main-module-extraction-plan/CODING.md
- .coding/packets/20260602-160700-workflow-audit-priority-fixes/stories/S03-main-module-extraction-plan/VERIFY.md
- .coding/packets/20260602-160700-workflow-audit-priority-fixes/stories/S03-main-module-extraction-plan/SHIP.md

## Summary

- Extracted workflow vocabulary constants into src/workflow_vocab.rs.
- Verified formatting, tests, release build, and story CLI smoke checks.

## Verification

Status: passed

Source: .coding/packets/20260602-160700-workflow-audit-priority-fixes/stories/S03-main-module-extraction-plan/VERIFY.md

## Notes

- Committed story changes.
