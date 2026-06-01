# Ship

## Ready

true

## Commit Message

```text
fix(ship): reuse existing SHIP.md for git actions
```

## Commit Body

- Allow cflow ship to avoid stdin blocking when no JSON input is provided.
- Reuse an existing SHIP.md for dry-run and commit modes after VERIFY.md gates pass.

## Changed Files

- src/main.rs
- .coding/tasks/20260602-015246-fix-ship-reuse-existing-ship-md/REQUEST.md
- .coding/tasks/20260602-015246-fix-ship-reuse-existing-ship-md/PLAN.md
- .coding/tasks/20260602-015246-fix-ship-reuse-existing-ship-md/CODING.md
- .coding/tasks/20260602-015246-fix-ship-reuse-existing-ship-md/VERIFY.md
- .coding/tasks/20260602-015246-fix-ship-reuse-existing-ship-md/SHIP.md

## Summary

- Added optional JSON input handling for ship so empty stdin can mean reuse existing SHIP.md.
- Added VERIFY.md gates for passed status and empty findings before dry-run or commit.
- Added SHIP.md commit subject parsing and reused it for commit mode.

## Verification

Status: passed

Source: .coding/tasks/20260602-015246-fix-ship-reuse-existing-ship-md/VERIFY.md

## Notes

- No git commit was created.
