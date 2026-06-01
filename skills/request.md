---
name: cflow:request
description: Classify a user request before planning.
argument-hint: "[--auto] [--input <request.md>]"
allowed-tools:
  - Read
  - Bash
  - AskUserQuestion
requires:
  - cflow
---

# cflow:request

## Purpose

Evaluate the user request before creating a plan.

Do not create or edit the code plan during this step.

## Boundary

```text
User request
  -> LLM classifies request
  -> LLM produces request JSON
  -> CLI validates
  -> CLI renders .coding/REQUEST.md
```

LLM does not edit markdown directly.

## Request Types

Choose exactly one:

```text
question
unclear
investigation
new_feature
bug_fix
refactor
maintenance
documentation
test_only
```

## Lanes

Choose exactly one:

```text
none
needs_clarification
tiny
normal
high_risk
```

## Risk Flags

Use these when applicable:

```text
auth
authorization
data_model
security_privacy
external_system
public_contract
cross_platform
existing_behavior_change
weak_proof
multi_domain
```

## Rules

- If the request is a pure question, set `planning_needed=false` and `next_action=answer`.
- If the request is unclear, set `lane=needs_clarification` and include minimal clarification questions.
- If investigation is required before implementation, set `type=investigation` and `next_action=investigate`.
- If code/docs/tests/config changes are needed, set `planning_needed=true` and `next_action=plan`.
- Do not ask the human to classify risk. The system must classify it.
- Do not edit markdown directly.

## JSON Output

Produce only valid JSON matching:

```json
{
  "summary": "",
  "type": "new_feature",
  "planning_needed": true,
  "lane": "normal",
  "risk_flags": [],
  "hard_gates": [],
  "assumptions": [],
  "clarifying_questions": [],
  "next_action": "plan"
}
```

## CLI Command

```bash
cflow request --input .coding/request.json
```
