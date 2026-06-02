---
name: cflow:agent-coding
description: Implement code from PLAN.md or fix VERIFY.md findings using a oneshot agent and return a coding summary JSON.
argument-hint: "[--task current] [--fix]"
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

Implement the current task or story according to PLAN.md, or fix only findings from VERIFY.md when `--fix` is used.

## Initial Mode

Used by:

```bash
cflow agent coding --task current
cflow story agent coding --story current
```

Rules:

- Read PLAN.md (and STORY.md, PACKET.md in story mode).
- Implement the code changes described in the plan.
- Do not edit REQUEST.md, STORY.md, PACKET.md.
- Do not edit PLAN.md.
- Do not edit CODING.md.
- Do not edit VERIFY.md.
- Do not edit SHIP.md.
- Do not create JSON files.
- Do not run verify.
- Do not run ship.
- Do not commit.
- Return valid JSON only.
- cflow will validate and render CODING.md.
- Verification is handled by the verify phase.
- Return `mode: "initial"`.

## Fix Mode

Used by:

```bash
cflow agent coding --task current --fix
cflow story agent coding --story current --fix
```

Rules:

- Read PLAN.md (and STORY.md, PACKET.md in story mode).
- Read VERIFY.md.
- Read CODING.md if present.
- Fix only unresolved findings from VERIFY.md.
- Do not expand scope.
- Do not rewrite unrelated code.
- Do not re-plan.
- Do not edit REQUEST.md, STORY.md, PACKET.md.
- Do not edit PLAN.md.
- Do not edit CODING.md.
- Do not edit VERIFY.md.
- Do not edit SHIP.md.
- Do not create JSON files.
- Do not verify.
- Do not run ship.
- Do not commit.
- Return valid JSON only.
- cflow will validate and render CODING.md.
- Return `mode: "fix"`.
- Put fixed finding ids into `fixed_findings`.

## Output JSON

Return only JSON matching:

{
  "mode": "fix",
  "status": "ready_for_verify",
  "summary": [],
  "fixed_findings": [],
  "changed_files": [],
  "notes": [],
  "next": "verify"
}

For initial mode, use `"mode": "initial"`.

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
