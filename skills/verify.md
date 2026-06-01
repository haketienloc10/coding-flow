---
name: cflow:verify
description: Verify implementation before shipping.
argument-hint: "[--input <verify.json>]"
allowed-tools:
  - Read
  - Bash
requires:
  - cflow
---

# cflow:verify

## Purpose

Record verification results after implementation.

Verification is the gate before ship.

## Boundary

```text
Implemented changes
  -> LLM/dev records verification JSON
  -> CLI validates
  -> CLI renders .coding/VERIFY.md
```

## Rules

- Use `passed` only when verification is actually complete.
- Use `partial` if some checks were skipped or only partially completed.
- Use `failed` if any blocking issue remains.
- Use `skipped` only when the user explicitly accepts skipped verification.
- Do not ship if verification fails.
- Do not edit markdown directly.

## JSON Output

Produce only valid JSON matching:

```json
{
  "status": "passed",
  "checks": [
    {
      "name": "Unit tests",
      "command": "npm test",
      "status": "passed",
      "notes": ""
    }
  ],
  "manual_checks": [],
  "regressions_checked": [],
  "known_issues": [],
  "done_criteria_verified": []
}
```

Allowed status:

```text
passed
failed
partial
skipped
```

## CLI Command

```bash
cflow verify --input .coding/verify.json
```
