# Plan Skill

1. Do NOT create or write `.coding/plan.json`. JSON is not stored.
2. Pipe JSON directly into the CLI via stdin:

```bash
cat <<'JSON' | cflow plan --task current
{
  "objective": "Build feature X",
  "scope": { ... },
  ...
}
JSON
```

3. The CLI will validate and render `.coding/tasks/<task-id>/PLAN.md`. Do not create or edit markdown artifacts yourself.
