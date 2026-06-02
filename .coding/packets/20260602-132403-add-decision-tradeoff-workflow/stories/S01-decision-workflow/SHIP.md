# Ship

## Ready

true

## Commit Message

```text
feat(decision): add decision log workflow
```

## Commit Body

- Add markdown-backed cflow decision commands.
- Document decision logging guidance and add tests.

## Changed Files

- src/main.rs
- README.md
- AGENTS.md
- GEMINI.md
- .coding/knowledge/decisions.md

## Summary

- Added decision log workflow and lifecycle commands.
- Added tests and docs.

## Verification

Status: passed

Source: cargo test; cargo build --release; temp-workspace decision CLI smoke

## Notes

- Dry-run only; no commit created.
