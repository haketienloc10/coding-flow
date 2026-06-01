---
name: cflow:coding
description: Process manual coding output into CODING.md
argument-hint: "[--task current]"
allowed-tools:
  - Read
requires:
  - cflow
---

# cflow:coding

## Purpose

Process coding JSON and render CODING.md.

## Rules

- Do not create `.coding/*.json` files.
- Pass JSON via stdin or using a transient file with `--input`.
- Keep JSON output transient.
- The artifact will be `.coding/tasks/<task-id>/CODING.md`.

## Expected Output JSON Format

```json
{
  "status": "ready_for_verify",
  "summary": [],
  "changed_files": [],
  "notes": [],
  "next": "verify"
}
```
