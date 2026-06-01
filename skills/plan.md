# Plan Skill

1. Pipe JSON directly into the CLI via stdin:

```bash
cat <<'JSON' | cflow plan --task current
{
  "objective": "Build feature X",
  "scope": { ... },
  ...
}
JSON
```

2. The CLI will validate and render `.coding/tasks/<task-id>/PLAN.md`. Do not create or edit markdown artifacts yourself.
