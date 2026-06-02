# Ship

## Ready

true

## Commit Message

```text
docs(agents): standardize phase JSON guidance on stdin heredoc
```

## Commit Body

- Route AGENTS.md phase steps to skills instead of persistent JSON filenames.
- Remove --input fallback wording from agent-facing skills.
- Add packet phase skill files for heredoc payloads.

## Changed Files

- AGENTS.md
- skills/coding.md
- skills/verify.md
- skills/ship.md
- skills/intake.md
- skills/packet-brief.md
- skills/packet-split.md
- skills/packet-verify.md
- skills/packet-ship.md

## Summary

- Standardized agent-facing phase JSON guidance on stdin heredoc only.
- Kept AGENTS.md concise by routing payload details to skill files.
- Added missing packet phase skill files.

## Verification

Status: passed

Source: .coding/tasks/20260602-231730-align-agents-request-command-with-skill/VERIFY.md

## Notes

- Dry run only; no commit was created.
