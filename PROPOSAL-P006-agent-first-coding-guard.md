# Proposal: P006 Agent-First Coding Guard

## Status

Rejected. This file is kept as a rejected proposal for audit only. It does not implement any CLI behavior.

## Rejection Rationale

This proposal does not directly solve the core P006 failure mode: an agent can still start manual source edits before invoking `story agent coding`.

The proposed CLI guard mostly enforces behavior at artifact-write time (`story coding`), `next`, `status`, `verify`, and `ship`. Those checks happen after, or adjacent to, the agent's decision to work manually. They can detect and reject invalid manual provenance, but they cannot reliably prevent the main context from spending tokens and editing source files before the required `story agent coding` attempt.

Because the requirement is to prevent unauthorized manual implementation before it begins, not merely reject or audit it later, this approach is insufficient.

Any replacement proposal must enforce the agent-first rule at the point where the agent enters implementation mode, before source edits are allowed.

## Problem

P006 records that a story implementation was performed directly in the main context and then written into `CODING.md` through `cflow story coding`, instead of first running:

```bash
./bin/cflow story agent coding --story current
```

That creates two workflow failures:

- The intended agent subprocess boundary is bypassed.
- The manual path is discovered only after implementation work already happened, so a late reject wastes main-context tokens and effort.

The required behavior is agent-first: manual coding is allowed only when agent coding truly fails and cannot be used for the current story.

## Goals

- Make `story agent coding` the default and enforced coding path for stories.
- Block manual coding before implementation begins when no failed agent attempt exists.
- Allow manual fallback only after a failed agent coding attempt for the same story.
- Record enough provenance to audit why manual fallback was allowed.
- Avoid hardcoding any specific story, packet, provider, schema, or failure case.

## Non-Goals

- Do not remove manual fallback entirely.
- Do not make every coding task a packet-level task.
- Do not create persistent phase JSON files beyond `.coding/state.json`.
- Do not rely on a specific provider failure string.
- Do not require editing `.coding` markdown artifacts by hand.

## Core Rule

For story-level implementation, the main context must not start manual source edits unless the current story has a recorded failed `story agent coding` attempt with fallback explicitly opened.

Allowed coding paths:

1. Normal path:

```bash
./bin/cflow story agent coding --story current
```

2. Fallback path, only after the normal path failed:

```bash
./bin/cflow story coding --story current \
  --manual-fallback \
  --reason "<why agent coding cannot be used>" \
  --related-problem <problem-id>
```

Rejected path:

```bash
./bin/cflow story coding --story current
```

unless the current story already has a failed agent coding attempt and fallback is open.

## State Model

Use `.coding/state.json` as the only persistent JSON state.

Add story-scoped coding attempt metadata under the current packet story entry:

```json
{
  "coding_execution": {
    "source": "none|agent|manual_fallback",
    "fallback_allowed": false,
    "fallback_reason": null,
    "related_problem": null,
    "last_agent_attempt": {
      "status": "not_attempted|failed|succeeded",
      "provider": "codex",
      "command": "cflow story agent coding --story current",
      "attempted_at": "2026-06-02T00:00:00+07:00",
      "error_summary": null
    }
  }
}
```

Field meanings:

- `source`: where the accepted `CODING.md` came from.
- `fallback_allowed`: whether manual fallback may begin.
- `fallback_reason`: required when `source = manual_fallback`.
- `related_problem`: recommended for repeated or workflow-significant failures.
- `last_agent_attempt.status`: controls whether manual coding can be accepted.

The shape is generic and can be reused for `plan` later if needed:

```json
{
  "<artifact>_execution": {
    "source": "...",
    "last_agent_attempt": {}
  }
}
```

## Pre-Implementation Gate

Add a lightweight command or status check that the main context must run before source edits for story coding:

```bash
./bin/cflow story coding guard --story current
```

Expected outputs:

Agent coding required:

```text
Coding guard: agent coding required.
Next: ./bin/cflow story agent coding --story current
Manual fallback: blocked until agent coding fails for this story.
```

Manual fallback allowed:

```text
Coding guard: manual fallback allowed.
Reason: previous story agent coding attempt failed.
Next: ./bin/cflow story coding --story current --manual-fallback --reason "..." --related-problem Pxxx
```

Already coded:

```text
Coding guard: coding already completed.
Source: agent
```

This guard is useful because it blocks intent before implementation. It is not enough to reject at `story coding` after stdin, because by then manual work may already be done.

## Command Behavior

### `story agent coding`

Before launching provider:

- Resolve the current story through the shared story resolver.
- Set `last_agent_attempt.status = not_attempted` or create metadata if missing.
- Ensure manual fallback is closed before a new attempt unless explicitly retrying after failure.

On success:

- Render `CODING.md`.
- Set `coding_execution.source = agent`.
- Set `last_agent_attempt.status = succeeded`.
- Set `fallback_allowed = false`.
- Clear stale `fallback_reason` and `related_problem`.

On failure:

- Do not render `CODING.md`.
- Set `last_agent_attempt.status = failed`.
- Capture provider, command, timestamp, and short `error_summary`.
- Set `fallback_allowed = true`.
- Print the exact allowed fallback command shape.
- Recommend recording a problem if the failure is workflow-significant or likely to repeat.

### `story coding`

This is the manual artifact writer.

It must enforce the guard before reading stdin. That avoids token waste from building JSON that will be rejected.

Reject when:

- story has no failed `last_agent_attempt`
- `fallback_allowed != true`
- `--manual-fallback` is missing
- `--reason` is missing
- `--related-problem` is required by policy and missing

Accepted manual fallback:

- Validate JSON input.
- Render `CODING.md`.
- Set `coding_execution.source = manual_fallback`.
- Store `fallback_reason`.
- Store `related_problem` if provided.
- Set `fallback_allowed = false` after successful manual fallback.

Suggested reject message:

```text
Manual story coding rejected before input.
Run `./bin/cflow story agent coding --story current` first.
Manual fallback is allowed only after a failed agent coding attempt for this story.
```

### `story status`

Show coding provenance:

```text
Coding Source: agent
Agent Coding Attempt: succeeded
Manual Fallback: blocked
```

or:

```text
Coding Source: none
Agent Coding Attempt: failed
Manual Fallback: allowed
```

### `next`

When current scope is a story and `CODING.md` is missing:

```text
Next: ./bin/cflow story agent coding --story current
Reason: current story CODING.md is missing and agent coding has not succeeded
Manual fallback: blocked until agent coding fails
```

After agent coding failed:

```text
Next: manual fallback may be used
Reason: story agent coding failed for current story
Command: ./bin/cflow story coding --story current --manual-fallback --reason "..." --related-problem Pxxx
```

### `verify` and `ship`

These should be secondary enforcement, not the first line of defense.

Reject or hard-warn when:

- `CODING.md` exists but `coding_execution.source` is missing.
- `source = manual_fallback` but `fallback_reason` is missing.
- `source = manual_fallback` but there was no failed agent attempt.

This catches stale or externally edited artifacts, but the primary block remains pre-implementation.

## Main Context Rule

Add this operational rule to workflow guidance:

Before making source edits for a story coding phase, the main context must run the story coding guard or follow `cflow next`.

If the guard says agent coding is required, the main context must run:

```bash
./bin/cflow story agent coding --story current
```

The main context may start manual implementation only after the CLI reports fallback is allowed for the current story.

## Failure Classification

Manual fallback should be allowed for provider or subprocess failures that prevent agent coding from being used, for example:

- provider binary missing
- provider command exits non-zero
- provider API/auth unavailable
- schema rejected by provider
- provider output invalid after retry policy
- subprocess timeout
- context/input exceeds provider limits

Manual fallback should not be allowed merely because:

- the main context prefers to implement directly
- the story seems small
- agent coding might be slower
- a previous unrelated story had an agent failure

The failed attempt must be story-scoped.

## Provider Retry Policy

Before opening manual fallback, `story agent coding` may retry only bounded, generic repairs:

- stricter JSON-only prompt
- schema-compatible prompt adjustment
- provider-specific response format fallback if configured

Do not loop indefinitely. After the bounded retry policy fails, record one failed attempt summary and open fallback.

## Tests

Add tests for command behavior without hardcoding story IDs:

- `story coding` rejects before stdin when no agent attempt exists.
- `story agent coding` success sets `source = agent`.
- `story agent coding` failure sets `fallback_allowed = true`.
- `story coding --manual-fallback` succeeds only after failed attempt.
- fallback permission is scoped to the same packet/story.
- fallback permission is closed after successful manual fallback.
- `story status` displays provenance.
- `next` points to `story agent coding` before fallback is open.
- `verify` or `ship` rejects missing provenance.

## Rollout Plan

1. Add state fields and resolver helpers.
2. Add `story coding guard`.
3. Update `story agent coding` success/failure state writes.
4. Add pre-input guard to `story coding`.
5. Display provenance in `story status` and `next`.
6. Add secondary checks in `verify` or `ship`.
7. Update README/AGENTS guidance.
8. Resolve P006 after tests prove manual fallback cannot happen before a failed story agent coding attempt.

## Expected Result

The workflow becomes agent-first by construction:

- A normal story must use `story agent coding`.
- Manual coding cannot start cleanly until the agent path has failed for that story.
- If manual fallback is used, the reason is durable and auditable.
- The solution applies to all stories and providers without hardcoding one known failure case.
