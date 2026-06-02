# Ship Skill

1. For Tiny Flow, pipe JSON directly into the CLI via stdin:

```bash
cat <<'JSON' | cflow ship --task current --dry-run
{
  "ready": true,
  "verification": { "status": "passed" },
  "commit": { ... }
}
JSON
```

2. For Story Flow, use the story command:

```bash
cat <<'JSON' | cflow story ship --story current --dry-run
{
  "ready": true,
  "verification": { "status": "passed" },
  "commit": { ... }
}
JSON
```

3. The CLI will validate and render `SHIP.md`. Do not create or edit markdown artifacts yourself.
4. Only use `--commit` if explicitly requested by the user.
