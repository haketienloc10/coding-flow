# Verify Skill

1. Do NOT create or write `.coding/verify.json`. JSON is not stored.
2. Pipe JSON directly into the CLI via stdin:

```bash
cat <<'JSON' | cflow verify --task current
{
  "status": "passed",
  "checks": [...],
  ...
}
JSON
```

3. The CLI will validate and render `.coding/tasks/<task-id>/VERIFY.md`. Do not create or edit markdown artifacts yourself.
