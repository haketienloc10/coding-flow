# Ship

## Ready

true

## Commit Message

```text
fix(agent-provider): separate codex prompt argument
```

## Commit Body

- Add an end-of-options separator before Codex arg-mode prompts so prompts beginning with dashes are not parsed as CLI options.

## Changed Files

- src/main.rs

## Summary

- Fixed Codex provider command args by inserting -- before the prompt.
- Verified formatting, tests, release build, and doctor command output.

## Verification

Status: passed

Source: .coding/tasks/20260602-100049-fix-codex-prompt-separator/VERIFY.md

## Notes

- Dry run only; no commit was created.
