---
name: cflow:agent-coding
description: Implement code from PLAN.md using a oneshot agent and return a coding summary JSON.
argument-hint: "[--task current]"
allowed-tools:
  - Read
  - Edit
  - MultiEdit
  - Bash
requires:
  - cflow
---

# cflow:agent-coding

## Purpose

Implement the current task according to PLAN.md.

## Rules

- Read PLAN.md.
- Implement the code changes described in the plan.
- Do not edit REQUEST.md.
- Do not edit PLAN.md.
- Do not edit CODING.md.
- Do not edit VERIFY.md.
- Do not edit SHIP.md.
- Do not create JSON files.
- Do not run ship.
- Do not commit.
- Return valid JSON only.
- cflow will validate and render CODING.md.
- Verification is handled by the verify phase.

## Output JSON

Return only JSON matching:

{
  "status": "ready_for_verify",
  "summary": [],
  "changed_files": [],
  "notes": [],
  "next": "verify"
}

Allowed status:

- ready_for_verify
- blocked
- partial
- failed

Allowed next:

- verify
- plan
- clarify
- none
