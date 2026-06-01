# Ship Skill

1. Pipe JSON directly into the CLI via stdin:

```bash
cat <<'JSON' | cflow ship --task current --dry-run
{
  "ready": true,
  "verification": { "status": "passed" },
  "commit": { ... }
}
JSON
```

2. The CLI will validate and render `.coding/tasks/<task-id>/SHIP.md`. Do not create or edit markdown artifacts yourself.
3. Only use `--commit` if explicitly requested by the user.
