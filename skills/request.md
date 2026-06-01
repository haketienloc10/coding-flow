# Request Skill

1. Do NOT create or write `.coding/request.json`. JSON is not stored.
2. Pipe JSON directly into the CLI via stdin:

```bash
cat <<'JSON' | cflow request --task current
{
  "summary": "Implement feature X",
  "type": "new_feature",
  "planning_needed": true,
  "lane": "normal",
  "next_action": "plan"
}
JSON
```

3. The CLI will validate and render `.coding/tasks/<task-id>/REQUEST.md`. Do not create or edit markdown artifacts yourself.
