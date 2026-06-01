# Verify Skill

1. Pipe JSON directly into the CLI via stdin:

```bash
cat <<'JSON' | cflow verify --task current
{
  "status": "passed",
  "checks": [...],
  ...
}
JSON
```

2. The CLI will validate and render `.coding/tasks/<task-id>/VERIFY.md`. Do not create or edit markdown artifacts yourself.
