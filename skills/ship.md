# Ship Skill

1. Do NOT create or write `.coding/ship.json`. JSON is not stored.
2. Pipe JSON directly into the CLI via stdin:

```bash
cat <<'JSON' | cflow ship --task current --dry-run
{
  "ready": true,
  "verification": { "status": "passed" },
  "commit": { ... }
}
JSON
```

3. The CLI will validate and render `.coding/tasks/<task-id>/SHIP.md`. Do not create or edit markdown artifacts yourself.
4. Only use `--commit` if explicitly requested by the user.
