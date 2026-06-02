# Agent Instructions

## Flow

Use this flow for coding work:

```text
request -> agent plan -> agent coding -> verify -> ship
```

Manual fallback:

```text
request -> plan -> coding -> verify -> ship
```

## Core Rules

* Always create/select a task first with `bin/cflow new "<task-name>"`.
* Use `.coding/tasks/<task-id>/` for task artifacts.
* Do not create or keep `.coding/*.json` files.
* Use JSON only as transient stdin/stdout input for `cflow`.
* Do not edit artifact markdown directly.
* Let `cflow` render:

  * REQUEST.md
  * PLAN.md
  * CODING.md
  * VERIFY.md
  * SHIP.md
* Prefer `bin/cflow agent plan --task current` for non-trivial planning.
* Prefer `bin/cflow agent coding --task current` for non-trivial implementation.
* For non-trivial implementation, MUST run `bin/cflow agent coding --task current` after `PLAN.md` exists. Main context must not edit code directly unless the agent command fails or the user explicitly requests manual implementation.
* Before any file edit for coding work, check whether `PLAN.md` exists. If it does, run `bin/cflow agent coding --task current` instead of editing in main context. If falling back manually, record the reason in `CODING.md`.
* Agent provider can be selected with `--provider` or `CFLOW_AGENT_PROVIDER`.
* Provider fallback order is `--provider`, `CFLOW_AGENT_PROVIDER`, `.coding/agent.toml` `default_provider`, then `codex`.
* Built-in providers include `codex`, `claude`, `gemini`, and `antigravity`.
* Keep main context clean: rely on short `cflow` summaries.
* Do not read long artifacts unless debugging or explicitly needed.
* Do not ship unless `VERIFY.md` exists and verification status is `passed`.
* Use `--dry-run` before `--commit`.
* Commit only when the user explicitly asks to commit.

## Coding / Verify Loop

If verification fails, do not fix code in the main context.

Use this loop:

```text
CODING.md -> VERIFY.md -> agent coding --fix -> VERIFY.md
```

Rules:

* Verify records findings only.
* Main does not implement fixes.
* Use `cflow agent coding --task current --fix` to fix findings.
* Re-run verify after each fix.
* Ship only when verify is `passed`.
