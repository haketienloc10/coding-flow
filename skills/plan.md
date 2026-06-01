---
name: cflow:plan
description: Create a 9-section code plan from structured JSON.
argument-hint: "[--input <plan.json>]"
allowed-tools:
  - Read
  - Bash
  - AskUserQuestion
requires:
  - cflow
---

# cflow:plan

## Purpose

Create a 9-section code plan after request intake has allowed planning.

The LLM must not edit `.coding/PLAN.md` directly.

## Boundary

```text
Approved request intake
  -> LLM produces plan JSON
  -> CLI validates required planning fields
  -> CLI renders .coding/PLAN.md
```

## Required 9 Sections

The final markdown plan must contain exactly these sections:

1. Objective
2. Scope
3. Requirements
4. Technical Approach
5. Files to Change
6. Implementation Steps
7. Test Plan
8. Risks
9. Done Criteria

## Required Fields At Planning Time

The following must be meaningfully filled:

```text
objective
scope.in
scope.out
requirements
technical_approach
done_criteria.items
```

## Draft Fields

These may be draft and updated during implementation:

```text
files_to_change
implementation_steps
test_plan.planned
risks
```

## Post-Implementation Fields

These are normally completed during verify:

```text
test_plan.result
done_criteria.verified
```

## Rules

- Do not skip request intake.
- Do not edit markdown directly.
- Do not render custom markdown.
- Let `cflow plan` validate and render the plan artifact.
- If required fields are missing, improve JSON before running CLI.

## JSON Output

Produce only valid JSON matching:

```json
{
  "objective": "",
  "scope": {
    "in": [],
    "out": []
  },
  "requirements": [],
  "technical_approach": [],
  "files_to_change": [],
  "implementation_steps": [
    {
      "text": "",
      "status": "todo"
    }
  ],
  "test_plan": {
    "planned": [],
    "result": []
  },
  "risks": [],
  "done_criteria": {
    "items": [],
    "verified": []
  }
}
```

Allowed step status:

```text
todo
in_progress
done
blocked
```

## CLI Command

```bash
cflow plan --input .coding/plan.json
```
