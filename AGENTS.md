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
* Keep main context clean: rely on short `cflow` summaries.
* Do not read long artifacts unless debugging or explicitly needed.
* Do not ship unless `VERIFY.md` exists and verification status is `passed`.
* Use `--dry-run` before `--commit`.
* Commit only when the user explicitly asks to commit.
