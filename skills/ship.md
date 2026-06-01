---
name: cflow:ship
description: Prepare and commit verified work.
argument-hint: "[--dry-run] [--commit]"
allowed-tools:
  - Read
  - Bash
requires:
  - cflow
---

# cflow:ship

## Purpose

Prepare the final shipping artifact and optionally create a git commit.

## Boundary

```text
Verified work
  -> LLM/dev prepares ship JSON
  -> CLI validates verification status
  -> CLI renders .coding/SHIP.md
  -> CLI optionally commits
```

## Rules

- Never ship if verification status is not `passed`.
- Never commit unless `--commit` is explicitly requested.
- Prefer `--dry-run` first.
- Commit message should follow Conventional Commits.
- Do not modify request, plan, or verify markdown directly.

## JSON Output

Produce only valid JSON matching:

```json
{
  "ready": true,
  "commit": {
    "type": "feat",
    "scope": "",
    "message": "",
    "body": []
  },
  "changed_files": [],
  "summary": [],
  "verification": {
    "status": "passed",
    "source": ".coding/VERIFY.md"
  },
  "notes": []
}
```

Allowed commit types:

```text
feat
fix
refactor
docs
test
chore
ci
build
perf
```

## CLI Commands

Dry run:

```bash
cflow ship --input .coding/ship.json --dry-run
```

Commit:

```bash
cflow ship --input .coding/ship.json --commit
```
