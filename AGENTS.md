# AGENTS.md

Flow: request -> plan -> implement -> verify -> ship

Always create/select a task first:
cflow new "<task-name>"

Do not create or keep .coding/*.json files.

Use JSON only as stdin input to cflow.

Persistent artifacts are only:
- .coding/tasks/<task-id>/REQUEST.md
- .coding/tasks/<task-id>/PLAN.md
- .coding/tasks/<task-id>/VERIFY.md
- .coding/tasks/<task-id>/SHIP.md

Do not edit these markdown artifacts directly.
Use cflow to render them.

Do not ship unless VERIFY.md exists and verification status is passed.

Use --dry-run before --commit.
Commit only when the user explicitly asks to commit.
