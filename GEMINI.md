# AGENTS.md

## Coding Flow

For coding tasks, use this flow:

```text
request -> plan -> implement -> verify -> ship
```

## Core Rule

Do not edit generated markdown artifacts directly.

The agent writes JSON in `.coding/*.json`. The `cflow` CLI validates that JSON and renders `.coding/*.md`.

```text
agent -> .coding/*.json -> cflow -> .coding/*.md
```

## Commands

### 1. Request

Always classify the request before planning:

```bash
cflow request --input .coding/request.json
```

If `next_action` is not `plan`, do not create a plan.

### 2. Plan

Create a plan only after request intake allows it:

```bash
cflow plan --input .coding/plan.json
```

A valid plan keeps these 9 sections:

1. Objective
2. Scope
3. Requirements
4. Technical Approach
5. Files to Change
6. Implementation Steps
7. Test Plan
8. Risks
9. Done Criteria

Required at planning time:

```text
objective
scope
requirements
technical_approach
done_criteria.items
```

### 3. Implement

Implement only after `.coding/PLAN.md` exists.

Keep changes within the accepted plan. If scope, files, risks, or steps change, update `.coding/plan.json` and rerun `cflow plan`.

### 4. Verify

After implementation, record verification:

```bash
cflow verify --input .coding/verify.json
```

Do not ship unless verification status is `passed`.

### 5. Ship

Run dry-run first:

```bash
cflow ship --input .coding/ship.json --dry-run
```

Commit only when the user explicitly asks:

```bash
cflow ship --input .coding/ship.json --commit
```

## Skill Files

Use phase-specific guidance when needed:

```text
skills/request.md
skills/plan.md
skills/verify.md
skills/ship.md
```
