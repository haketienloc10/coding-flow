# Ship

## Ready

true

## Commit Message

```text
refactor(cli): introduce cflow library boundary
```

## Commit Body

- Move the runtime implementation into src/lib.rs.
- Keep src/main.rs as a thin binary wrapper.

## Changed Files

- src/lib.rs
- src/main.rs

## Summary

- Introduced library boundary for cflow runtime.
- Preserved existing CLI behavior and tests.

## Verification

Status: passed

Source: .coding/packets/20260602-200549-main-module-extraction-rollout/stories/S01-lib-boundary/VERIFY.md

## Notes

- No helper/domain extraction included in this story.
