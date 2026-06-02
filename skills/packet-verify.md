# Packet Verify Skill

Pipe JSON directly into the CLI via stdin:

```bash
cat <<'JSON' | cflow packet verify --packet current
{
  "status": "passed",
  "goal_achieved": true,
  "regressions_checked": true,
  "findings": []
}
JSON
```

The CLI will validate and render `PACKET_VERIFY.md`. Do not create or edit markdown artifacts yourself.
