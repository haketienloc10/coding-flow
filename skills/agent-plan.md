---
name: cflow:agent-plan
description: Generate a code plan JSON from REQUEST.md using a oneshot agent.
argument-hint: "[--task current]"
allowed-tools:
  - Read
  - Bash
requires:
  - cflow
---

# cflow:agent-plan

## Purpose

Create a 9-section code plan from the current task's REQUEST.md.

## Rules

- Read REQUEST.md.
- Do not edit code.
- Do not edit markdown artifacts.
- Do not create JSON files.
- Return valid JSON only.
- cflow will validate and render PLAN.md.

## Required plan sections

1. Objective
2. Scope
3. Requirements
4. Technical Approach
5. Files to Change
6. Implementation Steps
7. Test Plan
8. Risks
9. Done Criteria

## Required fields

The output JSON must meaningfully fill:

- objective
- scope
- requirements
- technical_approach
- done_criteria.items

## Output JSON

Return only JSON matching:

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
